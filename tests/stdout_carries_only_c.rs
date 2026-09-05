// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2025 Fabian Schmieder

//! `--stdout` must emit the generated C and nothing else.
//!
//! The subscriber used to write to stdout as well, so `png2lvgl in.png
//! --stdout > out.c` produced a file whose first lines were timestamps and
//! ANSI escape sequences. That output does not compile, and nothing in the
//! program noticed.

use std::process::Command;

use image::{DynamicImage, GrayImage};

/// Four distinct grey values per row, so an all-zero result would stand out.
fn write_test_png(path: &std::path::Path) {
    let pixels = [50u8, 100, 150, 200].repeat(4);
    let img = GrayImage::from_raw(4, 4, pixels).expect("4x4 buffer");
    DynamicImage::ImageLuma8(img)
        .save(path)
        .expect("write test png");
}

#[test]
fn stdout_holds_the_generated_c_alone() {
    let dir = tempfile::tempdir().expect("tempdir");
    let input = dir.path().join("input.png");
    write_test_png(&input);

    let out = Command::new(env!("CARGO_BIN_EXE_png2lvgl"))
        .arg(&input)
        .arg("--stdout")
        // An indexed format warns about the colour count, so this exercises a
        // run that actually has something to say on the diagnostic channel.
        .args(["--format", "indexed1"])
        .output()
        .expect("run png2lvgl");

    assert!(out.status.success(), "png2lvgl failed: {out:?}");

    let stdout = String::from_utf8(out.stdout).expect("stdout is UTF-8");
    let stderr = String::from_utf8(out.stderr).expect("stderr is UTF-8");

    assert!(
        stdout.starts_with("/**"),
        "stdout does not begin with the C comment:\n{stdout}"
    );
    assert!(
        !stdout.contains("Loading image"),
        "a diagnostic line reached stdout:\n{stdout}"
    );
    assert!(
        !stdout.contains('\u{1b}'),
        "an escape sequence reached stdout:\n{stdout:?}"
    );
    assert!(
        stderr.contains("Loading image"),
        "the diagnostics did not reach stderr:\n{stderr}"
    );
}
