use crate::error::Result;
use crate::types::{ConversionPath, ConversionStep, ConversionTarget};
use crate::core::registry::Registry;

/// 转换规划器 - 在转换图上寻径与评分
pub struct Planner {
    registry: std::sync::Arc<Registry>,
}

impl Planner {
    pub fn new() -> Self {
        Self {
            registry: Registry::global(),
        }
    }

    /// 规划从源格式到目标格式的转换路径
    pub fn plan(
        &self,
        source_mime: &str,
        target_mime: &str,
        quality_threshold: f32,
    ) -> Result<Vec<ConversionPath>> {
        // TODO: 实现图搜索算法（BFS/Dijkstra）
        // 1. 直达路径优先
        // 2. 考虑质量门槛
        // 3. 多候选路径
        // 4. 依赖检查

        let mut paths = Vec::new();

        // 简化实现：只尝试直接转换
        let converters = self.registry.find_converters(source_mime, target_mime);

        for converter_id in converters {
            if let Some(capability) = self.registry.get(&converter_id) {
                if capability.quality >= quality_threshold {
                    paths.push(ConversionPath {
                        steps: vec![ConversionStep {
                            converter_id: converter_id.clone(),
                            from_format: source_mime.to_string(),
                            to_format: target_mime.to_string(),
                            quality: capability.quality,
                        }],
                        total_quality: capability.quality,
                        estimated_time_ms: 1000, // TODO: 实际估算
                        risks: capability.risks.clone(),
                    });
                }
            }
        }

        // 按质量排序
        paths.sort_by(|a, b| {
            b.total_quality
                .partial_cmp(&a.total_quality)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(paths)
    }

    /// 获取给定文件的所有可能目标格式
    pub fn get_available_targets(&self, source_mime: &str) -> Vec<ConversionTarget> {
        self.registry.get_targets(source_mime)
    }

    /// 评估路径质量得分
    fn score_path(&self, path: &ConversionPath) -> f32 {
        // TODO: 综合考虑质量、速度、成本
        path.total_quality
    }
}

impl Default for Planner {
    fn default() -> Self {
        Self::new()
    }
}
