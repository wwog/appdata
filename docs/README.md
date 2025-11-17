<div></div>

## Overview

Standardize application data directory ownership and management

## Directory Principles

The AppData directory follows these principles:

- Default: searches for `data` directory under the startup path
- If the `data` directory doesn't exist under the startup path, creates `data` directory in the user directory

## Examples

```rust
use app_data::AppData;

let app_data = AppData::default();
```

