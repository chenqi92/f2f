use crate::error::Result;
use std::path::Path;
use std::process::{Command, Output};

/// 工具适配器 - 沙箱调用外部工具
pub struct Adapter;

impl Adapter {
    /// 在沙箱中执行命令
    pub fn execute_sandboxed(
        tool: &str,
        args: &[&str],
        _input_path: &Path,
        _output_path: &Path,
        _timeout_s: Option<u32>,
        _max_mem_mb: Option<u32>,
    ) -> Result<Output> {
        // TODO: 实现沙箱机制
        // 1. 资源限制（CPU/内存/时间）
        // 2. 文件系统白名单
        // 3. 错误处理
        // 4. 日志记录

        tracing::info!(
            "Executing tool: {} with args: {:?}",
            tool,
            args
        );

        let output = Command::new(tool)
            .args(args)
            .output()
            .map_err(|e| crate::error::AppError::ToolError(format!("Failed to execute {}: {}", tool, e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(crate::error::AppError::ToolError(format!(
                "Tool {} failed: {}",
                tool, stderr
            )));
        }

        Ok(output)
    }

    /// 检查工具是否可用
    pub fn check_tool(tool: &str) -> bool {
        Command::new(tool)
            .arg("--version")
            .output()
            .is_ok()
    }

    /// 获取工具版本
    pub fn get_tool_version(tool: &str) -> Option<String> {
        let output = Command::new(tool)
            .arg("--version")
            .output()
            .ok()?;

        if output.status.success() {
            Some(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            None
        }
    }
}
