# markclip

A Wayland clipboard daemon that automatically enriches plain-text Markdown copies with a `text/html` representation. When you copy Markdown text, apps that accept rich HTML (email clients, Notion, Google Docs, etc.) will receive formatted output instead of raw markup.

## How it works

markclip polls the Wayland clipboard every 500 ms. When it finds plain-text content that looks like Markdown and no `text/html` type is already present, it re-writes the clipboard with both the original plain text and a rendered HTML version.

Markdown detection checks for:
- ATX and Setext headings
- Fenced code blocks (`` ``` `` or `~~~`)
- Blockquotes (`> `)
- Links and images (`](`)
- Thematic breaks / underlines (`---`, `===`, `***`)
- Inline emphasis (`**`, `__`, `~~`, `` ` ``)

Rendering uses [pulldown-cmark](https://github.com/raphlinus/pulldown-cmark) with tables, strikethrough, task lists, and footnotes enabled.

## Requirements

- A running Wayland session (`WAYLAND_DISPLAY` must be set)
- Rust toolchain (for building from source)

## Build

```sh
cargo build --release
```

The binary is written to `target/release/markclip`.

## Install

Copy the binary somewhere on your `PATH`:

```sh
cp target/release/markclip ~/.local/bin/
```

Or install via `cargo`:

```sh
cargo install --path .
```

## Run as a systemd user service

A unit file is included at `markclip.service`. It expects the binary at `~/.cargo/bin/markclip` (the default `cargo install` location).

```sh
# Install the unit
cp markclip.service ~/.config/systemd/user/markclip.service

# Enable and start
systemctl --user enable --now markclip
```

To check status or logs:

```sh
systemctl --user status markclip
journalctl --user -u markclip -f
```

## Building a .deb package

```sh
cargo install cargo-deb
cargo deb
```

The `.deb` installs the binary to `/usr/bin/markclip` and the service unit to `/usr/lib/systemd/user/markclip.service`.

## License

MIT
