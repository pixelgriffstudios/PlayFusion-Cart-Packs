# Maintainer: Jason Griffith / PixelGriff Studios
pkgbase=playfusion-cart-packs
pkgname=(
  playfusion-cart-core-kazeta
  playfusion-cart-core-kazetaplus
  playfusion-profiles-lite-kazeta
  playfusion-profiles-lite-kazetaplus
  playfusion-loose-media-kazeta
  playfusion-loose-media-kazetaplus
  playfusion-multi-rom-kazeta
  playfusion-multi-rom-kazetaplus
)
pkgver=1.0.0
pkgrel=1
arch=(x86_64)
url=https://github.com/pixelgriffstudios/PlayFusion-Cart-Packs
license=(MIT)
options=(!debug)

build() {
  cd "$startdir/ui"
  cargo build --release
}

_install_common() {
  install -Dm755 "$startdir/ui/target/release/playfusion-cart-ui" \
    "$pkgdir/usr/bin/playfusion-cart-ui"
  cp -a "$startdir/payload/common/." "$pkgdir/"
  cp -a "$startdir/payload/assets/." "$pkgdir/"
  chmod 755 "$pkgdir/usr/lib/playfusion-cart-packs/integrate"
  chmod 755 "$pkgdir/usr/lib/playfusion-cart-packs/session-wrapper"
  install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

package_playfusion-cart-core-kazeta() {
  pkgdesc='Safe shared integration for PlayFusion cart packs on vanilla Kazeta'
  depends=(bash coreutils gamescope grep sed util-linux polkit systemd-libs alsa-lib libx11 libxi mesa)
  provides=(playfusion-cart-core)
  conflicts=(playfusion-cart-core-kazetaplus)
  install=packaging/core-kazeta.install
  _install_common
}

package_playfusion-cart-core-kazetaplus() {
  pkgdesc='Safe shared integration for PlayFusion cart packs on Kazeta+'
  depends=(bash coreutils gamescope grep sed util-linux polkit systemd-libs alsa-lib libx11 libxi mesa)
  provides=(playfusion-cart-core)
  conflicts=(playfusion-cart-core-kazeta)
  install=packaging/core-kazetaplus.install
  _install_common
}

_install_profiles() {
  cp -a "$startdir/payload/profiles/." "$pkgdir/"
  chmod 755 "$pkgdir/usr/bin/playfusion-profile-apply"
  install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

package_playfusion-profiles-lite-kazeta() {
  pkgdesc='Four controller-friendly profiles with isolated saves for vanilla Kazeta'
  depends=(playfusion-cart-core-kazeta)
  provides=(playfusion-profiles-lite)
  conflicts=(playfusion-profiles-lite-kazetaplus)
  install=packaging/profiles.install
  _install_profiles
}

package_playfusion-profiles-lite-kazetaplus() {
  pkgdesc='Four controller-friendly profiles with isolated saves for Kazeta+'
  depends=(playfusion-cart-core-kazetaplus)
  provides=(playfusion-profiles-lite)
  conflicts=(playfusion-profiles-lite-kazeta)
  install=packaging/profiles.install
  _install_profiles
}

_install_loose() {
  cp -a "$startdir/payload/loose-media/." "$pkgdir/"
  chmod 755 "$pkgdir/usr/bin/playfusion-loose-media-helper"
  install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

package_playfusion-loose-media-kazeta() {
  pkgdesc='Read-only loose-ROM virtual carts for vanilla Kazeta'
  depends=(playfusion-cart-core-kazeta bash coreutils findutils grep sed gawk util-linux curl unzip)
  provides=(playfusion-loose-media)
  conflicts=(playfusion-loose-media-kazetaplus)
  install=packaging/loose-media.install
  _install_loose
}

package_playfusion-loose-media-kazetaplus() {
  pkgdesc='Read-only loose-ROM virtual carts for Kazeta+'
  depends=(playfusion-cart-core-kazetaplus bash coreutils findutils grep sed gawk util-linux curl unzip)
  provides=(playfusion-loose-media)
  conflicts=(playfusion-loose-media-kazeta)
  install=packaging/loose-media.install
  _install_loose
}

_install_multi() {
  cp -a "$startdir/payload/multi-rom/." "$pkgdir/"
  install -Dm644 "$startdir/LICENSE" "$pkgdir/usr/share/licenses/$pkgname/LICENSE"
}

package_playfusion-multi-rom-kazeta() {
  pkgdesc='Controller-friendly play-only multi-ROM browser for vanilla Kazeta'
  depends=(playfusion-cart-core-kazeta playfusion-loose-media-kazeta)
  provides=(playfusion-multi-rom)
  conflicts=(playfusion-multi-rom-kazetaplus)
  _install_multi
}

package_playfusion-multi-rom-kazetaplus() {
  pkgdesc='Controller-friendly play-only multi-ROM browser for Kazeta+'
  depends=(playfusion-cart-core-kazetaplus playfusion-loose-media-kazetaplus)
  provides=(playfusion-multi-rom)
  conflicts=(playfusion-multi-rom-kazeta)
  _install_multi
}
