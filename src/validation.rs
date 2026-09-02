// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Fabian Schmieder

use std::fs;
use std::io::Read;
use std::path::Path;

use tracing::{debug, warn};

use crate::error::{Result, ValidationError};

const MAX_WIDTH: u32 = 8192;
const MAX_HEIGHT: u32 = 8192;
const MIN_WIDTH: u32 = 1;
const MIN_HEIGHT: u32 = 1;
/// 100 MiB.
const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;
/// PNG magic bytes.
const PNG_HEADER: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

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

    let mut header = [0u8; PNG_HEADER.len()];
    file.read_exact(&mut header)
        .map_err(|_| ValidationError::InvalidPngHeader)?;

    if &header != PNG_HEADER {
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

    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
        && (!parent.exists() || fs::metadata(parent).is_err())
    {
        return Err(ValidationError::OutputNotWritable {
            path: parent.to_path_buf(),
        }
        .into());
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

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{
        MAX_HEIGHT, MAX_WIDTH, PNG_HEADER, validate_dimensions, validate_input_file,
        validate_output_path,
    };

    /// Smallest thing `validate_input_file` accepts: the magic bytes are all it
    /// reads, decoding happens later.
    fn write_png_stub(dir: &std::path::Path, name: &str) -> std::path::PathBuf {
        let path = dir.join(name);
        fs::write(&path, PNG_HEADER).expect("write stub");
        path
    }

    #[test]
    fn missing_input_is_rejected() {
        let dir = tempfile::tempdir().expect("tempdir");
        let err = validate_input_file(&dir.path().join("absent.png")).unwrap_err();
        assert!(
            err.to_string().to_lowercase().contains("not found"),
            "{err}"
        );
    }

    #[test]
    fn a_file_without_png_magic_is_rejected() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("not-a-png.png");
        fs::write(&path, b"GIF89a  this is not a png at all").expect("write");
        assert!(validate_input_file(&path).is_err());
    }

    /// Truncated shorter than the magic bytes: `read_exact` fails rather than the
    /// comparison, and that path must also be an error.
    #[test]
    fn a_truncated_file_is_rejected() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("short.png");
        fs::write(&path, b"\x89PNG").expect("write");
        assert!(validate_input_file(&path).is_err());
    }

    #[test]
    fn png_magic_is_accepted() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = write_png_stub(dir.path(), "ok.png");
        assert!(validate_input_file(&path).is_ok());
    }

    #[test]
    fn zero_dimensions_are_rejected() {
        assert!(validate_dimensions(0, 1).is_err());
        assert!(validate_dimensions(1, 0).is_err());
        assert!(validate_dimensions(0, 0).is_err());
    }

    #[test]
    fn dimensions_at_the_limit_are_accepted() {
        assert!(validate_dimensions(1, 1).is_ok());
        assert!(validate_dimensions(MAX_WIDTH, MAX_HEIGHT).is_ok());
    }

    #[test]
    fn dimensions_past_the_limit_are_rejected() {
        assert!(validate_dimensions(MAX_WIDTH + 1, MAX_HEIGHT).is_err());
        assert!(validate_dimensions(MAX_WIDTH, MAX_HEIGHT + 1).is_err());
    }

    #[test]
    fn an_existing_output_needs_the_overwrite_flag() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("out.c");
        fs::write(&path, b"previous contents").expect("write");

        assert!(validate_output_path(&path, false).is_err());
        assert!(validate_output_path(&path, true).is_ok());
    }

    #[test]
    fn a_new_output_in_an_existing_directory_is_accepted() {
        let dir = tempfile::tempdir().expect("tempdir");
        assert!(validate_output_path(&dir.path().join("fresh.c"), false).is_ok());
    }

    #[test]
    fn an_output_in_a_missing_directory_is_rejected() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().join("no-such-dir").join("out.c");
        assert!(validate_output_path(&path, false).is_err());
    }

    /// A bare filename has an empty parent. That must not be mistaken for a
    /// missing directory.
    #[test]
    fn a_bare_filename_is_accepted() {
        assert!(validate_output_path(std::path::Path::new("out.c"), true).is_ok());
    }
}
