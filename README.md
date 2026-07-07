# FreatePad

A lightweight, cross-platform Markdown editor built with Rust and [egui](https://github.com/emilk/egui).

## Features

- **Split view** with live Markdown preview and draggable divider
- **Syntax highlighting** for code blocks via [syntect](https://github.com/trishume/syntect)
- **Math rendering** (Typst-powered, coming soon)
- **UTF-8 support** with BOM detection (UTF-8, UTF-16 LE/BE)
- **Keyboard shortcuts** (Ctrl+S, Ctrl+Z, Ctrl+F, etc.)
- **Drag & drop** file opening
- **Recent files** history
- **File association** installer (Linux .desktop, Windows registry)
- **Configurable settings** (TOML-based)

## Installation

### From source

```bash
git clone https://github.com/freatevietnam/freatepad
cd freatepad
cargo build --release
```

The binary will be at `target/release/freatepad` (~10 MB with release optimizations).

### Dependencies (Linux)

```bash
# Ubuntu/Debian
sudo apt install libfontconfig1-dev libxcb-render0-dev libxcb-shape0-dev \
  libxcb-xfixes0-dev libxkbcommon-dev libgtk-3-dev

# Fedora
sudo dnf install fontconfig-devel libxcb-devel xcb-util-keysyms-devel \
  libxkbcommon-devel gtk3-devel
```

## Usage

```bash
# Open a file
freatepad README.md

# Open with file dialog
freatepad
```

### Keyboard Shortcuts

| Shortcut | Action |
|---|---|
| Ctrl+N | New file |
| Ctrl+O | Open file |
| Ctrl+S | Save |
| Ctrl+Shift+S | Save as |
| Ctrl+Z | Undo |
| Ctrl+Shift+Z | Redo |
| Ctrl+F | Find |
| Ctrl+H | Replace |
| Ctrl+B | Bold |
| Ctrl+I | Italic |
| Ctrl+1/2/3 | Switch view mode |
| Ctrl+= | Zoom in |
| Ctrl+- | Zoom out |

## License

Licensed under either of:

- [MIT License](LICENSE-MIT)
- [Apache License, Version 2.0](LICENSE-APACHE)

at your option.
