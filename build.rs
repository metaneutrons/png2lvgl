use std::env;
use std::path::PathBuf;

fn main() {
    // Generate build info
    built::write_built_file().expect("Failed to acquire build-time information");
    
    // Generate manpage
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let cmd = build_cli();
    
    let man = clap_mangen::Man::new(cmd)
        .section("1")
        .date(chrono::Local::now().format("%Y-%m-%d").to_string())
        .source(format!("png2lvgl {}", env!("CARGO_PKG_VERSION")));
    
    let mut buffer = Vec::new();
    man.render(&mut buffer).expect("Failed to render manpage");
    
    // Add custom sections
    let custom_sections = r#"
.SH EXAMPLES
.TP
Convert a PNG with automatic format detection (LVGL 9.x):
.B png2lvgl icon.png
.PP
.RS
Creates \fBicon.c\fR with automatically detected format using LVGL 9.x constants.
.RE
.TP
Target LVGL 8.x:
.B png2lvgl icon.png \-\-lvgl-v8
.PP
.RS
Generates output compatible with LVGL 8.x using LV_IMG_CF_* constants.
.RE
.TP
Use 4-bit indexed grayscale:
.B png2lvgl logo.png \-f indexed4
.PP
.RS
Converts to 16-color grayscale palette, ideal for small icons.
.RE
.TP
Output to stdout:
.B png2lvgl image.png \-\-stdout > result.c
.TP
Convert with true color and alpha:
.B png2lvgl button.png \-f true-color-alpha \-o ui/button.c
.SH OUTPUT FORMAT
Generated C files include:
.IP \(bu 2
LVGL header includes with proper guards
.IP \(bu 2
Memory alignment attributes (LV_ATTRIBUTE_MEM_ALIGN)
.IP \(bu 2
Color palette data (for indexed formats)
.IP \(bu 2
Pixel data array
.IP \(bu 2
Image descriptor structure (lv_img_dsc_t)
.SH FORMAT SELECTION GUIDE
.TP
.B True Color (RGB565)
Use for full-color images, photos, or complex graphics. 16-bit per pixel.
.TP
.B True Color Alpha
Use when transparency is needed with full color. 24-bit per pixel.
.TP
.B Indexed (1/2/4/8-bit)
Use for icons, logos, or images with limited colors. Saves memory with palette-based encoding.
.TP
.B Alpha Only (1/2/4/8-bit)
Use for masks, monochrome icons, or alpha-only overlays. Most memory-efficient.
.SH LVGL VERSION COMPATIBILITY
.TP
.B LVGL 9.x (default)
Uses new color format constants: LV_COLOR_FORMAT_RGB565, LV_COLOR_FORMAT_I4, LV_COLOR_FORMAT_A8
.TP
.B LVGL 8.x
Uses legacy image format constants: LV_IMG_CF_TRUE_COLOR, LV_IMG_CF_INDEXED_4BIT, LV_IMG_CF_ALPHA_8BIT
.PP
Use \fB\-\-lvgl-v8\fR flag for LVGL 8.x projects, or omit for LVGL 9.x (default).
.SH NOTES
.IP \(bu 2
The tool preserves image dimensions in the output
.IP \(bu 2
Indexed formats automatically convert images to grayscale
.IP \(bu 2
Alpha-only formats extract only the alpha channel
.IP \(bu 2
Output files use proper C naming conventions (underscores for hyphens)
.SH BUGS
Report bugs at: https://github.com/metaneutrons/png2lvgl/issues
.SH AUTHOR
Written by metaneutrons.
.SH COPYRIGHT
Copyright \(co 2025 metaneutrons.
.br
License GPLv3: GNU GPL version 3 <https://gnu.org/licenses/gpl.html>
.br
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
.SH SEE ALSO
.BR lvgl (7),
.BR convert (1),
.BR imagemagick (1)
.PP
LVGL documentation: https://docs.lvgl.io/
.br
Project homepage: https://github.com/metaneutrons/png2lvgl
"#;
    
    let mut full_content = String::from_utf8(buffer).unwrap();
    full_content.push_str(custom_sections);
    
    std::fs::write(out_dir.join("png2lvgl.1"), &full_content)
        .expect("Failed to write manpage");
    
    // Also copy to docs/ for distribution
    std::fs::write("docs/png2lvgl.1", full_content)
        .expect("Failed to write manpage to docs/");
    
    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=build.rs");
}

fn build_cli() -> clap::Command {
    use clap::{Arg, Command};
    
    Command::new("png2lvgl")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Convert PNG images to LVGL C arrays")
        .long_about("png2lvgl converts PNG images to LVGL (Light and Versatile Graphics Library) compatible C arrays. It supports multiple color formats including true color, indexed palettes, and alpha-only modes, making it ideal for embedded systems and resource-constrained environments.")
        .arg(Arg::new("input")
            .help("Input PNG file")
            .required(true)
            .value_name("INPUT"))
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .help("Output C file (defaults to input filename with .c extension)")
            .value_name("OUTPUT"))
        .arg(Arg::new("stdout")
            .long("stdout")
            .help("Write to stdout instead of file")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("format")
            .short('f')
            .long("format")
            .help("Color format")
            .long_help("Specify the color format for conversion. Available formats:\n\
                       auto - Automatically detect optimal format (default)\n\
                       true-color - RGB565 format (16-bit per pixel)\n\
                       true-color-alpha - RGB565 + 8-bit alpha (24-bit per pixel)\n\
                       true-color-chroma - RGB565 with chroma key\n\
                       indexed1/2/4/8 - Palette-based (2/4/16/256 colors)\n\
                       alpha1/2/4/8 - Alpha only (2/4/16/256 levels)")
            .value_name("FORMAT")
            .default_value("auto")
            .value_parser(["auto", "true-color", "true-color-alpha", "true-color-chroma", 
                          "indexed1", "indexed2", "indexed4", "indexed8",
                          "alpha1", "alpha2", "alpha4", "alpha8"]))
        .arg(Arg::new("overwrite")
            .long("overwrite")
            .help("Overwrite existing output file")
            .action(clap::ArgAction::SetTrue))
        .arg(Arg::new("lvgl-v8")
            .long("lvgl-v8")
            .help("Target LVGL 8.x (uses LV_IMG_CF_* constants)")
            .action(clap::ArgAction::SetTrue)
            .conflicts_with("lvgl-v9"))
        .arg(Arg::new("lvgl-v9")
            .long("lvgl-v9")
            .help("Target LVGL 9.x (uses LV_COLOR_FORMAT_* constants, default)")
            .action(clap::ArgAction::SetTrue))
}
