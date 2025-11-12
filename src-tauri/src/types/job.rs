use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum JobState {
    Queued,
    Running,
    Succeeded,
    Failed,
    Canceled,
}

/// 任务记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub state: JobState,
    pub inputs: Vec<String>,
    pub target_format: String,
    pub recipe_id: Option<String>,
    pub options: serde_json::Value,
    pub progress: f32,
    pub stage: Option<String>,
    pub eta_ms: Option<u64>,
    pub created_at: i64,
    pub started_at: Option<i64>,
    pub finished_at: Option<i64>,
    pub error: Option<String>,
}

impl Job {
    pub fn new(
        inputs: Vec<String>,
        target_format: String,
        recipe_id: Option<String>,
        options: serde_json::Value,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            state: JobState::Queued,
            inputs,
            target_format,
            recipe_id,
            options,
            progress: 0.0,
            stage: None,
            eta_ms: None,
            created_at: chrono::Utc::now().timestamp_millis(),
            started_at: None,
            finished_at: None,
            error: None,
        }
    }
}

/// 任务控制动作
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobAction {
    Pause,
    Resume,
    Cancel,
    Retry,
}

/// 任务进度事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobProgress {
    pub job_id: String,
    pub progress: f32,
    pub stage: String,
    pub eta_ms: Option<u64>,
}

/// 任务状态变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobStateChanged {
    pub job_id: String,
    pub old_state: JobState,
    pub new_state: JobState,
    pub error: Option<String>,
}

/// 产物就绪事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactReady {
    pub job_id: String,
    pub path: String,
    pub size_bytes: u64,
    pub format: String,
}

/// 产物信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub id: String,
    pub job_id: String,
    pub path: String,
    pub filename: String,
    pub size_bytes: u64,
    pub format: String,
    pub created_at: i64,
}

/// 日志条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: i64,
    pub level: LogLevel,
    pub message: String,
    pub context: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}
