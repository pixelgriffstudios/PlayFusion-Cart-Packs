# PlayFusion Cart Packs

Optional, rollback-safe enhancements for cart-focused Kazeta and Kazeta+
systems. These packs do **not** add an internal-games library and never copy
game content to the system drive.

The approved feature set is deliberately limited to:

1. **Profiles Lite** — Default plus three optional profiles with custom names,
   avatars, and isolated saves/game settings.
2. **Loose Media** — one recognized ROM becomes a temporary read-only virtual
   Kazeta cart with a stable ID.
3. **Multi-ROM Browser** — multiple recognized ROMs open a controller-friendly
   play-only browser.

Separate packages are produced for vanilla Kazeta and Kazeta+. The packages
share a small implementation-only core, validate the installed edition before
changing integration files, preserve backups, use atomic writes, and include a
health-check and rollback utility.

See [docs/INSTALL.md](docs/INSTALL.md) and [docs/SAFETY.md](docs/SAFETY.md).

![PlayFusion main menu](https://raw.githubusercontent.com/pixelgriffstudios/PlayFusion/main/assets/playfusion-main-menu.png)

## PC games

PC games are intentionally not treated as loose ROMs. Windows and Linux games
must remain normal Kazeta carts with a `.kzi`, stable `Id`, valid `Exec`, icon,
complete game files, and the appropriate Windows or Linux `.kzr` runtime.

## License

PlayFusion-written code is licensed under the MIT License. Kazeta and Kazeta+
remain separate upstream projects. Emulator runtimes, games, BIOS files, keys,
and copyrighted artwork are not included.
