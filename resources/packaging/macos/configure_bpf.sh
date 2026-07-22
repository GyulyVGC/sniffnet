#!/bin/sh
set -eu

BPF_MAJOR=23
BPF_COUNT="$(sysctl -n debug.bpf_maxdevices 2>/dev/null || printf '256')"
STATE_DIR="/var/run/sniffnet-bpf"
LOCK_DIR="/var/run/sniffnet-bpf.lock"
BASELINE_FILE="$STATE_DIR/baseline"
ACTIVE_USER_FILE="$STATE_DIR/active-user"
REF_DIR="$STATE_DIR/refs"
USER_ID="${1:?}"
APP_PID="${2:?}"
REF_FILE="$REF_DIR/$APP_PID"
LOCK_ACQUIRED=0
CREATE_BASELINE=0

is_unsigned_integer() {
    case "$1" in
        '' | *[!0-9]*)
            return 1
            ;;
        *)
            return 0
            ;;
    esac
}

acquire_lock() {
    while ! mkdir "$LOCK_DIR" 2>/dev/null; do
        sleep 1
    done
    LOCK_ACQUIRED=1
}

release_lock() {
    if [ "$LOCK_ACQUIRED" -eq 1 ]; then
        rmdir "$LOCK_DIR"
        LOCK_ACQUIRED=0
    fi
}

minor_from_node() {
    printf '%s\n' "${1#/dev/bpf}"
}

restore_bpf_permissions_locked() {
    [ -f "$BASELINE_FILE" ] || return 0

    while read -r node marker owner group mode; do
        case "$marker" in
            created)
                rm -f "$node" || true
                ;;
            existing)
                if [ ! -e "$node" ]; then
                    mknod "$node" c "$BPF_MAJOR" "$(minor_from_node "$node")" || true
                fi
                if [ -e "$node" ]; then
                    chown "$owner:$group" "$node" || true
                    chmod "$mode" "$node" || true
                fi
                ;;
        esac
    done <"$BASELINE_FILE"

    rm -f "$BASELINE_FILE" "$ACTIVE_USER_FILE"
}

cleanup_stale_refs_locked() {
    for ref in "$REF_DIR"/*; do
        [ -e "$ref" ] || continue
        pid="${ref##*/}"
        if ! is_unsigned_integer "$pid" || ! kill -0 "$pid" 2>/dev/null; then
            rm -f "$ref"
        fi
    done
}

has_refs_locked() {
    for ref in "$REF_DIR"/*; do
        [ -e "$ref" ] && return 0
    done
    return 1
}

restore_if_unused_locked() {
    cleanup_stale_refs_locked
    if ! has_refs_locked; then
        restore_bpf_permissions_locked
    fi
}

configure_bpf_nodes_locked() {
    i=0
    while [ "$i" -lt "$BPF_COUNT" ]; do
        node="/dev/bpf$i"

        if [ "$CREATE_BASELINE" -eq 1 ]; then
            if [ -e "$node" ]; then
                owner="$(stat -f '%u' "$node")"
                group="$(stat -f '%g' "$node")"
                mode="$(stat -f '%Lp' "$node")"
                printf '%s existing %s %s %s\n' "$node" "$owner" "$group" "$mode" >>"$BASELINE_FILE"
            else
                printf '%s created - - -\n' "$node" >>"$BASELINE_FILE"
            fi
        fi

        if [ ! -e "$node" ]; then
            mknod "$node" c "$BPF_MAJOR" "$i"
        fi

        # macOS BPF permissions are device-node based. Keep the grant scoped
        # to this user's Sniffnet session and restore it when the last session exits.
        chown "$USER_ID" "$node"
        chmod u+rw,go-rw "$node"

        i=$((i + 1))
    done
}

cleanup_on_failure() {
    if [ "$LOCK_ACQUIRED" -eq 1 ]; then
        rm -f "$REF_FILE"
        restore_if_unused_locked
        release_lock
    fi
}

is_unsigned_integer "$USER_ID"
is_unsigned_integer "$APP_PID"
is_unsigned_integer "$BPF_COUNT"

umask 077
trap 'cleanup_on_failure' HUP INT TERM EXIT

acquire_lock
mkdir -p "$STATE_DIR" "$REF_DIR"
cleanup_stale_refs_locked

if [ -f "$BASELINE_FILE" ] && ! has_refs_locked; then
    restore_bpf_permissions_locked
fi

if [ -f "$ACTIVE_USER_FILE" ] && [ "$(cat "$ACTIVE_USER_FILE")" != "$USER_ID" ]; then
    printf 'Sniffnet is already configuring BPF devices for another user.\n' >&2
    exit 1
fi

if [ ! -f "$BASELINE_FILE" ]; then
    CREATE_BASELINE=1
    : >"$BASELINE_FILE"
fi
printf '%s\n' "$USER_ID" >"$ACTIVE_USER_FILE"
configure_bpf_nodes_locked
printf '%s\n' "$USER_ID" >"$REF_FILE"
release_lock

(
    while kill -0 "$APP_PID" 2>/dev/null; do
        sleep 2
    done

    acquire_lock
    rm -f "$REF_FILE"
    restore_if_unused_locked
    release_lock
) >/dev/null 2>&1 &

trap - HUP INT TERM EXIT
