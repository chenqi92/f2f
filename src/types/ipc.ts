/**
 * IPC 类型定义 - 与 Rust 后端的契约
 */

// ============================================================================
// 能力相关类型
// ============================================================================

export interface CapabilityRecord {
  id: string;
  name: string;
  version: string;
  license: string;
  vendor: string;
  binary_path?: string;
  inputs: FormatSpec[];
  outputs: FormatSpec[];
  quality: number;
  speed: number;
  cost: ResourceCost;
  requires: Requirements;
  preserve: PreserveFeatures;
  risks: string[];
  timeout_s?: number;
  max_mem_mb?: number;
  parallelism: number;
  streaming: boolean;
}

export interface FormatSpec {
  mime: string;
  extensions: string[];
}

export interface ResourceCost {
  cpu: number;
  memory: number;
  gpu: number;
}

export interface Requirements {
  tools: string[];
  lang_packs: string[];
  fonts: string[];
}

export type PreserveLevel = "true" | "false" | "partial";

export interface PreserveFeatures {
  styles: PreserveLevel;
  links: PreserveLevel;
  footnotes: PreserveLevel;
  headers: PreserveLevel;
  lists: PreserveLevel;
  tables: PreserveLevel;
  images: PreserveLevel;
  formulas: PreserveLevel;
}

export interface ConversionPath {
  steps: ConversionStep[];
  total_quality: number;
  estimated_time_ms: number;
  risks: string[];
}

export interface ConversionStep {
  converter_id: string;
  from_format: string;
  to_format: string;
  quality: number;
}

export interface ConversionTarget {
  format: string;
  mime_type: string;
  paths: ConversionPath[];
  recommended: boolean;
  available: boolean;
  missing_requirements: string[];
}

export interface ToolHealth {
  name: string;
  available: boolean;
  version?: string;
  path?: string;
  error?: string;
}

export interface HealthReport {
  tools: Record<string, ToolHealth>;
  timestamp: number;
}

// ============================================================================
// 任务相关类型
// ============================================================================

export type JobState = "queued" | "running" | "succeeded" | "failed" | "canceled";

export interface Job {
  id: string;
  state: JobState;
  inputs: string[];
  target_format: string;
  recipe_id?: string;
  options: Record<string, unknown>;
  progress: number;
  stage?: string;
  eta_ms?: number;
  created_at: number;
  started_at?: number;
  finished_at?: number;
  error?: string;
}

export type JobAction = "pause" | "resume" | "cancel" | "retry";

export interface JobProgress {
  job_id: string;
  progress: number;
  stage: string;
  eta_ms?: number;
}

export interface JobStateChanged {
  job_id: string;
  old_state: JobState;
  new_state: JobState;
  error?: string;
}

export interface ArtifactReady {
  job_id: string;
  path: string;
  size_bytes: number;
  format: string;
}

export interface Artifact {
  id: string;
  job_id: string;
  path: string;
  filename: string;
  size_bytes: number;
  format: string;
  created_at: number;
}

export type LogLevel = "DEBUG" | "INFO" | "WARN" | "ERROR";

export interface LogEntry {
  timestamp: number;
  level: LogLevel;
  message: string;
  context?: Record<string, unknown>;
}

// ============================================================================
// 转换相关类型
// ============================================================================

export interface FileDetection {
  path: string;
  mime_type: string;
  extension: string;
  size_bytes: number;
  is_scanned_pdf: boolean;
  metadata: Record<string, unknown>;
}

export type QualityPriority = "quality" | "balanced" | "speed";

export interface ConversionOptions {
  quality_priority: QualityPriority;
  max_file_size_mb?: number;
  ocr_enabled: boolean;
  ocr_languages: string[];
  preserve_metadata: boolean;
  custom_params: Record<string, unknown>;
}

export interface Recipe {
  id: string;
  name: string;
  description: string;
  nodes: RecipeNode[];
  edges: RecipeEdge[];
  created_at: number;
  updated_at: number;
}

export type NodeType =
  | "input"
  | "converter"
  | "preprocessor"
  | "postprocessor"
  | "ocr"
  | "llm"
  | "validator"
  | "output";

export interface RecipeNode {
  id: string;
  node_type: NodeType;
  config: Record<string, unknown>;
  position: NodePosition;
}

export interface NodePosition {
  x: number;
  y: number;
}

export interface RecipeEdge {
  from: string;
  to: string;
  condition?: string;
}

export interface QualityMetrics {
  ocr_cer?: number;
  structure_consistency?: number;
  table_recall?: number;
  media_bitrate_deviation?: number;
  schema_validation_passed: boolean;
}

// ============================================================================
// 错误类型
// ============================================================================

export type ErrorKind =
  | "VALIDATION_ERROR"
  | "RESOURCE_LIMIT"
  | "TOOL_ERROR"
  | "IO_ERROR"
  | "CANCELED"
  | "DATABASE_ERROR"
  | "INTERNAL";

export interface ErrorResponse {
  kind: ErrorKind;
  message: string;
  suggestion?: string;
}

// ============================================================================
// 设置类型
// ============================================================================

export interface AppSettings {
  theme: "light" | "dark" | "system";
  language: string;
  quality_priority: QualityPriority;
  max_concurrent_jobs: number;
  cache_size_mb: number;
  ocr_enabled: boolean;
  ocr_languages: string[];
}
