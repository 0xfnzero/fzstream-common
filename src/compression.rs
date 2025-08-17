use serde::{Serialize, Deserialize};
use crate::CompressionLevel;

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

/// Compress data using the specified compression algorithm
pub fn compress_data(data: &[u8], compression_level: CompressionLevel) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match compression_level {
        CompressionLevel::None => Ok(data.to_vec()),
        CompressionLevel::LZ4Fast => {
            let compressed = lz4::block::compress(data, None, false)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(compressed)
        }
        CompressionLevel::LZ4High => {
            let compressed = lz4::block::compress(data, None, true)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(compressed)
        }
        CompressionLevel::ZstdFast => {
            let compressed = zstd::encode_all(&data[..], 1)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(compressed)
        }
        CompressionLevel::ZstdMedium => {
            let compressed = zstd::encode_all(&data[..], 2)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(compressed)
        }
        CompressionLevel::ZstdHigh => {
            let compressed = zstd::encode_all(&data[..], 3)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(compressed)
        }
        CompressionLevel::ZstdMax => {
            let compressed = zstd::encode_all(&data[..], 4)
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(compressed)
        }
    }
}

/// Decompress data using the specified compression algorithm
pub fn decompress_data(data: &[u8], compression_level: CompressionLevel) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    match compression_level {
        CompressionLevel::None => Ok(data.to_vec()),
        CompressionLevel::LZ4High | CompressionLevel::LZ4Fast=> {
            let decompressed = lz4::block::decompress(data, Some((data.len() * 4) as i32))
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(decompressed)
        }
        CompressionLevel::ZstdFast | CompressionLevel::ZstdMedium | CompressionLevel::ZstdHigh | CompressionLevel::ZstdMax => {
            let decompressed = zstd::decode_all(&data[..])
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
            Ok(decompressed)
        }
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
