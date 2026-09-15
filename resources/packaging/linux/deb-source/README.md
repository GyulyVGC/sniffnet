# Ubuntu PPA packaging

The manual [Publish PPA](../../../../.github/workflows/ppa.yml) workflow prepares
and uploads a source package to `ppa:gyulyvgc/sniffnet`. Launchpad builds it for
Ubuntu 26.04 (`resolute`), amd64.

## GitHub Actions

Configure the PPA build dependency `ppa:rust-toolchain/staging` and these
repository Actions secrets:

- `PPA_GPG_PRIVATE_KEY`: ASCII-armored private key registered with Launchpad,
  matching `SIGNING_FINGERPRINT` in the workflow.
- `PPA_GPG_PASSPHRASE`: its passphrase, if set.

Once the workflow is on `main`, run **Actions → Publish PPA → Run workflow**
from `main`. Uncheck **Sign and publish** to produce unsigned artifacts only.
Check the [Launchpad package status](https://launchpad.net/~gyulyvgc/+archive/ubuntu/sniffnet/+packages)
after uploading: a successful transfer does not confirm acceptance or a successful build.

## Release inputs

Packaging templates live in this directory’s `debian/` subdirectory. Preparation
copies them to `debian/` at the generated source root, as required by Debian tools.
The paths below refer to these templates or their generated copies.

`debian/changelog` selects the Ubuntu series, package version and upstream tag
(`1.5.1-1~resolute1` uses `v1.5.1`). Preparation extracts that tag, vendors its
locked Cargo dependencies, verifies offline resolution, and adds the current
`debian/` files. The package records the upstream commit and dependency inventory.
The workflow uses Rust 1.97.1 to prepare sources; Launchpad uses the versioned
Rust/Cargo packages specified in `debian/control` and `debian/rules`.

For each upload with changed packaging, increment the Debian revision. For a new
upstream release, update the version and make its tag available. Launchpad does
not allow replacing a published version or its upstream orig tarball; preserve
the tag and preparation toolchain for Debian-only revisions. Check acceptance
before retrying an upload.

## Local preparation and builds

With Python 3.11+, Rust 1.97.1 and the release tag available:

```sh
python3 resources/packaging/linux/deb-source/prepare.py
```

Output goes to `target/debian-source/`. Use `--output PATH` for a fresh output
directory. Preparation needs network access for missing crates and preserves
bundled dependency and asset notices.

On Linux, install `debhelper` and `devscripts`, then add `--build-source` to
assemble an unsigned upload. To build the binary, install the dependencies in
`debian/control` on Ubuntu 26.04 amd64 with `ppa:rust-toolchain/staging` enabled:

```sh
dpkg-source -x sniffnet_1.5.1-1~resolute1.dsc
cd sniffnet-1.5.1
dpkg-buildpackage -b -us -uc
```

Builds and tests use frozen, offline Cargo resolution. Tests requiring Internet
access or raw ICMP sockets are excluded; `DEB_BUILD_OPTIONS=nocheck` skips tests.
