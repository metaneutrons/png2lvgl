use anyhow::{Context, Result};
use clap::Parser;
use image::{DynamicImage, GenericImageView, Rgba};
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

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

#[derive(Clone, clap::ValueEnum)]
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
    let args = Args::parse();

    if args.stdout && args.output.is_some() {
        anyhow::bail!("Cannot use both --stdout and --output");
    }

    let output = if !args.stdout {
        Some(args.output.unwrap_or_else(|| args.input.with_extension("c")))
    } else {
        None
    };

    if let Some(ref path) = output {
        if path.exists() && !args.overwrite {
            anyhow::bail!("Output file exists. Use --overwrite to replace it.");
        }
    }

    let img = image::open(&args.input)
        .context("Failed to open input image")?;

    let format = match &args.format {
        ColorFormat::Auto => detect_format(&img),
        f => f.clone(),
    };

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
        generate_c(&img, &mut handle, &var_name, &format)?;
    } else {
        let mut file = File::create(output.as_ref().unwrap())?;
        generate_c(&img, &mut file, &var_name, &format)?;
        println!("✓ {}x{} → {} ({})", 
            img.width(), img.height(), 
            output.unwrap().display(),
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

fn generate_c<W: Write>(img: &DynamicImage, writer: &mut W, var_name: &str, format: &ColorFormat) -> Result<()> {
    write_header(writer, var_name)?;
    
    match format {
        ColorFormat::Indexed4 => write_indexed4(img, writer, var_name)?,
        ColorFormat::Indexed8 => write_indexed8(img, writer, var_name)?,
        ColorFormat::TrueColor => write_true_color(img, writer, var_name, false)?,
        ColorFormat::TrueColorAlpha => write_true_color(img, writer, var_name, true)?,
        ColorFormat::Alpha8 => write_alpha8(img, writer, var_name)?,
        _ => anyhow::bail!("Format not yet implemented"),
    }
    
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
    writeln!(writer, "#ifndef LV_ATTRIBUTE_IMG_{}", var_name.to_uppercase())?;
    writeln!(writer, "#define LV_ATTRIBUTE_IMG_{}", var_name.to_uppercase())?;
    writeln!(writer, "#endif\n")?;
    Ok(())
}

fn write_indexed4<W: Write>(img: &DynamicImage, writer: &mut W, var_name: &str) -> Result<()> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    
    writeln!(writer, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
        var_name.to_uppercase(), var_name)?;
    
    // Palette (16 grayscale levels)
    for i in 0..16 {
        let v = (i * 255 / 15) as u8;
        writeln!(writer, "  0x{:02x}, 0x{:02x}, 0x{:02x}, 0xff, \t/*Color of index {}*/", v, v, v, i)?;
    }
    writeln!(writer)?;
    
    // Pixel data (2 pixels per byte)
    let mut data = Vec::new();
    for y in 0..h {
        for x in (0..w).step_by(2) {
            let p1 = gray.get_pixel(x, y)[0] >> 4;
            let p2 = if x + 1 < w { gray.get_pixel(x + 1, y)[0] >> 4 } else { 0 };
            data.push((p1 << 4) | p2);
        }
    }
    
    write_data_array(writer, &data)?;
    writeln!(writer, "}};\n")?;
    
    write_descriptor(writer, var_name, w, h, "LV_IMG_CF_INDEXED_4BIT", data.len())?;
    Ok(())
}

fn write_indexed8<W: Write>(img: &DynamicImage, writer: &mut W, var_name: &str) -> Result<()> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    
    writeln!(writer, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
        var_name.to_uppercase(), var_name)?;
    
    // Palette (256 grayscale levels)
    for i in 0..256 {
        writeln!(writer, "  0x{:02x}, 0x{:02x}, 0x{:02x}, 0xff, \t/*Color of index {}*/", i, i, i, i)?;
    }
    writeln!(writer)?;
    
    let data: Vec<u8> = gray.pixels().map(|p| p[0]).collect();
    write_data_array(writer, &data)?;
    writeln!(writer, "}};\n")?;
    
    write_descriptor(writer, var_name, w, h, "LV_IMG_CF_INDEXED_8BIT", data.len())?;
    Ok(())
}

fn write_true_color<W: Write>(img: &DynamicImage, writer: &mut W, var_name: &str, alpha: bool) -> Result<()> {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    
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
    
    let cf = if alpha { "LV_IMG_CF_TRUE_COLOR_ALPHA" } else { "LV_IMG_CF_TRUE_COLOR" };
    write_descriptor(writer, var_name, w, h, cf, data.len())?;
    Ok(())
}

fn write_alpha8<W: Write>(img: &DynamicImage, writer: &mut W, var_name: &str) -> Result<()> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    
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
        if i > 0 { writeln!(writer)?; }
        write!(writer, "  ")?;
        for (j, byte) in chunk.iter().enumerate() {
            if j > 0 { write!(writer, ", ")?; }
            write!(writer, "0x{:02x}", byte)?;
        }
        write!(writer, ",")?;
    }
    writeln!(writer)?;
    Ok(())
}

fn write_descriptor<W: Write>(writer: &mut W, var_name: &str, w: u32, h: u32, cf: &str, size: usize) -> Result<()> {
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
