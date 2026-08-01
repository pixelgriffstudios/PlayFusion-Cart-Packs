#!/usr/bin/env bash
set -Eeuo pipefail

repo=$(cd "$(dirname "$0")/.." && pwd)
integrate="$repo/payload/common/usr/lib/playfusion-cart-packs/integrate"
profiles="$repo/payload/profiles/usr/bin/playfusion-profile-apply"
loose="$repo/payload/loose-media/usr/bin/playfusion-loose-media-helper"

for script in "$integrate" "$profiles" "$loose" "$repo/payload/common/usr/lib/playfusion-cart-packs/session-wrapper"; do
    bash -n "$script"
done

tmp=$(mktemp -d)
trap 'rm -rf -- "$tmp"' EXIT
if ! command -v flock >/dev/null 2>&1; then
    mkdir -p "$tmp/test-bin"
    printf '#!/usr/bin/env bash\nexit 0\n' > "$tmp/test-bin/flock"
    chmod +x "$tmp/test-bin/flock"
    PATH="$tmp/test-bin:$PATH"
fi
root="$tmp/root"
mkdir -p "$root/etc/greetd" "$root/usr/bin" "$root/usr/lib/playfusion-cart-packs"
printf '#!/usr/bin/env bash\nexit 0\n' > "$root/usr/bin/kazeta"
printf '#!/usr/bin/env bash\nexit 0\n' > "$root/usr/bin/kazeta-session"
cp "$repo/payload/common/usr/lib/playfusion-cart-packs/session-wrapper" "$root/usr/lib/playfusion-cart-packs/session-wrapper"
chmod +x "$root/usr/bin/kazeta" "$root/usr/bin/kazeta-session" "$root/usr/lib/playfusion-cart-packs/session-wrapper"
printf '[initial_session]\ncommand = "/usr/bin/kazeta-session"\n' > "$root/etc/greetd/config.toml"
cp "$root/etc/greetd/config.toml" "$tmp/original.toml"

PLAYFUSION_TEST_ROOT="$root" "$integrate" install kazeta
grep -qx 'command = "/usr/lib/playfusion-cart-packs/session-wrapper"' "$root/etc/greetd/config.toml"
PLAYFUSION_TEST_ROOT="$root" "$integrate" check kazeta
PLAYFUSION_TEST_ROOT="$root" "$integrate" restore
cmp "$tmp/original.toml" "$root/etc/greetd/config.toml"

mkdir -p "$root/usr/share/kazeta-plus"
if PLAYFUSION_TEST_ROOT="$root" "$integrate" install kazeta >/dev/null 2>&1; then
    echo 'edition mismatch was not rejected' >&2
    exit 1
fi
cmp "$tmp/original.toml" "$root/etc/greetd/config.toml"

if [[ -z "${MSYSTEM:-}" ]]; then
    home="$tmp/gamer"
    state="$tmp/state"
    mkdir -p "$home/.local/share/kazeta/saves/default" "$state"
    printf original > "$home/.local/share/kazeta/saves/default/original.sav"
    printf kazeta > "$state/edition"
    PLAYFUSION_HOME="$home" PLAYFUSION_STATE_ROOT="$state" "$profiles" --apply profile-1
    [[ -L "$home/.local/share/kazeta/saves/default" ]]
    [[ -f "$home/.local/share/playfusion-cart-packs/slots/default/saves/original.sav" ]]
    printf profile1 > "$home/.local/share/kazeta/saves/default/profile-1.sav"
    PLAYFUSION_HOME="$home" PLAYFUSION_STATE_ROOT="$state" "$profiles" --apply default
    [[ -f "$home/.local/share/kazeta/saves/default/original.sav" ]]
    [[ -f "$home/.local/share/playfusion-cart-packs/slots/profile-1/saves/profile-1.sav" ]]
    PLAYFUSION_HOME="$home" PLAYFUSION_STATE_ROOT="$state" "$profiles" --restore
    [[ -d "$home/.local/share/kazeta/saves/default" && ! -L "$home/.local/share/kazeta/saves/default" ]]
    [[ -f "$home/.local/share/kazeta/saves/default/original.sav" ]]
else
    echo 'profile symlink test deferred to native Linux CI'
fi

romroot="$tmp/roms"
mkdir -p "$romroot"
printf '\x4e\x45\x53\x1aTEST' > "$romroot/Test Game.nes"
PLAYFUSION_TEST_ROOTS="$romroot" PLAYFUSION_ROM_LOCK="$tmp/rom.lock" \
    "$loose" probe "$romroot/Test Game.nes" | grep -qx $'nes\tnes-1.0'

echo 'source safety tests passed'
