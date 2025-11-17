<div></div>

## 概述

用于规范应用数据目录归属及管理

## 目录原则

AppData 目录遵循以下原则：

- 默认找寻启动路径下的 `data` 目录
- 如果启动路径下的 `data` 目录不存在，则会在用户目录下建立 `data` 目录

## 示例

```rust
use app_data::AppData;

let app_data = AppData::default();
```

