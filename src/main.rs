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

    let output = args.output.unwrap_or_else(|| {
        args.input.with_extension("c")
    });

    if output.exists() && !args.overwrite {
        anyhow::bail!("Output file exists. Use --overwrite to replace it.");
    }

    let img = image::open(&args.input)
        .context("Failed to open input image")?;

    let format = match &args.format {
        ColorFormat::Auto => detect_format(&img),
        f => f.clone(),
    };

    let var_name = output
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("image")
        .replace('-', "_");

    generate_c_file(&img, &output, &var_name, &format)?;

    println!("✓ {}x{} → {} ({})", 
        img.width(), img.height(), 
        output.display(),
        format_name(&format)
    );

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

fn generate_c_file(img: &DynamicImage, output: &Path, var_name: &str, format: &ColorFormat) -> Result<()> {
    let mut file = File::create(output)?;
    
    write_header(&mut file, var_name)?;
    
    match format {
        ColorFormat::Indexed4 => write_indexed4(img, &mut file, var_name)?,
        ColorFormat::Indexed8 => write_indexed8(img, &mut file, var_name)?,
        ColorFormat::TrueColor => write_true_color(img, &mut file, var_name, false)?,
        ColorFormat::TrueColorAlpha => write_true_color(img, &mut file, var_name, true)?,
        ColorFormat::Alpha8 => write_alpha8(img, &mut file, var_name)?,
        _ => anyhow::bail!("Format not yet implemented"),
    }
    
    Ok(())
}

fn write_header(file: &mut File, var_name: &str) -> Result<()> {
    writeln!(file, "#ifdef __has_include")?;
    writeln!(file, "    #if __has_include(\"lvgl.h\")")?;
    writeln!(file, "        #ifndef LV_LVGL_H_INCLUDE_SIMPLE")?;
    writeln!(file, "            #define LV_LVGL_H_INCLUDE_SIMPLE")?;
    writeln!(file, "        #endif")?;
    writeln!(file, "    #endif")?;
    writeln!(file, "#endif\n")?;
    writeln!(file, "#if defined(LV_LVGL_H_INCLUDE_SIMPLE)")?;
    writeln!(file, "    #include \"lvgl.h\"")?;
    writeln!(file, "#else")?;
    writeln!(file, "    #include \"lvgl/lvgl.h\"")?;
    writeln!(file, "#endif\n")?;
    writeln!(file, "#ifndef LV_ATTRIBUTE_MEM_ALIGN")?;
    writeln!(file, "#define LV_ATTRIBUTE_MEM_ALIGN")?;
    writeln!(file, "#endif\n")?;
    writeln!(file, "#ifndef LV_ATTRIBUTE_IMG_{}", var_name.to_uppercase())?;
    writeln!(file, "#define LV_ATTRIBUTE_IMG_{}", var_name.to_uppercase())?;
    writeln!(file, "#endif\n")?;
    Ok(())
}

fn write_indexed4(img: &DynamicImage, file: &mut File, var_name: &str) -> Result<()> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    
    writeln!(file, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
        var_name.to_uppercase(), var_name)?;
    
    // Palette (16 grayscale levels)
    for i in 0..16 {
        let v = (i * 255 / 15) as u8;
        writeln!(file, "  0x{:02x}, 0x{:02x}, 0x{:02x}, 0xff, \t/*Color of index {}*/", v, v, v, i)?;
    }
    writeln!(file)?;
    
    // Pixel data (2 pixels per byte)
    let mut data = Vec::new();
    for y in 0..h {
        for x in (0..w).step_by(2) {
            let p1 = gray.get_pixel(x, y)[0] >> 4;
            let p2 = if x + 1 < w { gray.get_pixel(x + 1, y)[0] >> 4 } else { 0 };
            data.push((p1 << 4) | p2);
        }
    }
    
    write_data_array(file, &data)?;
    writeln!(file, "}};\n")?;
    
    write_descriptor(file, var_name, w, h, "LV_IMG_CF_INDEXED_4BIT", data.len())?;
    Ok(())
}

fn write_indexed8(img: &DynamicImage, file: &mut File, var_name: &str) -> Result<()> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    
    writeln!(file, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
        var_name.to_uppercase(), var_name)?;
    
    // Palette (256 grayscale levels)
    for i in 0..256 {
        writeln!(file, "  0x{:02x}, 0x{:02x}, 0x{:02x}, 0xff, \t/*Color of index {}*/", i, i, i, i)?;
    }
    writeln!(file)?;
    
    let data: Vec<u8> = gray.pixels().map(|p| p[0]).collect();
    write_data_array(file, &data)?;
    writeln!(file, "}};\n")?;
    
    write_descriptor(file, var_name, w, h, "LV_IMG_CF_INDEXED_8BIT", data.len())?;
    Ok(())
}

fn write_true_color(img: &DynamicImage, file: &mut File, var_name: &str, alpha: bool) -> Result<()> {
    let rgba = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    
    writeln!(file, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
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
    
    write_data_array(file, &data)?;
    writeln!(file, "}};\n")?;
    
    let cf = if alpha { "LV_IMG_CF_TRUE_COLOR_ALPHA" } else { "LV_IMG_CF_TRUE_COLOR" };
    write_descriptor(file, var_name, w, h, cf, data.len())?;
    Ok(())
}

fn write_alpha8(img: &DynamicImage, file: &mut File, var_name: &str) -> Result<()> {
    let gray = img.to_luma8();
    let (w, h) = gray.dimensions();
    
    writeln!(file, "const LV_ATTRIBUTE_MEM_ALIGN LV_ATTRIBUTE_LARGE_CONST LV_ATTRIBUTE_IMG_{} uint8_t {}_map[] = {{", 
        var_name.to_uppercase(), var_name)?;
    
    let data: Vec<u8> = gray.pixels().map(|p| p[0]).collect();
    write_data_array(file, &data)?;
    writeln!(file, "}};\n")?;
    
    write_descriptor(file, var_name, w, h, "LV_IMG_CF_ALPHA_8BIT", data.len())?;
    Ok(())
}

fn write_data_array(file: &mut File, data: &[u8]) -> Result<()> {
    for (i, chunk) in data.chunks(16).enumerate() {
        if i > 0 { writeln!(file)?; }
        write!(file, "  ")?;
        for (j, byte) in chunk.iter().enumerate() {
            if j > 0 { write!(file, ", ")?; }
            write!(file, "0x{:02x}", byte)?;
        }
        write!(file, ",")?;
    }
    writeln!(file)?;
    Ok(())
}

fn write_descriptor(file: &mut File, var_name: &str, w: u32, h: u32, cf: &str, size: usize) -> Result<()> {
    writeln!(file, "const lv_img_dsc_t {} = {{", var_name)?;
    writeln!(file, "  .header.cf = {},", cf)?;
    writeln!(file, "  .header.always_zero = 0,")?;
    writeln!(file, "  .header.reserved = 0,")?;
    writeln!(file, "  .header.w = {},", w)?;
    writeln!(file, "  .header.h = {},", h)?;
    writeln!(file, "  .data_size = {},", size)?;
    writeln!(file, "  .data = {}_map,", var_name)?;
    writeln!(file, "}};")?;
    Ok(())
}
