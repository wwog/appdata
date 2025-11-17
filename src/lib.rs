#![doc = r#"
<details>
<summary><b>English Documentation</b></summary>

"#]
#![doc = include_str!("../docs/README.md")]
#![doc = r#"

</details>

<details>
<summary><b>中文文档</b></summary>

"#]
#![doc = include_str!("../docs/README_ZH.md")]
#![doc = r#"

</details>
"#]

mod app_data;

pub use app_data::AppData;