Another CSS Sprite
=======

[![Build status](https://github.com/Jozefpodlecki/another-css-sprite/workflows/ci/badge.svg)](https://github.com/Jozefpodlecki/another-css-sprite/actions)
![Release](https://img.shields.io/github/v/release/Jozefpodlecki/another-css-sprite?label=latest%20release&color=brightgreen)

A command-line tool for generating optimized CSS sprite sheets from a folder of images.<br/>
Supports horizontal, vertical, and space-efficient packed layouts with automatic CSS class generation.<br/>
Output your sprites as PNG or WebP with minimal config

### 📥 Installation

Grab a prebuilt binary from releases and run it directly.

Or build from source

```sh
git clone https://github.com/Jozefpodlecki/another-css-sprite
cd another-css-sprite
cargo build --release
cd target/release
```

### 🚀 Usage

```sh
anothercssspritecli.exe --input C:\images --layout packed --output test.webp
```

#### Options

- `--input` — Path to your input folder containing images.
- `--layout` — Layout style: `horizontal`, `vertical`, or `packed`.
- `--output` — Output sprite image filename (`.png` or `.webp`).

CSS is generated alongside the sprite image automatically.