# Enterprise Features

## Error Handling

### Custom Error Types
- `Png2LvglError`: Top-level error type with context
- `ValidationError`: Input/output validation errors
- `FormatError`: Format-specific errors

### Error Recovery
- Automatic cleanup of partial output files on failure
- Graceful degradation for non-critical errors
- Detailed error messages with context

## Structured Logging

### Tracing Integration
Enable detailed logging with `RUST_LOG` environment variable:

```bash
# Info level (default)
RUST_LOG=info png2lvgl input.png

# Debug level (detailed)
RUST_LOG=debug png2lvgl input.png

# Trace level (very detailed)
RUST_LOG=trace png2lvgl input.png
```

### Log Levels
- `error`: Fatal errors
- `warn`: Non-critical issues
- `info`: Progress information
- `debug`: Detailed operation info
- `trace`: Function entry/exit

## Input Validation

### File Validation
- File existence check
- Read permission verification
- PNG header validation
- File size limits (max 100MB)

### Dimension Validation
- Minimum: 1x1 pixels
- Maximum: 8192x8192 pixels
- Prevents memory exhaustion

### Output Validation
- Directory writability check
- Filename validity check
- Overwrite protection

## Usage Examples

### Basic with logging
```bash
RUST_LOG=info png2lvgl input.png -o output.c
```

### Debug mode
```bash
RUST_LOG=debug png2lvgl large_image.png --overwrite
```

### Error handling
```bash
# Will fail with clear error message
png2lvgl nonexistent.png

# Will warn about large dimensions
png2lvgl huge_image.png
```
