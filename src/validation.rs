use crate::error::{Result, ValidationError};
use std::fs;
use std::path::Path;
use tracing::{debug, warn};

const MAX_WIDTH: u32 = 8192;
const MAX_HEIGHT: u32 = 8192;
const MIN_WIDTH: u32 = 1;
const MIN_HEIGHT: u32 = 1;
const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100MB

pub fn validate_input_file(path: &Path) -> Result<()> {
    debug!(?path, "Validating input file");

    if !path.exists() {
        return Err(ValidationError::FileNotFound {
            path: path.to_path_buf(),
        }
        .into());
    }

    let metadata = fs::metadata(path).map_err(|_| ValidationError::FileNotReadable {
        path: path.to_path_buf(),
    })?;

    if metadata.len() > MAX_FILE_SIZE {
        return Err(ValidationError::FileSizeTooLarge {
            size: metadata.len(),
            max_size: MAX_FILE_SIZE,
        }
        .into());
    }

    let mut file = fs::File::open(path).map_err(|_| ValidationError::FileNotReadable {
        path: path.to_path_buf(),
    })?;

    let mut header = [0u8; 8];
    use std::io::Read;
    file.read_exact(&mut header)
        .map_err(|_| ValidationError::InvalidPngHeader)?;

    if &header != b"\x89PNG\r\n\x1a\n" {
        return Err(ValidationError::InvalidPngHeader.into());
    }

    debug!("Input file validation passed");
    Ok(())
}

pub fn validate_dimensions(width: u32, height: u32) -> Result<()> {
    debug!(width, height, "Validating dimensions");

    if width < MIN_WIDTH || height < MIN_HEIGHT {
        return Err(ValidationError::DimensionsTooSmall {
            width,
            height,
            min_width: MIN_WIDTH,
            min_height: MIN_HEIGHT,
        }
        .into());
    }

    if width > MAX_WIDTH || height > MAX_HEIGHT {
        warn!(width, height, "Large image dimensions detected");
        return Err(ValidationError::DimensionsTooLarge {
            width,
            height,
            max_width: MAX_WIDTH,
            max_height: MAX_HEIGHT,
        }
        .into());
    }

    Ok(())
}

pub fn validate_output_path(path: &Path, overwrite: bool) -> Result<()> {
    debug!(?path, overwrite, "Validating output path");

    if path.exists() && !overwrite {
        return Err(ValidationError::OutputExists {
            path: path.to_path_buf(),
        }
        .into());
    }

    if let Some(parent) = path.parent() {
        if parent.as_os_str().is_empty() {
            return Ok(());
        }
        if !parent.exists() || fs::metadata(parent).is_err() {
            return Err(ValidationError::OutputNotWritable {
                path: parent.to_path_buf(),
            }
            .into());
        }
    }

    if let Some(name) = path.file_name() {
        let name_str = name.to_string_lossy();
        if name_str.contains('\0') || name_str.trim().is_empty() {
            return Err(ValidationError::InvalidOutputFilename {
                name: name_str.to_string(),
            }
            .into());
        }
    }

    Ok(())
}
