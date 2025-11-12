use serde::{Deserialize, Serialize};

/// 文件探测结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDetection {
    pub path: String,
    pub mime_type: String,
    pub extension: String,
    pub size_bytes: u64,
    pub is_scanned_pdf: bool,
    pub metadata: serde_json::Value,
}

/// 转换选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionOptions {
    pub quality_priority: QualityPriority,
    pub max_file_size_mb: Option<u32>,
    pub ocr_enabled: bool,
    pub ocr_languages: Vec<String>,
    pub preserve_metadata: bool,
    pub custom_params: serde_json::Value,
}

impl Default for ConversionOptions {
    fn default() -> Self {
        Self {
            quality_priority: QualityPriority::Balanced,
            max_file_size_mb: None,
            ocr_enabled: false,
            ocr_languages: vec!["eng".to_string(), "chi_sim".to_string()],
            preserve_metadata: true,
            custom_params: serde_json::Value::Object(Default::default()),
        }
    }
}

/// 质量优先级
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum QualityPriority {
    Quality,
    Balanced,
    Speed,
}

/// 食谱定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: String,
    pub name: String,
    pub description: String,
    pub nodes: Vec<RecipeNode>,
    pub edges: Vec<RecipeEdge>,
    pub created_at: i64,
    pub updated_at: i64,
}

/// 食谱节点
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeNode {
    pub id: String,
    pub node_type: NodeType,
    pub config: serde_json::Value,
    pub position: NodePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodePosition {
    pub x: f32,
    pub y: f32,
}

/// 节点类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Input,
    Converter,
    Preprocessor,
    Postprocessor,
    Ocr,
    Llm,
    Validator,
    Output,
}

/// 食谱边
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeEdge {
    pub from: String,
    pub to: String,
    pub condition: Option<String>,
}

/// 质量指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityMetrics {
    pub ocr_cer: Option<f32>,
    pub structure_consistency: Option<f32>,
    pub table_recall: Option<f32>,
    pub media_bitrate_deviation: Option<f32>,
    pub schema_validation_passed: bool,
}
