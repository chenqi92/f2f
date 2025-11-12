use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 能力注册记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRecord {
    pub id: String,
    pub name: String,
    pub version: String,
    pub license: String,
    pub vendor: String,
    pub binary_path: Option<String>,
    pub inputs: Vec<FormatSpec>,
    pub outputs: Vec<FormatSpec>,
    pub quality: f32,
    pub speed: f32,
    pub cost: ResourceCost,
    pub requires: Requirements,
    pub preserve: PreserveFeatures,
    pub risks: Vec<String>,
    pub timeout_s: Option<u32>,
    pub max_mem_mb: Option<u32>,
    pub parallelism: u32,
    pub streaming: bool,
}

/// 格式规格
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatSpec {
    pub mime: String,
    pub extensions: Vec<String>,
}

/// 资源成本
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCost {
    pub cpu: f32,
    pub memory: f32,
    pub gpu: f32,
}

/// 依赖要求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Requirements {
    pub tools: Vec<String>,
    pub lang_packs: Vec<String>,
    pub fonts: Vec<String>,
}

/// 保留特性
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreserveFeatures {
    pub styles: PreserveLevel,
    pub links: PreserveLevel,
    pub footnotes: PreserveLevel,
    pub headers: PreserveLevel,
    pub lists: PreserveLevel,
    pub tables: PreserveLevel,
    pub images: PreserveLevel,
    pub formulas: PreserveLevel,
}

/// 保留级别
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreserveLevel {
    True,
    False,
    Partial,
}

/// 转换路径
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionPath {
    pub steps: Vec<ConversionStep>,
    pub total_quality: f32,
    pub estimated_time_ms: u64,
    pub risks: Vec<String>,
}

/// 转换步骤
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionStep {
    pub converter_id: String,
    pub from_format: String,
    pub to_format: String,
    pub quality: f32,
}

/// 转换目标选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionTarget {
    pub format: String,
    pub mime_type: String,
    pub paths: Vec<ConversionPath>,
    pub recommended: bool,
    pub available: bool,
    pub missing_requirements: Vec<String>,
}

/// 工具健康状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolHealth {
    pub name: String,
    pub available: bool,
    pub version: Option<String>,
    pub path: Option<String>,
    pub error: Option<String>,
}

/// 健康检查报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthReport {
    pub tools: HashMap<String, ToolHealth>,
    pub timestamp: i64,
}
