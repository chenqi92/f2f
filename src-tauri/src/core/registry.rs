use crate::error::Result;
use crate::types::{CapabilityRecord, ConversionTarget, ToolHealth, HealthReport};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;

/// 全局能力注册表实例
static REGISTRY: Lazy<Arc<Registry>> = Lazy::new(|| Arc::new(Registry::new()));

/// 能力注册表 - 管理所有转换器的能力声明
pub struct Registry {
    /// 所有注册的能力
    capabilities: DashMap<String, CapabilityRecord>,
    /// 格式到转换器的映射 (from_mime -> to_mime -> converter_ids)
    format_graph: DashMap<String, DashMap<String, Vec<String>>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            capabilities: DashMap::new(),
            format_graph: DashMap::new(),
        }
    }

    /// 获取全局注册表实例
    pub fn global() -> Arc<Registry> {
        REGISTRY.clone()
    }

    /// 注册一个转换能力
    pub fn register(&self, capability: CapabilityRecord) -> Result<()> {
        let id = capability.id.clone();

        // 更新格式图
        for input in &capability.inputs {
            for input_mime in std::iter::once(&input.mime).chain(input.extensions.iter()) {
                for output in &capability.outputs {
                    for output_mime in std::iter::once(&output.mime).chain(output.extensions.iter()) {
                        self.format_graph
                            .entry(input_mime.clone())
                            .or_insert_with(DashMap::new)
                            .entry(output_mime.clone())
                            .or_insert_with(Vec::new)
                            .push(id.clone());
                    }
                }
            }
        }

        self.capabilities.insert(id, capability);
        Ok(())
    }

    /// 获取能力记录
    pub fn get(&self, id: &str) -> Option<CapabilityRecord> {
        self.capabilities.get(id).map(|r| r.clone())
    }

    /// 获取所有能力
    pub fn list_all(&self) -> Vec<CapabilityRecord> {
        self.capabilities
            .iter()
            .map(|entry| entry.value().clone())
            .collect()
    }

    /// 查找从源格式到目标格式的转换器
    pub fn find_converters(&self, from: &str, to: &str) -> Vec<String> {
        self.format_graph
            .get(from)
            .and_then(|targets| targets.get(to).map(|ids| ids.clone()))
            .unwrap_or_default()
    }

    /// 获取给定源格式的所有可达目标格式
    pub fn get_targets(&self, source_mime: &str) -> Vec<ConversionTarget> {
        // 简化实现 - 实际应该进行图搜索
        let mut targets = Vec::new();

        if let Some(target_map) = self.format_graph.get(source_mime) {
            for entry in target_map.iter() {
                let to_mime = entry.key();
                let converter_ids = entry.value();

                if !converter_ids.is_empty() {
                    targets.push(ConversionTarget {
                        format: to_mime.clone(),
                        mime_type: to_mime.clone(),
                        paths: vec![], // TODO: 计算实际路径
                        recommended: false,
                        available: true,
                        missing_requirements: vec![],
                    });
                }
            }
        }

        targets
    }

    /// 刷新注册表 - 重新扫描所有工具
    pub fn refresh(&self) -> Result<()> {
        self.capabilities.clear();
        self.format_graph.clear();

        // TODO: 扫描和注册所有可用工具
        tracing::info!("Registry refreshed");

        Ok(())
    }

    /// 检查工具健康状态
    pub fn check_health(&self) -> HealthReport {
        let mut tools = HashMap::new();
        let timestamp = chrono::Utc::now().timestamp_millis();

        // TODO: 实际检查各个工具的可用性
        // 这里只是示例
        tools.insert(
            "pandoc".to_string(),
            ToolHealth {
                name: "Pandoc".to_string(),
                available: false,
                version: None,
                path: None,
                error: Some("未安装".to_string()),
            },
        );

        HealthReport { tools, timestamp }
    }
}

/// 初始化注册表
pub fn initialize(_app_handle: &AppHandle) -> Result<()> {
    let registry = Registry::global();

    // 注册内置转换器
    register_builtin_converters(&registry)?;

    // TODO: 扫描并注册外部工具

    tracing::info!("Registry initialized with {} capabilities", registry.capabilities.len());

    Ok(())
}

/// 注册内置转换器（示例）
fn register_builtin_converters(registry: &Registry) -> Result<()> {
    use crate::types::*;

    // 示例：Pandoc DOCX -> Markdown 转换器
    registry.register(CapabilityRecord {
        id: "pandoc-docx-md".to_string(),
        name: "Pandoc DOCX to Markdown".to_string(),
        version: "3.0.0".to_string(),
        license: "GPL-2.0".to_string(),
        vendor: "John MacFarlane".to_string(),
        binary_path: None,
        inputs: vec![FormatSpec {
            mime: "application/vnd.openxmlformats-officedocument.wordprocessingml.document".to_string(),
            extensions: vec!["docx".to_string()],
        }],
        outputs: vec![FormatSpec {
            mime: "text/markdown".to_string(),
            extensions: vec!["md".to_string()],
        }],
        quality: 0.9,
        speed: 0.8,
        cost: ResourceCost {
            cpu: 0.5,
            memory: 0.3,
            gpu: 0.0,
        },
        requires: Requirements {
            tools: vec!["pandoc".to_string()],
            lang_packs: vec![],
            fonts: vec![],
        },
        preserve: PreserveFeatures {
            styles: PreserveLevel::Partial,
            links: PreserveLevel::True,
            footnotes: PreserveLevel::True,
            headers: PreserveLevel::True,
            lists: PreserveLevel::True,
            tables: PreserveLevel::True,
            images: PreserveLevel::True,
            formulas: PreserveLevel::Partial,
        },
        risks: vec!["样式可能丢失".to_string()],
        timeout_s: Some(300),
        max_mem_mb: Some(512),
        parallelism: 1,
        streaming: false,
    })?;

    Ok(())
}
