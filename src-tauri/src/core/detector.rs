use crate::error::{AppError, Result};
use crate::types::FileDetection;
use std::path::Path;

/// 文件格式探测器
pub struct FileDetector;

impl FileDetector {
    /// 探测文件的 MIME 类型和元数据
    pub fn detect(path: &Path) -> Result<FileDetection> {
        if !path.exists() {
            return Err(AppError::ValidationError(format!(
                "文件不存在: {}",
                path.display()
            )));
        }

        let metadata = std::fs::metadata(path)?;
        let size_bytes = metadata.len();

        // 读取文件头部用于探测
        let buffer = std::fs::read(path)
            .or_else(|_| -> std::result::Result<Vec<u8>, std::io::Error> {
                // 如果文件太大，只读取前 8KB
                let mut file = std::fs::File::open(path)?;
                use std::io::Read;
                let mut buffer = vec![0; 8192];
                let _ = file.read(&mut buffer)?;
                Ok(buffer)
            })?;

        // 使用 infer 探测 MIME 类型
        let mime_type = infer::get(&buffer)
            .map(|t| t.mime_type().to_string())
            .or_else(|| {
                // 回退到基于扩展名的猜测
                path.extension()
                    .and_then(|ext| ext.to_str())
                    .and_then(|ext| mime_guess::from_ext(ext).first())
                    .map(|mime| mime.to_string())
            })
            .unwrap_or_else(|| "application/octet-stream".to_string());

        let extension = path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        // 检测是否是扫描PDF（通过简单启发式）
        let is_scanned_pdf = mime_type == "application/pdf"
            && Self::is_likely_scanned_pdf(&buffer);

        Ok(FileDetection {
            path: path.to_string_lossy().to_string(),
            mime_type,
            extension,
            size_bytes,
            is_scanned_pdf,
            metadata: serde_json::json!({
                "modified": metadata.modified().ok().and_then(|t| {
                    t.duration_since(std::time::UNIX_EPOCH)
                        .ok()
                        .map(|d| d.as_secs())
                }),
            }),
        })
    }

    /// 简单启发式：检测PDF是否可能是扫描件
    fn is_likely_scanned_pdf(buffer: &[u8]) -> bool {
        // 这是一个简化的启发式方法
        // 实际实现需要解析PDF结构来判断文本对象的数量
        // 这里只是示例，检查是否包含"Image"关键字
        let content = String::from_utf8_lossy(buffer);
        let image_count = content.matches("/Image").count();
        let text_count = content.matches("/Text").count();

        // 如果图片多于文本，可能是扫描件
        image_count > 10 && text_count < 5
    }

    /// 批量探测多个文件
    pub fn detect_batch(paths: &[&Path]) -> Vec<Result<FileDetection>> {
        paths.iter().map(|path| Self::detect(path)).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_detect_text_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let mut file = std::fs::File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        let detection = FileDetector::detect(&file_path).unwrap();
        assert_eq!(detection.extension, "txt");
        assert!(detection.mime_type.contains("text"));
    }
}
