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

### From a .deb package (recommended)

Download the latest `.deb` from the [Releases](../../releases) page and install it:

```sh
sudo apt install ./markclip_*_amd64.deb
```

This places the binary at `/usr/bin/markclip` and the systemd user unit at `/usr/lib/systemd/user/markclip.service`.

Then enable and start the service:

```sh
systemctl --user daemon-reload
systemctl --user enable --now markclip
```

### From source

```sh
cargo build --release
cp target/release/markclip ~/.local/bin/
```

Then install the unit file and start the service:

```sh
cp markclip.service ~/.config/systemd/user/markclip.service
systemctl --user daemon-reload
systemctl --user enable --now markclip
```

### Via cargo install

```sh
cargo install --path .
cp markclip.service ~/.config/systemd/user/markclip.service
systemctl --user daemon-reload
systemctl --user enable --now markclip
```

## Check status / logs

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
