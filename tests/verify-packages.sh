#!/usr/bin/env bash
set -Eeuo pipefail

expected=(
  playfusion-cart-core-kazeta
  playfusion-cart-core-kazetaplus
  playfusion-profiles-lite-kazeta
  playfusion-profiles-lite-kazetaplus
  playfusion-loose-media-kazeta
  playfusion-loose-media-kazetaplus
  playfusion-multi-rom-kazeta
  playfusion-multi-rom-kazetaplus
)

package_for() {
    local name=$1
    compgen -G "./${name}-[0-9]*.pkg.tar.zst" | head -n 1
}

for name in "${expected[@]}"; do
    file=$(package_for "$name")
    [[ -n "$file" && -s "$file" ]] || { echo "missing package: $name" >&2; exit 1; }
    bsdtar -tf "$file" | grep -qx '.PKGINFO'
done

for edition in kazeta kazetaplus; do
    core=$(package_for "playfusion-cart-core-$edition")
    bsdtar -tf "$core" | grep -qx 'usr/bin/playfusion-cart-ui'
    bsdtar -tf "$core" | grep -qx 'usr/lib/playfusion-cart-packs/session-wrapper'
    bsdtar -tf "$core" | grep -qx 'usr/lib/playfusion-cart-packs/integrate'
    ! bsdtar -tf "$core" | grep -Eq '(^|/)usr/bin/kazeta$|(^|/)usr/bin/kazeta-session$'

    loose=$(package_for "playfusion-loose-media-$edition")
    bsdtar -tf "$loose" | grep -qx 'usr/bin/playfusion-loose-media-helper'
    bsdtar -tf "$loose" | grep -qx 'usr/lib/systemd/system/playfusion-loose-media.path'

    multi=$(package_for "playfusion-multi-rom-$edition")
    bsdtar -tf "$multi" | grep -qx 'usr/lib/playfusion-cart-packs/features/multi-rom'
done

extract_root=$(mktemp -d)
trap 'rm -rf -- "$extract_root"' EXIT
bsdtar -xf "$(package_for playfusion-cart-core-kazeta)" -C "$extract_root"
test -x "$extract_root/usr/bin/playfusion-cart-ui"
test -x "$extract_root/usr/lib/playfusion-cart-packs/session-wrapper"
test -x "$extract_root/usr/lib/playfusion-cart-packs/integrate"
rm -rf -- "$extract_root"/*
bsdtar -xf "$(package_for playfusion-profiles-lite-kazeta)" -C "$extract_root"
test -x "$extract_root/usr/bin/playfusion-profile-apply"
rm -rf -- "$extract_root"/*
bsdtar -xf "$(package_for playfusion-loose-media-kazeta)" -C "$extract_root"
test -x "$extract_root/usr/bin/playfusion-loose-media-helper"

bash tests/verify-source.sh
echo 'all eight package files passed inspection'
