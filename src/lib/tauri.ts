/**
 * Tauri IPC 封装 - 类型安全的命令调用
 */
import { invoke } from "@tauri-apps/api/core";
import type {
  CapabilityRecord,
  ConversionTarget,
  Job,
  JobAction,
  Artifact,
  LogEntry,
  HealthReport,
  ErrorResponse,
} from "@/types/ipc";

/**
 * 探测系统能力
 */
export async function detectCapabilities(): Promise<CapabilityRecord[]> {
  return invoke<CapabilityRecord[]>("detect_capabilities");
}

/**
 * 规划目标格式
 */
export async function planTargets(filePath: string): Promise<ConversionTarget[]> {
  return invoke<ConversionTarget[]>("plan_targets", { filePath });
}

/**
 * 创建转换任务
 */
export async function createJob(
  inputs: string[],
  target: string,
  recipeId?: string,
  options?: Record<string, unknown>,
): Promise<string> {
  return invoke<string>("create_job", {
    inputs,
    target,
    recipeId,
    options: options ?? {},
  });
}

/**
 * 列出任务
 */
export async function listJobs(): Promise<Job[]> {
  return invoke<Job[]>("list_jobs");
}

/**
 * 获取任务详情
 */
export async function getJob(jobId: string): Promise<Job | null> {
  return invoke<Job | null>("get_job", { jobId });
}

/**
 * 控制任务
 */
export async function controlJob(jobId: string, action: JobAction): Promise<void> {
  return invoke<void>("control_job", { jobId, action });
}

/**
 * 获取产物列表
 */
export async function getArtifacts(jobId: string): Promise<Artifact[]> {
  return invoke<Artifact[]>("get_artifacts", { jobId });
}

/**
 * 获取任务日志
 */
export async function getLogs(jobId: string, cursor?: number): Promise<LogEntry[]> {
  return invoke<LogEntry[]>("get_logs", { jobId, cursor });
}

/**
 * 获取设置
 */
export async function getSettings(): Promise<Record<string, unknown>> {
  return invoke<Record<string, unknown>>("get_settings");
}

/**
 * 设置配置
 */
export async function setSettings(settings: Record<string, unknown>): Promise<void> {
  return invoke<void>("set_settings", { settings });
}

/**
 * 运行健康检查
 */
export async function runHealthCheck(): Promise<HealthReport> {
  return invoke<HealthReport>("run_health_check");
}

/**
 * 错误处理辅助函数
 */
export function isErrorResponse(error: unknown): error is ErrorResponse {
  return (
    typeof error === "object" &&
    error !== null &&
    "kind" in error &&
    "message" in error
  );
}

/**
 * 格式化错误消息
 */
export function formatError(error: unknown): string {
  if (isErrorResponse(error)) {
    return error.suggestion
      ? `${error.message}\n建议: ${error.suggestion}`
      : error.message;
  }
  return String(error);
}
