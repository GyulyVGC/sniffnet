#!/usr/bin/env python3
"""Stage a Debian source release with locked, vendored Cargo dependencies."""

import argparse
import gzip
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tarfile
import tempfile
import tomllib


ROOT = Path(__file__).resolve().parents[4]
DEBIAN = Path(__file__).resolve().parent / "debian"


def run(*args, cwd=ROOT, **kwargs):
    return subprocess.run(args, cwd=cwd, check=True, **kwargs)


def prepare(output):
    if output.exists() and any(output.iterdir()):
        raise SystemExit(f"Refusing to overwrite an existing release in {output}; use --output with a fresh directory")
    # Stage outside the checkout so Cargo cannot inherit a newer parent workspace.
    with tempfile.TemporaryDirectory(prefix="sniffnet-debian-") as staging:
        source = prepare_staged(Path(staging))
        output.mkdir(parents=True, exist_ok=True)
        for path in Path(staging).iterdir():
            shutil.move(str(path), output / path.name)
    print(f"Release files: {output}", flush=True)
    return output / source.name


def prepare_staged(output):
    changelog = (DEBIAN / "changelog").read_text()
    match = re.match(r"sniffnet \(([^)]+)\) ([\w-]+);", changelog)
    if not match:
        raise SystemExit("Cannot parse debian/changelog")
    version = match[1].rsplit("-", 1)[0]
    tag = f"refs/tags/v{version}"
    commit = run("git", "rev-parse", "--verify", f"{tag}^{{commit}}",
                 capture_output=True, text=True).stdout.strip()
    manifest = tomllib.loads(run("git", "show", f"{commit}:Cargo.toml",
                                capture_output=True, text=True).stdout)
    if manifest["package"]["version"] != version:
        raise SystemExit("debian/changelog must match the tagged Cargo.toml version")
    source = output / f"sniffnet-{version}"
    archive = output / f"sniffnet_{version}.orig.tar.gz"
    if source.exists() or archive.exists():
        raise SystemExit(f"Refusing to overwrite an existing release in {output}; use --output with a fresh directory")
    epoch = int(run("git", "show", "-s", "--format=%ct", commit,
                    capture_output=True, text=True).stdout)
    output.mkdir(parents=True, exist_ok=True)
    source.mkdir()

    # Extract only release-tag inputs. A branch with the same name cannot win
    # ref resolution, and local application edits cannot enter the package.
    snapshot = run("git", "archive", "--format=tar", commit,
                   capture_output=True).stdout
    run("tar", "-xf", "-", cwd=source, input=snapshot)
    # Overlay current Debian packaging after creating the upstream archive.
    if (source / "debian").exists():
        shutil.rmtree(source / "debian")
    print(f"Upstream tag: {tag}\nUpstream commit: {commit}", flush=True)

    print(f"Vendoring dependencies into {source}", flush=True)
    config = run(
        "cargo", "vendor", "--locked", "--versioned-dirs", "vendor",
        cwd=source, capture_output=True, text=True,
    ).stdout
    (source / ".cargo").mkdir(exist_ok=True)
    (source / ".cargo/config.toml").write_text(config + "\n[net]\noffline = true\n")

    # A fresh Cargo home proves dependency resolution does not use the host cache.
    cargo_home = output / "cargo-home-check"
    cargo_home.mkdir()
    try:
        env = dict(os.environ, CARGO_HOME=str(cargo_home))
        run("cargo", "metadata", "--frozen", "--format-version", "1",
            cwd=source, env=env, stdout=subprocess.DEVNULL)
    finally:
        shutil.rmtree(cargo_home)

    inventory = []
    for crate in sorted((source / "vendor").iterdir()):
        package = tomllib.loads((crate / "Cargo.toml").read_text())["package"]
        license_files = [
            p.relative_to(source).as_posix() for p in sorted(crate.rglob("*"))
            if p.is_file() and p.name.upper().startswith(("LICENSE", "LICENCE", "COPYING", "NOTICE", "COPYRIGHT"))
        ]
        inventory.append({
            "name": package["name"], "version": package["version"],
            "license": package.get("license"), "license_file": package.get("license-file"),
            "rust_version": package.get("rust-version"),
            "repository": package.get("repository"), "license_files": license_files,
        })

    print(f"Writing {archive} ({len(inventory)} vendored crates)", flush=True)
    # Normalize archive metadata so identical inputs produce identical orig tarballs.
    def normalize(info):
        info.uid = info.gid = 0
        info.uname = info.gname = "root"
        info.mtime = epoch
        info.pax_headers = {}
        if info.isfile():
            info.mode = 0o755 if info.mode & 0o111 else 0o644
        elif info.isdir():
            info.mode = 0o755
        return info

    with archive.open("wb") as raw:
        with gzip.GzipFile(filename="", mode="wb", fileobj=raw, mtime=epoch, compresslevel=6) as compressed:
            with tarfile.open(fileobj=compressed, mode="w|", format=tarfile.PAX_FORMAT) as tar:
                tar.add(source, arcname=source.name, filter=normalize)

    packaging_ignore = shutil.ignore_patterns(".DS_Store", "__pycache__", "*.pyc")
    shutil.copytree(DEBIAN, source / "debian", ignore=packaging_ignore)
    (source / "debian/upstream-source.json").write_text(
        json.dumps({"tag": tag, "commit": commit}, indent=2) + "\n")
    (source / "debian/vendor-inventory.json").write_text(json.dumps(inventory, indent=2) + "\n")
    # Install the actual bundled notices alongside the binary, including
    # nested third-party notices (for example in crypto and graphics crates).
    with (source / "debian/vendor-notices.txt").open("w") as notices:
        for crate in inventory:
            notices.write(f"\n{'=' * 78}\n{crate['name']} {crate['version']}\n")
            notices.write(f"Declared license: {crate['license']}\nRepository: {crate['repository']}\n")
            if not crate["license_files"]:
                notices.write("No standalone license text shipped in this crate; see vendor-inventory.json for its declared license.\n")
            for relative in crate["license_files"]:
                notices.write(f"\n--- {relative} ---\n")
                notices.write((source / relative).read_text(errors="replace") + "\n")
    print(f"Prepared {source}\nTarget distribution: {match[2]}", flush=True)
    return source


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, default=ROOT / "target/debian-source")
    parser.add_argument("--build-source", action="store_true",
                        help="also run dpkg-buildpackage to create an unsigned source upload (Linux)")
    args = parser.parse_args()
    if args.build_source and not shutil.which("dpkg-buildpackage"):
        parser.error("--build-source requires dpkg-buildpackage and debhelper")
    source = prepare(args.output.resolve())
    if args.build_source:
        # Build-Depends are needed for the binary build, not source assembly.
        run("dpkg-buildpackage", "-S", "-sa", "-us", "-uc", "-d", cwd=source)


if __name__ == "__main__":
    try:
        main()
    except subprocess.CalledProcessError as error:
        if error.stderr:
            print(error.stderr, file=sys.stderr)
        raise SystemExit(f"Command failed with exit code {error.returncode}: {error.cmd}") from error
