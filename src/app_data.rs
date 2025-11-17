use std::{
    env::{self, current_dir, var},
    fs,
    path::PathBuf,
};

#[cfg(target_os = "windows")]
pub fn get_sys_app_data_dir() -> std::io::Result<PathBuf> {
    if let Ok(base) = var("APPDATA") {
        Ok(PathBuf::from(base))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "APPDATA not found",
        ))
    }
}

#[cfg(target_os = "macos")]
pub fn get_sys_app_data_dir() -> std::io::Result<PathBuf> {
    if let Ok(base) = var("HOME") {
        Ok(PathBuf::from(base).join("Library/Application Support"))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "HOME not found",
        ))
    }
}

#[cfg(target_os = "linux")]
pub fn get_sys_app_data_dir() -> std::io::Result<PathBuf> {
    if let Ok(xdg) = var("XDG_DATA_HOME") {
        Ok(PathBuf::from(xdg))
    } else if let Ok(home) = var("HOME") {
        Ok(PathBuf::from(home).join(".local/share"))
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "XDG_DATA_HOME and HOME not found",
        ))
    }
}

pub struct AppData {
    /// The name of the application, if the storage location is in the system application directory, use this application name as a subdirectory
    /// <details><summary><b>中文说明</b></summary>
    /// 应用名称，如果存储位置在系统应用目录，以此应用名作为子目录
    /// </details>
    pub app_name: String,
    /// Whether to force create the data directory under the startup path
    /// <details><summary><b>中文说明</b></summary>
    /// 是否强制在运行目录下创建 data 目录
    /// </details>
    pub force_local: bool,
}

/// Create a new AppData instance
impl AppData {
    pub fn new(app_name: &str) -> Self {
        Self {
            app_name: app_name.to_string(),
            force_local: false,
        }
    }

    pub fn with_force_local(app_name: &str, force_local: bool) -> Self {
        Self {
            app_name: app_name.to_string(),
            force_local,
        }
    }
}

impl AppData {
    /// Return the application data directory according to the search rules, ensuring that the directory is valid and exists
    /// 
    /// <strong>Besides logging, you don't need to care about this method, just keep track of the data file path</strong>
    /// 
    /// <details><summary><b>中文说明</b></summary>
    /// 按照寻找规则返回应用数据目录,会确保目录有效且存在。
    /// <strong>除了记录日志，你并不需要关心此方法,保持关注数据文件路径即可</strong>
    /// </details>
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_data::AppData;
    ///
    /// let app_data = AppData::default();
    /// let data_dir = app_data.ensure_data_dir().unwrap();
    /// println!("data_dir: {}", data_dir.display());
    /// ```
    pub fn ensure_data_dir(&self) -> std::io::Result<PathBuf> {
        let path = current_dir()?;
        let root_path = path.join("data");
        if root_path.exists() {
            return Ok(root_path);
        }
        if self.force_local {
            fs::create_dir_all(&root_path)?;
            return Ok(root_path);
        }
        let sys_path = get_sys_app_data_dir()?.join(&self.app_name);
        if !sys_path.exists() {
            fs::create_dir_all(&sys_path)?;
        }
        Ok(sys_path)
    }

    pub fn get_file_path(&self, file_name: &str) -> std::io::Result<PathBuf> {
        let data_dir = self.ensure_data_dir()?;
        Ok(data_dir.join(file_name))
    }
}

impl Default for AppData {
    /// Default using `CARGO_PKG_NAME` as the application name, if `CARGO_PKG_NAME` is not set, 
    /// then `force_local` is true
    /// 
    /// <details><summary><b>中文说明</b></summary>
    /// 默认使用 `CARGO_PKG_NAME` 作为应用名称，如果 `CARGO_PKG_NAME` 未设置，则`force_local` 为true
    /// </details>
    fn default() -> Self {
        let app_name = env::var("CARGO_PKG_NAME");
        if app_name.is_err() {
            return Self::with_force_local("", true);
        }
        Self::new(&app_name.unwrap())
    }
}
