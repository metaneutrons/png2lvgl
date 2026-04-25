// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2025 Fabian Schmieder

use std::collections::HashSet;

use image::DynamicImage;
use tracing::{debug, warn};

use crate::error::{FormatError, Result};

/// Maximum unique colors before early-exit in `count_unique_colors`.
const MAX_INDEXED_COLORS: usize = 256;

/// Alpha bit-depth threshold multiplier for quality validation.
const ALPHA_DEPTH_THRESHOLD: u16 = 4;

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
pub enum LvglVersion {
    V8,
    V9,
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ColorFormat {
    Auto,
    TrueColor,
    TrueColorAlpha,
    TrueColorChroma,
    Indexed1,
    Indexed2,
    Indexed4,
    Indexed8,
    Alpha1,
    Alpha2,
    Alpha4,
    Alpha8,
}

impl ColorFormat {
    /// Returns the bits-per-pixel for indexed and alpha formats.
    pub const fn bpp(&self) -> Option<u8> {
        match self {
            Self::Indexed1 | Self::Alpha1 => Some(1),
            Self::Indexed2 | Self::Alpha2 => Some(2),
            Self::Indexed4 | Self::Alpha4 => Some(4),
            Self::Indexed8 | Self::Alpha8 => Some(8),
            _ => None,
        }
    }

    /// Returns the LVGL C constant name for this format.
    pub const fn lvgl_const(&self, version: LvglVersion) -> &'static str {
        match version {
            LvglVersion::V8 => match self {
                Self::Auto => "auto",
                Self::TrueColor => "LV_IMG_CF_TRUE_COLOR",
                Self::TrueColorAlpha => "LV_IMG_CF_TRUE_COLOR_ALPHA",
                Self::TrueColorChroma => "LV_IMG_CF_TRUE_COLOR_CHROMA_KEYED",
                Self::Indexed1 => "LV_IMG_CF_INDEXED_1BIT",
                Self::Indexed2 => "LV_IMG_CF_INDEXED_2BIT",
                Self::Indexed4 => "LV_IMG_CF_INDEXED_4BIT",
                Self::Indexed8 => "LV_IMG_CF_INDEXED_8BIT",
                Self::Alpha1 => "LV_IMG_CF_ALPHA_1BIT",
                Self::Alpha2 => "LV_IMG_CF_ALPHA_2BIT",
                Self::Alpha4 => "LV_IMG_CF_ALPHA_4BIT",
                Self::Alpha8 => "LV_IMG_CF_ALPHA_8BIT",
            },
            LvglVersion::V9 => match self {
                Self::Auto => "auto",
                Self::TrueColor => "LV_COLOR_FORMAT_RGB565",
                Self::TrueColorAlpha => "LV_COLOR_FORMAT_RGB565A8",
                Self::TrueColorChroma => "LV_COLOR_FORMAT_RGB565_CHROMA_KEYED",
                Self::Indexed1 => "LV_COLOR_FORMAT_I1",
                Self::Indexed2 => "LV_COLOR_FORMAT_I2",
                Self::Indexed4 => "LV_COLOR_FORMAT_I4",
                Self::Indexed8 => "LV_COLOR_FORMAT_I8",
                Self::Alpha1 => "LV_COLOR_FORMAT_A1",
                Self::Alpha2 => "LV_COLOR_FORMAT_A2",
                Self::Alpha4 => "LV_COLOR_FORMAT_A4",
                Self::Alpha8 => "LV_COLOR_FORMAT_A8",
            },
        }
    }

    /// Human-readable description for the C file header comment.
    pub const fn description(&self) -> &'static str {
        match self {
            Self::Indexed1 => "1-bit indexed (2 colors)",
            Self::Indexed2 => "2-bit indexed (4 colors)",
            Self::Indexed4 => "4-bit indexed (16 colors)",
            Self::Indexed8 => "8-bit indexed (256 colors)",
            Self::Alpha1 => "1-bit alpha (2 levels)",
            Self::Alpha2 => "2-bit alpha (4 levels)",
            Self::Alpha4 => "4-bit alpha (16 levels)",
            Self::Alpha8 => "8-bit alpha (256 levels)",
            Self::TrueColor => "RGB565 true color",
            Self::TrueColorAlpha => "RGB565 true color + alpha",
            Self::TrueColorChroma => "RGB565 true color + chroma key",
            Self::Auto => "Auto-detected",
        }
    }

    /// Whether this format uses RGB565 encoding (affected by endianness).
    pub const fn is_rgb565(&self) -> bool {
        matches!(self, Self::TrueColor | Self::TrueColorAlpha)
    }

    /// Whether this format includes an alpha channel.
    pub const fn has_alpha(&self) -> bool {
        matches!(self, Self::TrueColorAlpha)
    }
}

/// Auto-detect the best color format based on image properties.
pub fn detect(img: &DynamicImage) -> ColorFormat {
    if img.color().has_alpha() {
        ColorFormat::TrueColorAlpha
    } else {
        ColorFormat::TrueColor
    }
}

/// Validate that the image is compatible with the chosen format.
pub fn validate(img: &DynamicImage, format: &ColorFormat) -> Result<()> {
    debug!(?format, "Validating format compatibility");

    match format {
        ColorFormat::Indexed1
        | ColorFormat::Indexed2
        | ColorFormat::Indexed4
        | ColorFormat::Indexed8 => validate_indexed(img, format),
        ColorFormat::Alpha1 | ColorFormat::Alpha2 | ColorFormat::Alpha4 | ColorFormat::Alpha8 => {
            validate_alpha(img, format)
        }
        _ => Ok(()),
    }
}

fn validate_indexed(img: &DynamicImage, format: &ColorFormat) -> Result<()> {
    let bpp = format.bpp().expect("indexed format must have bpp");
    let max_colors = 1_usize << bpp;
    let unique_colors = count_unique_colors(img);

    debug!(unique_colors, max_colors, "Checking color count");

    if unique_colors > max_colors {
        return Err(FormatError::TooManyColors {
            colors: unique_colors,
            max_colors,
            format: format!("{format:?}"),
        }
        .into());
    }
    Ok(())
}

fn validate_alpha(img: &DynamicImage, format: &ColorFormat) -> Result<()> {
    let bpp = format.bpp().expect("alpha format must have bpp");

    if img.color().has_color() {
        warn!("Converting color image to alpha-only format");
    }

    let img_bits = img.color().bits_per_pixel();
    if bpp < 8 && img_bits > u16::from(bpp) * ALPHA_DEPTH_THRESHOLD {
        return Err(FormatError::InvalidBitDepth {
            depth: bpp,
            format: format!("{format:?}"),
        }
        .into());
    }
    Ok(())
}

fn count_unique_colors(img: &DynamicImage) -> usize {
    let rgba = img.to_rgba8();
    let mut colors = HashSet::new();

    for pixel in rgba.pixels() {
        colors.insert((pixel[0], pixel[1], pixel[2]));
        if colors.len() > MAX_INDEXED_COLORS {
            return colors.len();
        }
    }

    colors.len()
}
