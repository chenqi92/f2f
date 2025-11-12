use crate::core::{FileDetector, Planner, Pipeline, Registry};
use crate::error::ErrorResponse;
use crate::types::*;
use serde_json::Value;
use std::path::Path;

/// 探测系统能力
#[tauri::command]
pub async fn detect_capabilities() -> std::result::Result<Vec<CapabilityRecord>, ErrorResponse> {
    Registry::global()
        .list_all()
        .pipe(Ok)
        .map_err(|e: crate::error::AppError| e.into())
}

/// 探测文件格式
#[tauri::command]
pub async fn detect_file(file_path: String) -> std::result::Result<FileDetection, ErrorResponse> {
    let path = Path::new(&file_path);
    FileDetector::detect(path)
        .map_err(|e| e.into())
}

/// 规划目标格式
#[tauri::command]
pub async fn plan_targets(file_path: String) -> std::result::Result<Vec<ConversionTarget>, ErrorResponse> {
    // 探测文件格式
    let path = Path::new(&file_path);
    let detection = FileDetector::detect(path)
        .map_err(|e| -> ErrorResponse { e.into() })?;

    let planner = Planner::new();
    planner
        .get_available_targets(&detection.mime_type)
        .pipe(Ok)
        .map_err(|e: crate::error::AppError| e.into())
}

/// 创建转换任务
#[tauri::command]
pub async fn create_job(
    inputs: Vec<String>,
    target: String,
    recipe_id: Option<String>,
    options: Value,
) -> std::result::Result<String, ErrorResponse> {
    let job = Job::new(inputs, target, recipe_id, options);
    let job_id = job.id.clone();

    let pipeline = Pipeline::new();
    pipeline
        .submit(job)
        .await
        .map_err(|e| e.into())
}

/// 列出任务
#[tauri::command]
pub async fn list_jobs() -> std::result::Result<Vec<Job>, ErrorResponse> {
    let pipeline = Pipeline::new();
    pipeline
        .list_jobs()
        .await
        .map_err(|e| e.into())
}

/// 获取任务详情
#[tauri::command]
pub async fn get_job(job_id: String) -> std::result::Result<Option<Job>, ErrorResponse> {
    let pipeline = Pipeline::new();
    pipeline
        .get_job(&job_id)
        .await
        .map_err(|e| e.into())
}

/// 控制任务
#[tauri::command]
pub async fn control_job(job_id: String, action: JobAction) -> std::result::Result<(), ErrorResponse> {
    let pipeline = Pipeline::new();

    match action {
        JobAction::Cancel => pipeline.cancel(&job_id).await,
        _ => Ok(()), // TODO: 实现其他动作
    }
    .map_err(|e| e.into())
}

/// 获取产物列表
#[tauri::command]
pub async fn get_artifacts(job_id: String) -> std::result::Result<Vec<Artifact>, ErrorResponse> {
    // TODO: 实现
    Ok(vec![])
}

/// 获取任务日志
#[tauri::command]
pub async fn get_logs(job_id: String, cursor: Option<i64>) -> std::result::Result<Vec<LogEntry>, ErrorResponse> {
    // TODO: 实现
    Ok(vec![])
}

/// 获取设置
#[tauri::command]
pub async fn get_settings() -> std::result::Result<Value, ErrorResponse> {
    // TODO: 实现
    Ok(serde_json::json!({
        "theme": "system",
        "language": "zh-CN",
        "quality_priority": "balanced"
    }))
}

/// 设置配置
#[tauri::command]
pub async fn set_settings(settings: Value) -> std::result::Result<(), ErrorResponse> {
    // TODO: 实现
    Ok(())
}

/// 运行健康检查
#[tauri::command]
pub async fn run_health_check() -> std::result::Result<HealthReport, ErrorResponse> {
    Registry::global()
        .check_health()
        .pipe(Ok)
        .map_err(|e: crate::error::AppError| e.into())
}

// 辅助 trait 用于 pipe 语法
trait Pipe: Sized {
    fn pipe<F, R>(self, f: F) -> R
    where
        F: FnOnce(Self) -> R,
    {
        f(self)
    }
}

impl<T> Pipe for T {}
