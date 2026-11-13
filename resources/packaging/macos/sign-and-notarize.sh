#!/usr/bin/env bash
# Optional Developer ID codesign + notarytool notarization for Sniffnet.app
# and the installer DMG.
#
# If APPLE_CERTIFICATE_P12 is unset or empty, this script prints a short
# message and exits 0, leaving the existing unsigned packaging unchanged.
#
# Expected GitHub Actions secrets (all optional as a group; notarization
# secrets are only required if you also want the DMG stapled):
#
#   APPLE_CERTIFICATE_P12        Base64-encoded Developer ID Application
#                                certificate (.p12).
#   APPLE_CERTIFICATE_PASSWORD   Password for that .p12.
#   APPLE_CODESIGN_IDENTITY      Optional. Defaults to the "Developer ID
#                                Application" identity found in the cert.
#   APPLE_TEAM_ID                Team ID, required for notarization.
#
#   Notary credentials — one of the two sets:
#     APPLE_ID + APPLE_APP_PASSWORD + APPLE_TEAM_ID
#     APPLE_API_KEY + APPLE_API_KEY_ID + APPLE_API_ISSUER
#       APPLE_API_KEY is the AuthKey_*.p8 PEM (raw, or base64-encoded).
#
# Usage:
#   sign-and-notarize.sh sign-app <Sniffnet.app>
#   sign-and-notarize.sh notarize-dmg <Sniffnet_macOS_*.dmg>
#
# The sign-app step, when credentials are present, replaces wrapper.sh as
# CFBundleExecutable with a small signed Mach-O launcher (launcher.c) that
# performs the same osascript admin-privileges prompt. A shell script cannot
# be the main executable of a notarized app.
#
# This script never prints secret values.

set -euo pipefail

usage() {
  echo "Usage: $0 sign-app <Sniffnet.app>" >&2
  echo "       $0 notarize-dmg <Sniffnet_macOS_*.dmg>" >&2
  exit 2
}

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ENTITLEMENTS="$SCRIPT_DIR/entitlements.plist"
LAUNCHER_SRC="$SCRIPT_DIR/launcher.c"
BUNDLE_ID="io.github.gyulyvgc.sniffnet"

if [[ $# -ne 2 ]]; then
  usage
fi

COMMAND="$1"
TARGET="$2"

log() {
  echo "sign-and-notarize: $*"
}

# macOS base64 uses -D; GNU base64 uses --decode. Python is on both.
decode_base64_to() {
  local dest="$1"
  python3 -c 'import sys, base64; sys.stdout.buffer.write(base64.b64decode(sys.stdin.buffer.read()))' >"$dest"
}

# Exit successfully (and leave artefacts unsigned) when the cert is missing.
# This is the no-op path used on pull-request runs and until the maintainer
# has added the secrets.
if [[ -z "${APPLE_CERTIFICATE_P12:-}" ]]; then
  log "APPLE_CERTIFICATE_P12 is not set; skipping codesign and notarization."
  exit 0
fi

if [[ -z "${APPLE_CERTIFICATE_PASSWORD:-}" ]]; then
  echo "sign-and-notarize: APPLE_CERTIFICATE_P12 is set but APPLE_CERTIFICATE_PASSWORD is not." >&2
  exit 1
fi

if [[ ! -f "$ENTITLEMENTS" ]]; then
  echo "sign-and-notarize: entitlements file not found: $ENTITLEMENTS" >&2
  exit 1
fi

CERT_PATH=""
KEYCHAIN_PATH=""
API_KEY_PATH=""
KEYCHAIN_PASSWORD=""

cleanup() {
  if [[ -n "${KEYCHAIN_PATH:-}" && -f "$KEYCHAIN_PATH" ]]; then
    security delete-keychain "$KEYCHAIN_PATH" >/dev/null 2>&1 || true
  fi
  if [[ -n "${CERT_PATH:-}" ]]; then
    rm -f "$CERT_PATH"
  fi
  if [[ -n "${API_KEY_PATH:-}" ]]; then
    rm -f "$API_KEY_PATH"
  fi
}
trap cleanup EXIT

import_certificate() {
  local work
  work="$(mktemp -d)"
  CERT_PATH="$work/developer-id.p12"
  KEYCHAIN_PATH="$work/sniffnet-signing.keychain-db"
  KEYCHAIN_PASSWORD="$(openssl rand -base64 32)"

  # The secret is base64 of a PKCS#12 blob; -D stops openssl from wrapping
  # lines, and we write straight to a file so the bytes never go to stdout.
  if ! printf '%s' "$APPLE_CERTIFICATE_P12" | decode_base64_to "$CERT_PATH"; then
    echo "sign-and-notarize: failed to base64-decode APPLE_CERTIFICATE_P12." >&2
    exit 1
  fi
  if [[ ! -s "$CERT_PATH" ]]; then
    echo "sign-and-notarize: decoded APPLE_CERTIFICATE_P12 is empty." >&2
    exit 1
  fi

  security create-keychain -p "$KEYCHAIN_PASSWORD" "$KEYCHAIN_PATH"
  security set-keychain-settings -lut 21600 "$KEYCHAIN_PATH"
  security unlock-keychain -p "$KEYCHAIN_PASSWORD" "$KEYCHAIN_PATH"
  security import "$CERT_PATH" \
    -k "$KEYCHAIN_PATH" \
    -P "$APPLE_CERTIFICATE_PASSWORD" \
    -T /usr/bin/codesign \
    -T /usr/bin/security \
    -f pkcs12
  security set-key-partition-list \
    -S apple-tool:,apple:,codesign: \
    -s \
    -k "$KEYCHAIN_PASSWORD" \
    "$KEYCHAIN_PATH" >/dev/null
  # Prepend our keychain; do not replace the user's default search list.
  local existing
  existing="$(security list-keychain -d user | sed 's/"//g')"
  # shellcheck disable=SC2086
  security list-keychain -d user -s "$KEYCHAIN_PATH" $existing
}

codesign_identity() {
  if [[ -n "${APPLE_CODESIGN_IDENTITY:-}" ]]; then
    echo "$APPLE_CODESIGN_IDENTITY"
    return
  fi
  local identity
  identity="$(
    security find-identity -v -p codesigning "$KEYCHAIN_PATH" \
      | awk -F '"' '/Developer ID Application/ { print $2; exit }'
  )"
  if [[ -z "$identity" ]]; then
    echo "sign-and-notarize: no Developer ID Application identity found in the imported certificate." >&2
    exit 1
  fi
  echo "$identity"
}

install_macho_launcher() {
  local app="$1"
  local macos_dir="$app/Contents/MacOS"
  local launcher="$macos_dir/SniffnetLauncher"
  local plist="$app/Contents/Info.plist"

  if [[ ! -f "$LAUNCHER_SRC" ]]; then
    echo "sign-and-notarize: launcher source not found: $LAUNCHER_SRC" >&2
    exit 1
  fi
  if [[ ! -x "$macos_dir/sniffnet" ]]; then
    echo "sign-and-notarize: sniffnet binary is missing or not executable in $macos_dir" >&2
    exit 1
  fi

  # macos-latest has clang. Target a conservative minimum so the launcher
  # runs on the same systems as the Rust binary.
  clang -O2 -mmacosx-version-min=11.0 -Wl,-dead_strip \
    -o "$launcher" "$LAUNCHER_SRC"
  chmod +x "$launcher"

  if [[ -f "$plist" ]]; then
    /usr/libexec/PlistBuddy -c "Set :CFBundleExecutable SniffnetLauncher" "$plist"
  fi

  # Leave wrapper.sh out of the signed bundle: an unsigned leftover in
  # Contents/MacOS would fail notarization review.
  rm -f "$macos_dir/wrapper.sh"
}

sign_app() {
  local app="$1"
  if [[ ! -d "$app" ]]; then
    echo "sign-and-notarize: app bundle not found: $app" >&2
    exit 1
  fi

  import_certificate
  local identity
  identity="$(codesign_identity)"
  log "codesigning $app with Developer ID (identity name omitted)."

  install_macho_launcher "$app"

  local macos_dir="$app/Contents/MacOS"
  # Inside-out: nested binaries first, then the bundle. --deep is avoided
  # because Apple recommends signing each executable explicitly.
  codesign --force --options runtime --timestamp \
    --identifier "$BUNDLE_ID.sniffnet" \
    --entitlements "$ENTITLEMENTS" \
    --sign "$identity" \
    --keychain "$KEYCHAIN_PATH" \
    "$macos_dir/sniffnet"

  codesign --force --options runtime --timestamp \
    --identifier "$BUNDLE_ID.launcher" \
    --entitlements "$ENTITLEMENTS" \
    --sign "$identity" \
    --keychain "$KEYCHAIN_PATH" \
    "$macos_dir/SniffnetLauncher"

  codesign --force --options runtime --timestamp \
    --identifier "$BUNDLE_ID" \
    --entitlements "$ENTITLEMENTS" \
    --sign "$identity" \
    --keychain "$KEYCHAIN_PATH" \
    "$app"

  codesign --verify --deep --strict --verbose=2 "$app"
  log "app signature verified."
}

write_api_key() {
  API_KEY_PATH="$(mktemp)"
  chmod 600 "$API_KEY_PATH"
  if [[ "$APPLE_API_KEY" == "-----BEGIN "* ]]; then
    printf '%s\n' "$APPLE_API_KEY" >"$API_KEY_PATH"
  else
    if ! printf '%s' "$APPLE_API_KEY" | decode_base64_to "$API_KEY_PATH"; then
      echo "sign-and-notarize: APPLE_API_KEY is neither a PEM nor valid base64." >&2
      exit 1
    fi
    if [[ ! -s "$API_KEY_PATH" ]]; then
      echo "sign-and-notarize: decoded APPLE_API_KEY is empty." >&2
      exit 1
    fi
  fi
}

notary_credentials_present() {
  if [[ -n "${APPLE_API_KEY:-}" && -n "${APPLE_API_KEY_ID:-}" && -n "${APPLE_API_ISSUER:-}" ]]; then
    return 0
  fi
  if [[ -n "${APPLE_ID:-}" && -n "${APPLE_APP_PASSWORD:-}" && -n "${APPLE_TEAM_ID:-}" ]]; then
    return 0
  fi
  return 1
}

# Submit and wait. Arguments are passed separately so a password
# containing shell metacharacters is not word-split.
submit_for_notarization() {
  local dmg="$1"
  if [[ -n "${APPLE_API_KEY:-}" && -n "${APPLE_API_KEY_ID:-}" && -n "${APPLE_API_ISSUER:-}" ]]; then
    write_api_key
    xcrun notarytool submit "$dmg"       --key "$API_KEY_PATH"       --key-id "$APPLE_API_KEY_ID"       --issuer "$APPLE_API_ISSUER"       --wait
    return
  fi
  xcrun notarytool submit "$dmg"     --apple-id "$APPLE_ID"     --password "$APPLE_APP_PASSWORD"     --team-id "$APPLE_TEAM_ID"     --wait
}

notarize_dmg() {
  local dmg="$1"
  if [[ ! -f "$dmg" ]]; then
    echo "sign-and-notarize: DMG not found: $dmg" >&2
    exit 1
  fi

  import_certificate
  local identity
  identity="$(codesign_identity)"

  # A compressed DMG is signed as a single file (no hardened runtime).
  codesign --force --timestamp \
    --sign "$identity" \
    --keychain "$KEYCHAIN_PATH" \
    "$dmg"
  log "DMG signed."

  if ! notary_credentials_present; then
    log "DMG is signed but notary credentials are missing; skipping notarization and stapling."
    log "Set APPLE_ID + APPLE_APP_PASSWORD + APPLE_TEAM_ID, or APPLE_API_KEY + APPLE_API_KEY_ID + APPLE_API_ISSUER."
    return 0
  fi

  submit_for_notarization "$dmg"
  xcrun stapler staple "$dmg"
  xcrun stapler validate "$dmg"
  log "DMG notarized and stapled."
}

case "$COMMAND" in
  sign-app)
    sign_app "$TARGET"
    ;;
  notarize-dmg)
    notarize_dmg "$TARGET"
    ;;
  *)
    usage
    ;;
esac
