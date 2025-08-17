use serde::{Serialize, Deserialize};

/// 压缩统计信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompressionStats {
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub compression_time_ms: u64,
    pub decompression_time_ms: u64,
}

impl CompressionStats {
    pub fn new(original_size: u64, compressed_size: u64, compression_time_ms: u64, decompression_time_ms: u64) -> Self {
        let compression_ratio = if original_size > 0 {
            compressed_size as f64 / original_size as f64
        } else {
            0.0
        };
        
        Self {
            original_size,
            compressed_size,
            compression_ratio,
            compression_time_ms,
            decompression_time_ms,
        }
    }
    
    pub fn compression_percentage(&self) -> f64 {
        (1.0 - self.compression_ratio) * 100.0
    }
}

/// 压缩算法信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionInfo {
    pub algorithm: String,
    pub level: u32,
    pub dictionary_size: Option<usize>,
    pub block_size: Option<usize>,
}

/// 压缩配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompressionConfig {
    pub enabled: bool,
    pub algorithm: String,
    pub level: u32,
    pub threshold_size: usize,
    pub max_compression_time_ms: u64,
}

impl Default for CompressionConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            algorithm: "lz4".to_string(),
            level: 1,
            threshold_size: 1024,
            max_compression_time_ms: 100,
        }
    }
}
