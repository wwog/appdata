# app_data

A cross-platform Rust library for managing application data directories on Windows, macOS, and Linux.

## Overview

The `app_data` crate provides a simple and standardized way to manage application data directories across different platforms. It automatically handles platform-specific directory conventions and ensures directories are created when needed.

## Features

- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Automatic directory creation
- ✅ Local and system directory modes
- ✅ Custom error types with detailed messages
- ✅ Zero dependencies (only uses standard library)

## Directory Principles

The AppData directory follows these principles:

- **Default behavior**: Searches for `data` directory under the startup path
- **Fallback**: If the `data` directory doesn't exist under the startup path, creates `data` directory in the system user directory
- **Force local mode**: When `force_local` is enabled, always creates directories in the startup path

## Platform-Specific Directories

### Windows
- **Data**: `%APPDATA%\app_name`

### macOS
- **Data**: `~/Library/Application Support/app_name`

### Linux
- **Data**: `$XDG_DATA_HOME/app_name` or `~/.local/share/app_name`

## Usage

### Basic Usage

```rust
use app_data::AppData;

// Create with default settings
let app_data = AppData::default();

// Or specify application name
let app_data = AppData::new("my_app");

// Get data directory (creates if not exists)
let data_dir = app_data.ensure_data_dir()?;
println!("Data directory: {}", data_dir.display());

// Get file path in data directory
let config_file = app_data.get_file_path("config.json")?;
```

### Force Local Mode

```rust
use app_data::AppData;

// Force creation in startup directory
let app_data = AppData::with_force_local("my_app", true);
let data_dir = app_data.ensure_data_dir()?;
// Creates: ./data/
```

### Error Handling

```rust
use app_data::{AppData, AppDataError};

match app_data.ensure_data_dir() {
    Ok(dir) => println!("Directory: {}", dir.display()),
    Err(AppDataError::EnvVarNotFound(var)) => {
        eprintln!("Environment variable {} not found", var);
    }
    Err(AppDataError::IoError(msg)) => {
        eprintln!("IO error: {}", msg);
    }
    Err(AppDataError::CurrentDirError(msg)) => {
        eprintln!("Failed to get current directory: {}", msg);
    }
}
```

## Examples

See the `examples/` directory for more complete examples.

## License

MIT

