# Safety design

The Cart Packs are deliberately smaller in scope than PlayFusion OS.

- The original `/usr/bin/kazeta` and `/usr/bin/kazeta-session` files are never replaced.
- The only boot integration change is one validated command in `/etc/greetd/config.toml`.
- The original greetd file is backed up with a SHA-256 checksum before the atomic change.
- Unsupported or customized greetd commands are refused instead of guessed.
- The wrapper is fail-open: an optional UI or media scan failure continues into the original Kazeta session.
- Removable game sources are never written to, renamed, moved, or deleted. Game paths and media-supplied runtimes are exposed through separate read-only bind mounts.
- Virtual cartridges contain metadata and symlinks only; ROM content is not copied internally.
- Real `.kzi` cartridges take priority and are excluded from loose-ROM scanning.
- Multi-file disc tracks referenced by `.cue` or `.gdi` files are hidden as companion files.
- Stable IDs include platform, normalized filename, size, and samples from the game data to prevent save collisions.
- Multi-ROM collections are not exposed unless Multi-ROM Browser is installed.
- Removal restores the backed-up session configuration and preserves user save data.

Every release workflow performs shell checks, Rust checks, disposable install/restore simulations, package-content inspection, and SHA-256 generation. A release is published only from a tagged commit that passes those checks.
