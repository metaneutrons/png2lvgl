use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Png2LvglError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Format error: {0}")]
    Format(#[from] FormatError),

    #[error("Configuration error: {0}")]
    Config(String),
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("File not found: {path}")]
    FileNotFound { path: PathBuf },

    #[error("File not readable: {path}")]
    FileNotReadable { path: PathBuf },

    #[error("Invalid PNG header")]
    InvalidPngHeader,

    #[error("Image dimensions {width}x{height} exceed maximum {max_width}x{max_height}")]
    DimensionsTooLarge {
        width: u32,
        height: u32,
        max_width: u32,
        max_height: u32,
    },

    #[error("Image dimensions {width}x{height} below minimum {min_width}x{min_height}")]
    DimensionsTooSmall {
        width: u32,
        height: u32,
        min_width: u32,
        min_height: u32,
    },

    #[error("File size {size} bytes exceeds maximum {max_size} bytes")]
    FileSizeTooLarge { size: u64, max_size: u64 },

    #[error("Output directory not writable: {path}")]
    OutputNotWritable { path: PathBuf },

    #[error("Invalid output filename: {name}")]
    InvalidOutputFilename { name: String },

    #[error("Output file exists: {path}")]
    OutputExists { path: PathBuf },
}

#[derive(Error, Debug)]
pub enum FormatError {
    #[error("Format {format} not implemented")]
    NotImplemented { format: String },

    #[error("Image has {colors} colors, exceeds {max_colors} for {format}")]
    TooManyColors {
        colors: usize,
        max_colors: usize,
        format: String,
    },

    #[error("Invalid bit depth {depth} for format {format}")]
    InvalidBitDepth { depth: u8, format: String },
}

pub type Result<T> = std::result::Result<T, Png2LvglError>;
