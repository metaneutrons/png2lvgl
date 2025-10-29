use clap::Parser;
use image::{DynamicImage, GenericImageView, Rgba};
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use tracing::{debug, error, info, instrument, warn};

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

mod error;
mod validation;

use error::{FormatError, Png2LvglError, Result};

#[derive(Parser)]
#[command(name = "png2lvgl")]
#[command(version = built_info::GIT_VERSION.unwrap_or(built_info::PKG_VERSION))]
#[command(about = "Convert PNG images to LVGL C arrays", long_about = None)]
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
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum ColorFormat {
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

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let result = run();
    
    if let Err(ref e) = result {
        error!("Fatal error: {}", e);
    }
    
    result
}

#[instrument(skip_all)]
fn run() -> Result<()> {
    let args = Args::parse();

    if args.stdout && args.output.is_some() {
        return Err(Png2LvglError::Config(
            "Cannot use both --stdout and --output".to_string(),
        ));
    }

    if let Err(e) = validation::validate_input_file(&args.input) {
        error!("Input validation failed: {}", e);
        return Err(e);
    }

    let output = if !args.stdout {
        Some(
            args.output
                .unwrap_or_else(|| args.input.with_extension("c")),
        )
    } else {
        None
    };

    if let Some(ref path) = output {
        if let Err(e) = validation::validate_output_path(path, args.overwrite) {
            error!("Output validation failed: {}", e);
            return Err(e);
        }
    }

    info!(?args.input, "Loading image");
    let img = match image::open(&args.input) {
        Ok(img) => img,
        Err(e) => {
            error!("Failed to load image: {}", e);
            return Err(e.into());
        }
    };

    let (w, h) = img.dimensions();
    if let Err(e) = validation::validate_dimensions(w, h) {
        error!("Dimension validation failed: {}", e);
        return Err(e);
    }

    let format = match &args.format {
        ColorFormat::Auto => detect_format(&img),
        f => f.clone(),
    };

    if let Err(e) = validate_format(&img, &format) {
        warn!("Format validation warning: {}", e);
    }

    let var_name = output
        .as_ref()
        .and_then(|p| p.file_stem())
        .or_else(|| args.input.file_stem())
        .and_then(|s| s.to_str())
        .unwrap_or("image")
        .replace('-', "_");

    if args.stdout {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        if let Err(e) = generate_c(&img, &mut handle, &var_name, &format) {
            error!("Failed to generate C code: {}", e);
            return Err(e);
        }
    } else {
        let output_path = output.as_ref().unwrap();
        let mut file = match File::create(output_path) {
            Ok(f) => f,
            Err(e) => {
                error!("Failed to create output file: {}", e);
                return Err(e.into());
            }
        };
        
        if let Err(e) = generate_c(&img, &mut file, &var_name, &format) {
            error!("Failed to generate C code: {}", e);
            let _ = std::fs::remove_file(output_path);
            return Err(e);
        }
        
        info!(
            "✓ {}x{} → {} ({})",
            w,
            h,
            output_path.display(),
            format_name(&format)
        );
    }

    Ok(())
}

fn detect_format(img: &DynamicImage) -> ColorFormat {
    if img.color().has_alpha() {
        ColorFormat::TrueColorAlpha
    } else {
        ColorFormat::TrueColor
    }
}

fn validate_format(img: &DynamicImage, format: &ColorFormat) -> Result<()> {
    debug!(?format, "Validating format compatibility");
    
    match format {
        ColorFormat::Indexed1 | ColorFormat::Indexed2 | ColorFormat::Indexed4 | ColorFormat::Indexed8 => {
            let (max_colors, format_name) = match format {
                ColorFormat::Indexed1 => (2, "Indexed1"),
                ColorFormat::Indexed2 => (4, "Indexed2"),
                ColorFormat::Indexed4 => (16, "Indexed4"),
                ColorFormat::Indexed8 => (256, "Indexed8"),
                _ => unreachable!(),
            };
            
            let unique_colors = count_unique_colors(img);
            debug!(unique_colors, max_colors, "Checking color count");
            
            if unique_colors > max_colors {
                return Err(FormatError::TooManyColors {
                    colors: unique_colors,
                    max_colors,
                    format: format_name.to_string(),
                }
                .into());
            }
        }
        ColorFormat::Alpha1 | ColorFormat::Alpha2 | ColorFormat::Alpha4 | ColorFormat::Alpha8 => {
            let (bit_depth, format_name) = match format {
                ColorFormat::Alpha1 => (1, "Alpha1"),
                ColorFormat::Alpha2 => (2, "Alpha2"),
                ColorFormat::Alpha4 => (4, "Alpha4"),
                ColorFormat::Alpha8 => (8, "Alpha8"),
                _ => unreachable!(),
            };
            
            if img.color().has_color() {
                warn!("Converting color image to alpha-only format");
            }
            
            let img_bits = img.color().bits_per_pixel();
            if bit_depth < 8 && img_bits > bit_depth * 4 {
                return Err(FormatError::InvalidBitDepth {
                    depth: bit_depth as u8,
                    format: format_name.to_string(),
                }
                .into());
            }
        }
        _ => {}
    }
    
    Ok(())
}

fn count_unique_colors(img: &DynamicImage) -> usize {
    use std::collections::HashSet;
    let rgba = img.to_rgba8();
    let mut colors = HashSet::new();
    
    for pixel in rgba.pixels() {
        colors.insert((pixel[0], pixel[1], pixel[2]));
        if colors.len() > 256 {
            return colors.len();
        }
    }
    
    colors.len()
}

fn format_name(format: &ColorFormat) -> &str {
    match format {
        ColorFormat::Auto => "auto",
        ColorFormat::TrueColor => "LV_IMG_CF_TRUE_COLOR",
        ColorFormat::TrueColorAlpha => "LV_IMG_CF_TRUE_COLOR_ALPHA",
        ColorFormat::TrueColorChroma => "LV_IMG_CF_TRUE_COLOR_CHROMA_KEYED",
        ColorFormat::Indexed1 => "LV_IMG_CF_INDEXED_1BIT",
        ColorFormat::Indexed2 => "LV_IMG_CF_INDEXED_2BIT",
        ColorFormat::Indexed4 => "LV_IMG_CF_INDEXED_4BIT",
        ColorFormat::Indexed8 => "LV_IMG_CF_INDEXED_8BIT",
        ColorFormat::Alpha1 => "LV_IMG_CF_ALPHA_1BIT",
        ColorFormat::Alpha2 => "LV_IMG_CF_ALPHA_2BIT",
        ColorFormat::Alpha4 => "LV_IMG_CF_ALPHA_4BIT",
        ColorFormat::Alpha8 => "LV_IMG_CF_ALPHA_8BIT",
    }
}

#[instrument(skip(img, writer))]
fn generate_c<W: Write>(
    img: &DynamicImage,
    writer: &mut W,
    var_name: &str,
    format: &ColorFormat,
) -> Result<()> {
    debug!(?format, var_name, "Generating C code");
    write_header(writer, var_name)?;

    match format {
        ColorFormat::Indexed4 => write_indexed4(img, writer, var_name)?,
        ColorFormat::Indexed8 => write_indexed8(img, writer, var_name)?,
        ColorFormat::TrueColor => write_true_color(img, writer, var_name, false)?,
        ColorFormat::TrueColorAlpha => write_true_color(img, writer, var_name, true)?,
        ColorFormat::Alpha8 => write_alpha8(img, writer, var_name)?,
        f => {
            return Err(FormatError::NotImplemented {
                format: format!("{:?}", f),
            }
            .into())
        }
    }

    debug!("C code generation complete");
    Ok(())
}

fn write_header<W: Write>(writer: &mut W, var_name: &str) -> Result<()> {
    writeln!(writer, "#ifdef __has_include")?;
    writeln!(writer, "    #if __has_include(\"lvgl.h\")")?;
    writeln!(writer, "        #ifndef LV_LVGL_H_INCLUDE_SIMPLE")?;
    writeln!(writer, "            #define LV_LVGL_H_INCLUDE_SIMPLE")?;
    writeln!(writer, "        #endif")?;
    writeln!(writer, "    #endif")?;
    writeln!(writer, "#endif\n")?;
    writeln!(writer, "#if defined(LV_LVGL_H_INCLUDE_SIMPLE)")?;
    writeln!(writer, "    #include \"lvgl.h\"")?;
    writeln!(writer, "#else")?;
    writeln!(writer, "    #include \"lvgl/lvgl.h\"")?;
    writeln!(writer, "#endif\n")?;
    writeln!(writer, "#ifndef LV_ATTRIBUTE_MEM_ALIGN")?;
    writeln!(writer, "#define LV_ATTRIBUTE_MEM_ALIGN")?;
    writeln!(writer, "#endif\n")?;
    writeln!(
        writer,
        "#ifndef LV_ATTRIBUTE_IMG_{}",
        var_name.to_uppercase()
    )?;
    writeln!(
        writer,
        "#define LV_ATTRIBUTE_IMG_{}",
        var_name.to_uppercase()
    )?;
    writeln!(writer, "#endif\n")?;
    Ok(())
}

#[instrument(skip(img, writer))]
fn write_indexed4<W: Write>(img: &DynamicImage, writer: &mut W, var_name: &str) -> Result<()> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    debug!(w, h, "Writing indexed 4-bit data");

    writeln!(writer, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
        var_name.to_uppercase(), var_name)?;

    // Palette (16 grayscale levels)
    for i in 0..16 {
        let v = (i * 255 / 15) as u8;
        writeln!(
            writer,
            "  0x{:02x}, 0x{:02x}, 0x{:02x}, 0xff, \t/*Color of index {}*/",
            v, v, v, i
        )?;
    }
    writeln!(writer)?;

    // Pixel data (2 pixels per byte)
    let mut data = Vec::new();
    for y in 0..h {
        for x in (0..w).step_by(2) {
            let p1 = gray.get_pixel(x, y)[0] >> 4;
            let p2 = if x + 1 < w {
                gray.get_pixel(x + 1, y)[0] >> 4
            } else {
                0
            };
            data.push((p1 << 4) | p2);
        }
    }

    write_data_array(writer, &data)?;
    writeln!(writer, "}};\n")?;

    write_descriptor(writer, var_name, w, h, "LV_IMG_CF_INDEXED_4BIT", data.len())?;
    Ok(())
}

#[instrument(skip(img, writer))]
fn write_indexed8<W: Write>(img: &DynamicImage, writer: &mut W, var_name: &str) -> Result<()> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    debug!(w, h, "Writing indexed 8-bit data");

    writeln!(writer, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
        var_name.to_uppercase(), var_name)?;

    // Palette (256 grayscale levels)
    for i in 0..256 {
        writeln!(
            writer,
            "  0x{:02x}, 0x{:02x}, 0x{:02x}, 0xff, \t/*Color of index {}*/",
            i, i, i, i
        )?;
    }
    writeln!(writer)?;

    let data: Vec<u8> = gray.pixels().map(|p| p[0]).collect();
    write_data_array(writer, &data)?;
    writeln!(writer, "}};\n")?;

    write_descriptor(writer, var_name, w, h, "LV_IMG_CF_INDEXED_8BIT", data.len())?;
    Ok(())
}

#[instrument(skip(img, writer))]
fn write_true_color<W: Write>(
    img: &DynamicImage,
    writer: &mut W,
    var_name: &str,
    alpha: bool,
) -> Result<()> {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    debug!(w, h, alpha, "Writing true color data");

    writeln!(writer, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
        var_name.to_uppercase(), var_name)?;

    let mut data = Vec::new();
    for pixel in rgba.pixels() {
        let Rgba([r, g, b, a]) = *pixel;
        // RGB565 format
        let rgb565 = ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | (b as u16 >> 3);
        data.push((rgb565 & 0xFF) as u8);
        data.push((rgb565 >> 8) as u8);
        if alpha {
            data.push(a);
        }
    }

    write_data_array(writer, &data)?;
    writeln!(writer, "}};\n")?;

    let cf = if alpha {
        "LV_IMG_CF_TRUE_COLOR_ALPHA"
    } else {
        "LV_IMG_CF_TRUE_COLOR"
    };
    write_descriptor(writer, var_name, w, h, cf, data.len())?;
    Ok(())
}

#[instrument(skip(img, writer))]
fn write_alpha8<W: Write>(img: &DynamicImage, writer: &mut W, var_name: &str) -> Result<()> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    debug!(w, h, "Writing alpha 8-bit data");

    writeln!(writer, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
        var_name.to_uppercase(), var_name)?;

    let data: Vec<u8> = gray.pixels().map(|p| p[0]).collect();
    write_data_array(writer, &data)?;
    writeln!(writer, "}};\n")?;

    write_descriptor(writer, var_name, w, h, "LV_IMG_CF_ALPHA_8BIT", data.len())?;
    Ok(())
}

fn write_data_array<W: Write>(writer: &mut W, data: &[u8]) -> Result<()> {
    for (i, chunk) in data.chunks(16).enumerate() {
        if i > 0 {
            writeln!(writer)?;
        }
        write!(writer, "  ")?;
        for (j, byte) in chunk.iter().enumerate() {
            if j > 0 {
                write!(writer, ", ")?;
            }
            write!(writer, "0x{:02x}", byte)?;
        }
        write!(writer, ",")?;
    }
    writeln!(writer)?;
    Ok(())
}

fn write_descriptor<W: Write>(
    writer: &mut W,
    var_name: &str,
    w: u32,
    h: u32,
    cf: &str,
    size: usize,
) -> Result<()> {
    writeln!(writer, "const lv_img_dsc_t {} = {{", var_name)?;
    writeln!(writer, "  .header.cf = {},", cf)?;
    writeln!(writer, "  .header.always_zero = 0,")?;
    writeln!(writer, "  .header.reserved = 0,")?;
    writeln!(writer, "  .header.w = {},", w)?;
    writeln!(writer, "  .header.h = {},", h)?;
    writeln!(writer, "  .data_size = {},", size)?;
    writeln!(writer, "  .data = {}_map,", var_name)?;
    writeln!(writer, "}};")?;
    Ok(())
}
