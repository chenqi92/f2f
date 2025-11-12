use crate::error::Result;
use crate::types::{Job, JobState, ConversionPath};
use std::sync::Arc;
use tokio::sync::RwLock;

/// 流水线执行器 - DAG 调度、并发、重试、回退
pub struct Pipeline {
    jobs: Arc<RwLock<Vec<Job>>>,
}

impl Pipeline {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 提交任务到队列
    pub async fn submit(&self, mut job: Job) -> Result<String> {
        let job_id = job.id.clone();
        job.state = JobState::Queued;

        let mut jobs = self.jobs.write().await;
        jobs.push(job);

        tracing::info!("Job {} submitted to queue", job_id);
        Ok(job_id)
    }

    /// 执行任务
    pub async fn execute(&self, job_id: &str, path: ConversionPath) -> Result<()> {
        // TODO: 实现实际的执行逻辑
        // 1. 更新任务状态为 Running
        // 2. 按 path.steps 顺序执行每个转换步骤
        // 3. 处理错误和重试
        // 4. 更新进度
        // 5. 生成产物

        tracing::info!("Executing job {}", job_id);

        // 简化实现
        self.update_job_state(job_id, JobState::Running).await?;

        // 模拟执行
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        self.update_job_state(job_id, JobState::Succeeded).await?;

        Ok(())
    }

    /// 更新任务状态
    async fn update_job_state(&self, job_id: &str, new_state: JobState) -> Result<()> {
        let mut jobs = self.jobs.write().await;

        if let Some(job) = jobs.iter_mut().find(|j| j.id == job_id) {
            job.state = new_state;
            match job.state {
                JobState::Running => {
                    job.started_at = Some(chrono::Utc::now().timestamp_millis());
                }
                JobState::Succeeded | JobState::Failed | JobState::Canceled => {
                    job.finished_at = Some(chrono::Utc::now().timestamp_millis());
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// 取消任务
    pub async fn cancel(&self, job_id: &str) -> Result<()> {
        self.update_job_state(job_id, JobState::Canceled).await
    }

    /// 获取任务
    pub async fn get_job(&self, job_id: &str) -> Result<Option<Job>> {
        let jobs = self.jobs.read().await;
        Ok(jobs.iter().find(|j| j.id == job_id).cloned())
    }

    /// 列出所有任务
    pub async fn list_jobs(&self) -> Result<Vec<Job>> {
        let jobs = self.jobs.read().await;
        Ok(jobs.clone())
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Self::new()
    }
}
