use std::{
    env::{self, current_dir, var},
    fmt, fs,
    path::PathBuf,
};

/// Custom error type
#[derive(Debug, Clone)]
pub enum AppDataError {
    /// Environment variable not found
    EnvVarNotFound(String),
    /// IO error
    IoError(String),
    /// Failed to get current directory
    CurrentDirError(String),
}

impl fmt::Display for AppDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppDataError::EnvVarNotFound(var) => {
                write!(f, "Environment variable {} not found", var)
            }
            AppDataError::IoError(msg) => {
                write!(f, "IO error: {}", msg)
            }
            AppDataError::CurrentDirError(msg) => {
                write!(f, "Failed to get current directory: {}", msg)
            }
        }
    }
}

impl std::error::Error for AppDataError {}

impl From<std::io::Error> for AppDataError {
    fn from(err: std::io::Error) -> Self {
        AppDataError::IoError(err.to_string())
    }
}

#[cfg(target_os = "windows")]
pub fn get_sys_app_data_dir() -> Result<PathBuf, AppDataError> {
    var("APPDATA")
        .map(PathBuf::from)
        .map_err(|_| AppDataError::EnvVarNotFound("APPDATA".to_string()))
}

#[cfg(target_os = "macos")]
pub fn get_sys_app_data_dir() -> Result<PathBuf, AppDataError> {
    var("HOME")
        .map(|home| PathBuf::from(home).join("Library/Application Support"))
        .map_err(|_| AppDataError::EnvVarNotFound("HOME".to_string()))
}

#[cfg(target_os = "linux")]
pub fn get_sys_app_data_dir() -> Result<PathBuf, AppDataError> {
    if let Ok(xdg) = var("XDG_DATA_HOME") {
        Ok(PathBuf::from(xdg))
    } else if let Ok(home) = var("HOME") {
        Ok(PathBuf::from(home).join(".local/share"))
    } else {
        Err(AppDataError::EnvVarNotFound(
            "XDG_DATA_HOME and HOME".to_string(),
        ))
    }
}

/// # Examples
///
/// ```rust
/// use app_data::AppData;
///
/// let app_data = AppData::default();
///
/// let app_data = AppData::new("my_app");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
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
    pub fn ensure_data_dir(&self) -> Result<PathBuf, AppDataError> {
        let path = current_dir().map_err(|e| AppDataError::CurrentDirError(e.to_string()))?;
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

    /// 获取数据目录中的文件路径
    ///
    /// Get the file path in the data directory
    ///
    /// # Examples
    ///
    /// ```rust
    /// use app_data::AppData;
    ///
    /// let app_data = AppData::new("my_app");
    /// let file_path = app_data.get_file_path("config.json").unwrap();
    /// ```
    pub fn get_file_path(&self, file_name: &str) -> Result<PathBuf, AppDataError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_app_data_new() {
        let app_data = AppData::new("test_app");
        assert_eq!(app_data.app_name, "test_app");
        assert_eq!(app_data.force_local, false);
    }

    #[test]
    fn test_app_data_with_force_local() {
        let app_data = AppData::with_force_local("test_app", true);
        assert_eq!(app_data.app_name, "test_app");
        assert_eq!(app_data.force_local, true);
    }

    #[test]
    fn test_app_data_debug() {
        let app_data = AppData::new("test_app");
        let debug_str = format!("{:?}", app_data);
        assert!(debug_str.contains("test_app"));
    }

    #[test]
    fn test_app_data_clone() {
        let app_data = AppData::new("test_app");
        let cloned = app_data.clone();
        assert_eq!(app_data, cloned);
    }

    #[test]
    fn test_app_data_partial_eq() {
        let app_data1 = AppData::new("test_app");
        let app_data2 = AppData::new("test_app");
        let app_data3 = AppData::new("other_app");
        assert_eq!(app_data1, app_data2);
        assert_ne!(app_data1, app_data3);
    }

    #[test]
    fn test_ensure_data_dir_force_local() {
        let app_data = AppData::with_force_local("test_app", true);
        let result = app_data.ensure_data_dir();
        assert!(result.is_ok());
        let data_dir = result.unwrap();
        assert!(data_dir.exists());
        assert!(data_dir.is_dir());
        assert!(data_dir.ends_with("data"));

        // 清理
        let _ = fs::remove_dir_all(&data_dir);
    }

    #[test]
    fn test_get_file_path() {
        let app_data = AppData::with_force_local("test_app", true);
        let result = app_data.get_file_path("test.txt");
        assert!(result.is_ok());
        let file_path = result.unwrap();
        assert!(file_path.ends_with("test.txt"));

        // 清理
        if let Ok(data_dir) = app_data.ensure_data_dir() {
            let _ = fs::remove_dir_all(&data_dir);
        }
    }

    #[test]
    fn test_app_data_error_display() {
        let error = AppDataError::EnvVarNotFound("TEST_VAR".to_string());
        let error_str = format!("{}", error);
        assert!(error_str.contains("TEST_VAR"));
    }

    #[test]
    fn test_app_data_error_from_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "test");
        let app_error: AppDataError = io_error.into();
        match app_error {
            AppDataError::IoError(_) => {}
            _ => panic!("Expected IoError"),
        }
    }
}
