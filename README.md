# png2lvgl

[![CI](https://github.com/metaneutrons/png2lvgl/workflows/CI/badge.svg)](https://github.com/metaneutrons/png2lvgl/actions/workflows/ci.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

Convert PNG images to LVGL C arrays with support for multiple color formats.

## Features

- 🎨 Multiple color format support (True Color, Indexed, Alpha)
- 🚀 Fast and efficient Rust implementation
- 📦 Zero runtime dependencies in generated C code
- 🔧 Automatic format detection
- 💾 Safe file handling (no accidental overwrites)

## Installation

### Cargo (Recommended)

```bash
cargo install png2lvgl
```

### Homebrew (macOS)

```bash
# Add the tap
brew tap metaneutrons/tap

# Install png2lvgl
brew install png2lvgl
```

### Pre-built Binaries

Download the latest release for your platform from [GitHub Releases](https://github.com/metaneutrons/png2lvgl/releases).

### From Source

```bash
cargo install --git https://github.com/metaneutrons/png2lvgl
```

## Usage

```bash
# Basic usage (auto-detects format, defaults to LVGL 9.0)
png2lvgl input.png

# Target LVGL 8.x
png2lvgl input.png --lvgl-v8

# Specify output file
png2lvgl input.png -o output.c

# Use 4-bit indexed grayscale (16 colors)
png2lvgl input.png -f indexed4

# Overwrite existing file
png2lvgl input.png --overwrite
```

## Supported Formats

| Format | Description | Use Case |
|--------|-------------|----------|
| `true-color` | RGB565 | Full color images |
| `true-color-alpha` | RGB565 + Alpha | Images with transparency |
| `true-color-chroma` | RGB565 + Chroma key | Transparent color key |
| `indexed1/2/4/8` | Palette (2/4/16/256 colors) | Small images, icons |
| `alpha1/2/4/8` | Alpha only (1/2/4/8 bit) | Masks, monochrome icons |

## Output Format

Generated C files are compatible with LVGL and include:

- Proper header guards
- Memory alignment attributes
- Color palette (for indexed formats)
- Image descriptor structure

Example output:
```c
const lv_img_dsc_t my_image = {
  .header.cf = LV_IMG_CF_INDEXED_4BIT,
  .header.w = 540,
  .header.h = 960,
  .data_size = 259200,
  .data = my_image_map,
};
```

## Building from Source

```bash
git clone https://github.com/metaneturons/png2lvgl
cd png2lvgl
cargo build --release
```

## License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

---

<p align="center">Made with ❤️ in Hannover</p>
