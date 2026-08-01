# Installing the Cart Packs

These packages are optional additions for existing Kazeta or Kazeta+ systems. They do not install PlayFusion OS, store games internally, or replace the Kazeta launcher.

## Pick the correct edition

- Vanilla Kazeta: use packages ending in `-kazeta`.
- Kazeta+: use packages ending in `-kazetaplus`.

The installer checks the installed edition before changing anything. A mismatched package stops without modifying the session configuration.

## Copy packages to a USB drive

Download the package files and `SHA256SUMS.txt` from the matching GitHub release. Verify them on another Linux computer if possible:

```bash
sha256sum -c SHA256SUMS.txt
```

On the Kazeta console, open a terminal and locate the mounted USB drive:

```bash
find /run/media /media -maxdepth 3 -type f -name 'playfusion-cart-core-*.pkg.tar.zst' 2>/dev/null
```

Change to the directory printed by that command.

## Install one feature

Profiles Lite needs the matching core and profile package:

```bash
sudo pacman -U ./playfusion-cart-core-kazeta-*.pkg.tar.zst ./playfusion-profiles-lite-kazeta-*.pkg.tar.zst
```

Loose Media needs the matching core and loose-media package:

```bash
sudo pacman -U ./playfusion-cart-core-kazeta-*.pkg.tar.zst ./playfusion-loose-media-kazeta-*.pkg.tar.zst
```

Multi-ROM Browser needs core, Loose Media, and Multi-ROM Browser:

```bash
sudo pacman -U ./playfusion-cart-core-kazeta-*.pkg.tar.zst ./playfusion-loose-media-kazeta-*.pkg.tar.zst ./playfusion-multi-rom-kazeta-*.pkg.tar.zst
```

For Kazeta+, replace every `-kazeta-` package name with its `-kazetaplus-` version. Reboot after installation.

## Remove a feature

Remove the optional feature package with `sudo pacman -R PACKAGE_NAME`. Remove the core only after all optional packages are gone. Core removal restores the exact backed-up greetd session configuration.

Profiles Lite removal restores the Default profile as the normal Kazeta save directory. Other profile saves are retained under the user's PlayFusion Cart Packs data folder.

## PC games

Loose Media intentionally does not launch unpackaged PC folders or `.exe` files. PC games must be built as normal Kazeta `.kzi` cartridges with their complete game directory and correct Windows or Linux runtime. See the main [PlayFusion Wiki](https://github.com/pixelgriffstudios/PlayFusion/wiki/Games-and-Removable-Media#pc-games-still-require-a-kazeta-cart).
