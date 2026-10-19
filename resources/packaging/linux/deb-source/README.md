# Publishing updates to the PPA

1. Update `debian/changelog` in this directory:
   - For a new Sniffnet release, use its version and ensure the matching Git tag
     is pushed (for example, `1.5.2-1~resolute1` uses `v1.5.2`).
   - For packaging-only changes, increment the revision
     (for example, `1.5.1-1~resolute1` → `1.5.1-1~resolute2`).
   - Keep `resolute` as the target distribution and update the entry’s date.
2. Merge the packaging changes into `main`.
3. Run **Actions → Publish PPA → Run workflow** from `main`, with
   **Sign and publish to ppa:gyulyvgc/sniffnet** checked.
4. Check [Launchpad’s package page](https://launchpad.net/~gyulyvgc/+archive/ubuntu/sniffnet/+packages)
   for acceptance and a successful build, then test the installed update.

The workflow packages the release tag selected by the changelog. It handles
vendoring, source assembly, signing and upload automatically.

Before retrying a failed upload, check whether Launchpad already accepted it.
Published versions cannot be overwritten. For packaging-only updates, keep the
upstream tag and preparation toolchain unchanged so the orig tarball stays identical.
