#!/usr/bin/env python3

import argparse
import re
import subprocess
import sys
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]

DEB_PACKAGES = {
    "libasound2",
    "libpcap0.8",
    "libcap2-bin",
    "ca-certificates",
    "libfontconfig1",
    "libdbus-1-3",
    "libxkbcommon0",
    "libxkbcommon-x11-0",
    "libwayland-client0",
    "libx11-6",
    "libx11-xcb1",
    "libxcursor1",
    "libxi6",
}

RPM_PACKAGES = {
    "alsa-lib",
    "libpcap",
    "libcap",
    "ca-certificates",
    "fontconfig",
    "dbus-libs",
    "libxkbcommon",
    "libxkbcommon-x11",
    "libwayland-client",
    "libX11",
    "libX11-xcb",
    "libXcursor",
    "libXi",
}

APPIMAGE_PACKAGES = {
    "libpcap0.8",
    "libdbus-1-3",
    "libxkbcommon0",
    "libxkbcommon-x11-0",
    "libxcursor1",
    "libxi6",
}

DOCKER_PACKAGES = DEB_PACKAGES - {"libcap2-bin"}
PPA_PACKAGES = DEB_PACKAGES - {"libasound2", "libpcap0.8"}

# Libraries present in the binary as ELF dependencies or runtime-loaded names.
# A new name should fail CI until its packaging impact is reviewed.
KNOWN_SHARED_LIBRARIES = {
    "libEGL.so",
    "libEGL.so.1",
    "libX11-xcb.so",
    "libX11-xcb.so.1",
    "libX11.so",
    "libX11.so.6",
    "libXcursor.so",
    "libXcursor.so.1",
    "libXi.so",
    "libXi.so.6",
    "libasound.so.2",
    "libc.so.6",
    "libdbus-1.so",
    "libdbus-1.so.3",
    "libdl.so.2",
    "libgcc_s.so.1",
    "libm.so.6",
    "libpcap.so.0.8",
    "libpcap.so.1",
    "libpthread.so.0",
    "librt.so.1",
    "libwayland-client.so",
    "libwayland-client.so.0",
    "libwayland-egl.so",
    "libwayland-egl.so.1",
    "libxkbcommon-x11.so",
    "libxkbcommon-x11.so.0",
    "libxkbcommon.so",
    "libxkbcommon.so.0",
}

REQUIRED_RUNTIME_LOADED_LIBRARIES = {
    "libX11.so.6",
    "libX11-xcb.so.1",
    "libXcursor.so.1",
    "libXi.so.6",
    "libdbus-1.so.3",
    "libwayland-client.so.0",
    "libxkbcommon-x11.so.0",
    "libxkbcommon.so.0",
}


def fail(message: str) -> None:
    print(f"error: {message}", file=sys.stderr)
    raise SystemExit(1)


def require_packages(actual: set[str], expected: set[str], source: str) -> None:
    missing = sorted(expected - actual)
    if missing:
        fail(f"{source} is missing runtime packages: {', '.join(missing)}")


def package_names(text: str) -> set[str]:
    return set(re.findall(r"(?<![$:{])\b[A-Za-z0-9][A-Za-z0-9+.-]*\b", text))


def audit_manifests() -> None:
    with (ROOT / "Cargo.toml").open("rb") as cargo_file:
        cargo = tomllib.load(cargo_file)

    metadata = cargo["package"]["metadata"]
    require_packages(package_names(metadata["deb"]["depends"]), DEB_PACKAGES, "Cargo DEB metadata")
    require_packages(set(metadata["generate-rpm"]["requires"]), RPM_PACKAGES, "Cargo RPM metadata")

    dockerfile = (ROOT / "Dockerfile").read_text()
    require_packages(package_names(dockerfile), DOCKER_PACKAGES, "Docker runtime stage")

    appimage = (ROOT / "resources/packaging/linux/AppImage/sniffnet.yml").read_text()
    require_packages(package_names(appimage), APPIMAGE_PACKAGES, "AppImage recipe")

    control = (ROOT / "resources/packaging/linux/deb-source/debian/control").read_text()
    binary_stanza = control.split("Package: sniffnet", maxsplit=1)[1]
    require_packages(package_names(binary_stanza), PPA_PACKAGES, "Debian source control")


def audit_binary(path: Path) -> None:
    binary = path.read_bytes()
    libraries = {
        match.decode("ascii")
        for match in re.findall(rb"lib[A-Za-z0-9_+.-]+?\.so(?:\.[0-9]+)*", binary)
    }
    unknown = sorted(libraries - KNOWN_SHARED_LIBRARIES)
    if unknown:
        fail(f"{path} contains unreviewed shared libraries: {', '.join(unknown)}")

    missing = sorted(REQUIRED_RUNTIME_LOADED_LIBRARIES - libraries)
    if missing:
        fail(f"{path} no longer contains expected runtime-loaded libraries: {', '.join(missing)}")


def command_output(command: list[str]) -> str:
    return subprocess.run(command, check=True, text=True, capture_output=True).stdout


def audit_deb(path: Path) -> None:
    dependencies = command_output(["dpkg-deb", "--field", str(path), "Depends"])
    require_packages(package_names(dependencies), DEB_PACKAGES, f"DEB package {path}")


def audit_rpm(path: Path) -> None:
    requirements = command_output(["rpm", "--query", "--package", "--requires", str(path)])
    require_packages(package_names(requirements), RPM_PACKAGES, f"RPM package {path}")


def main() -> None:
    parser = argparse.ArgumentParser(description="Audit Sniffnet's Linux runtime dependencies")
    parser.add_argument("--binary", type=Path)
    parser.add_argument("--deb", type=Path)
    parser.add_argument("--rpm", type=Path)
    args = parser.parse_args()

    audit_manifests()
    if args.binary:
        audit_binary(args.binary)
    if args.deb:
        audit_deb(args.deb)
    if args.rpm:
        audit_rpm(args.rpm)

    print("Linux runtime dependency audit passed")


if __name__ == "__main__":
    main()
