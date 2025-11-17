# app_data

一个跨平台的 Rust 库，用于管理 Windows、macOS 和 Linux 上的应用数据目录。

## 概述

`app_data` 库提供了一种简单且标准化的方式来管理不同平台上的应用数据目录。它会自动处理平台特定的目录约定，并在需要时确保目录被创建。

## 特性

- ✅ 跨平台支持（Windows、macOS、Linux）
- ✅ 自动创建目录
- ✅ 本地和系统目录模式
- ✅ 自定义错误类型，提供详细错误信息
- ✅ 零依赖（仅使用标准库）

## 目录原则

AppData 目录遵循以下原则：

- **默认行为**：在启动路径下搜索 `data` 目录
- **回退机制**：如果启动路径下不存在 `data` 目录，则在系统用户目录下创建 `data` 目录
- **强制本地模式**：启用 `force_local` 时，始终在启动路径下创建目录

## 平台特定目录

### Windows
- **数据目录**：`%APPDATA%\app_name`

### macOS
- **数据目录**：`~/Library/Application Support/app_name`

### Linux
- **数据目录**：`$XDG_DATA_HOME/app_name` 或 `~/.local/share/app_name`

## 使用方法

### 基本使用

```rust
use app_data::AppData;

// 使用默认设置创建
let app_data = AppData::default();

// 或指定应用名称
let app_data = AppData::new("my_app");

// 获取数据目录（如果不存在则创建）
let data_dir = app_data.ensure_data_dir()?;
println!("数据目录: {}", data_dir.display());

// 获取数据目录中的文件路径
let config_file = app_data.get_file_path("config.json")?;
```

### 强制本地模式

```rust
use app_data::AppData;

// 强制在启动目录下创建
let app_data = AppData::with_force_local("my_app", true);
let data_dir = app_data.ensure_data_dir()?;
// 创建：./data/
```

### 错误处理

```rust
use app_data::{AppData, AppDataError};

match app_data.ensure_data_dir() {
    Ok(dir) => println!("目录: {}", dir.display()),
    Err(AppDataError::EnvVarNotFound(var)) => {
        eprintln!("环境变量 {} 未找到", var);
    }
    Err(AppDataError::IoError(msg)) => {
        eprintln!("IO 错误: {}", msg);
    }
    Err(AppDataError::CurrentDirError(msg)) => {
        eprintln!("获取当前目录失败: {}", msg);
    }
}
```

## 示例

查看 `examples/` 目录获取更多完整示例。

## 许可证

MIT

