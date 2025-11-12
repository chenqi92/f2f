use crate::error::Result;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager};

/// 全局存储实例
static STORAGE: Lazy<Arc<Storage>> = Lazy::new(|| {
    Arc::new(Storage::new())
});

/// 存储管理器 - 工作区、缓存、数据库
pub struct Storage {
    workspace_dir: Option<PathBuf>,
    cache_dir: Option<PathBuf>,
}

impl Storage {
    pub fn new() -> Self {
        Self {
            workspace_dir: None,
            cache_dir: None,
        }
    }

    /// 获取全局存储实例
    pub fn global() -> Arc<Storage> {
        STORAGE.clone()
    }

    /// 初始化存储目录
    pub fn init(&mut self, app_handle: &AppHandle) -> Result<()> {
        // 获取应用数据目录
        let app_data_dir = app_handle
            .path()
            .app_data_dir()
            .map_err(|e| crate::error::AppError::Internal(e.to_string()))?;

        self.workspace_dir = Some(app_data_dir.join("workspace"));
        self.cache_dir = Some(app_data_dir.join("cache"));

        // 创建目录
        std::fs::create_dir_all(self.workspace_dir.as_ref().unwrap())?;
        std::fs::create_dir_all(self.cache_dir.as_ref().unwrap())?;

        tracing::info!("Storage initialized at {:?}", app_data_dir);

        Ok(())
    }

    /// 获取工作区目录
    pub fn workspace_dir(&self) -> Option<&PathBuf> {
        self.workspace_dir.as_ref()
    }

    /// 获取缓存目录
    pub fn cache_dir(&self) -> Option<&PathBuf> {
        self.cache_dir.as_ref()
    }

    /// 创建临时工作目录
    pub fn create_temp_dir(&self) -> Result<PathBuf> {
        let temp_dir = tempfile::tempdir()
            .map_err(|e| crate::error::AppError::IoError(e.to_string()))?;

        Ok(temp_dir.into_path())
    }

    /// 计算文件哈希（用于缓存）
    pub fn compute_hash(path: &PathBuf) -> Result<String> {
        use sha2::{Sha256, Digest};

        let content = std::fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();

        Ok(hex::encode(result))
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

/// 初始化存储
pub fn initialize(app_handle: &AppHandle) -> Result<()> {
    let mut storage = Storage::new();
    storage.init(app_handle)?;

    // TODO: 初始化 SQLite 数据库

    Ok(())
}
