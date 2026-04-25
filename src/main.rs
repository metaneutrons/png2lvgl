// SPDX-License-Identifier: GPL-3.0-only
// Copyright (C) 2025 Fabian Schmieder

use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use image::GenericImageView;
use tracing::{error, info, instrument, warn};

mod codegen;
mod error;
mod format;
mod validation;

use codegen::GenerateParams;
use error::{Png2LvglError, Result};
use format::{ColorFormat, LvglVersion};

#[derive(Parser)]
#[command(name = "png2lvgl")]
#[command(version = codegen::built_info::GIT_VERSION.unwrap_or(codegen::built_info::PKG_VERSION))]
#[command(about = "Convert PNG images to LVGL C arrays", long_about = None)]
#[allow(clippy::struct_excessive_bools)]
struct Args {
    /// Input PNG file
    input: PathBuf,

    /// Output C file (defaults to input filename with .c extension)
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Write to stdout instead of file
    #[arg(long)]
    stdout: bool,

    /// Color format
    #[arg(short = 'f', long, value_enum, default_value = "auto")]
    format: ColorFormat,

    /// Overwrite existing output file
    #[arg(long)]
    overwrite: bool,

    /// Target LVGL 8.x (uses `LV_IMG_CF_*` constants)
    #[arg(long, conflicts_with = "lvgl_v9", group = "lvgl_version")]
    lvgl_v8: bool,

    /// Target LVGL 9.x (uses `LV_COLOR_FORMAT_*` constants, default)
    #[arg(long, group = "lvgl_version")]
    lvgl_v9: bool,

    /// Generate big-endian RGB565 (for big-endian systems)
    #[arg(long)]
    big_endian: bool,
}

impl Args {
    const fn lvgl_version(&self) -> LvglVersion {
        if self.lvgl_v8 {
            LvglVersion::V8
        } else {
            LvglVersion::V9
        }
    }
}

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let result = run();
    if let Err(ref e) = result {
        error!("Fatal error: {e}");
    }
    result
}

#[instrument(skip_all)]
fn run() -> Result<()> {
    let args = Args::parse();
    let lvgl_version = args.lvgl_version();

    if args.stdout && args.output.is_some() {
        return Err(Png2LvglError::Config(
            "Cannot use both --stdout and --output".to_string(),
        ));
    }

    validation::validate_input_file(&args.input)?;

    let output = if args.stdout {
        None
    } else {
        Some(
            args.output
                .unwrap_or_else(|| args.input.with_extension("c")),
        )
    };

    if let Some(ref path) = output {
        validation::validate_output_path(path, args.overwrite)?;
    }

    info!(?args.input, "Loading image");
    let img = image::open(&args.input)?;

    let (w, h) = img.dimensions();
    validation::validate_dimensions(w, h)?;

    let fmt = match &args.format {
        ColorFormat::Auto => format::detect(&img),
        f => f.clone(),
    };

    if let Err(e) = format::validate(&img, &fmt) {
        warn!("Format validation warning: {e}");
    }

    if args.big_endian && !fmt.is_rgb565() {
        warn!("--big-endian flag ignored: only applies to RGB565 formats");
    }

    let var_name = output
        .as_ref()
        .and_then(|p| p.file_stem())
        .or_else(|| args.input.file_stem())
        .and_then(|s| s.to_str())
        .unwrap_or("image")
        .replace('-', "_");

    let source_filename = args
        .input
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown.png");

    let params = GenerateParams {
        var_name: &var_name,
        format: &fmt,
        lvgl_version,
        big_endian: args.big_endian,
        source_file: source_filename,
        output_file: output
            .as_ref()
            .and_then(|p| p.file_name())
            .and_then(|s| s.to_str())
            .unwrap_or("stdout"),
    };

    if args.stdout {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        codegen::generate(&img, &mut handle, &params)?;
    } else {
        let output_path = output.as_ref().expect("output path must be set");
        let mut file = File::create(output_path)?;

        if let Err(e) = codegen::generate(&img, &mut file, &params) {
            let _ = std::fs::remove_file(output_path);
            return Err(e);
        }

        info!(
            "✓ {w}x{h} → {} ({})",
            output_path.display(),
            fmt.lvgl_const(lvgl_version)
        );
    }

    Ok(())
}
