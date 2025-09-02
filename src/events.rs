use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::{CompressionLevel, SerializationProtocol};
pub use solana_streamer_sdk::streaming::event_parser::common::EventType;
pub use solana_streamer_sdk::streaming::event_parser::common::EventMetadata;

/// QUIC服务器和客户端之间传输的事件消息（自描述格式）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessage {
    pub event_id: String,
    pub event_type: EventType,
    pub data: Vec<u8>,  // 序列化和可能压缩的数据
    pub timestamp: u64,
    // 新增：格式元数据（服务器端决定，客户端自动适应）
    pub serialization_format: SerializationProtocol,
    pub compression_format: CompressionLevel,
    pub is_compressed: bool,  // 明确指示数据是否被压缩
    pub original_size: Option<usize>, // 压缩前的原始大小（用于验证）
    // 新增：时间戳字段用于性能分析
    pub grpc_arrival_time: u64,      // 1. 交易grpc到达时间
    pub parsing_time: u64,           // 2. 交易解析时间
    pub completion_time: u64,        // 3. 交易完成时间(到用户)
    pub client_processing_start: Option<u64>, // 客户端开始处理时间
    pub client_processing_end: Option<u64>,   // 客户端处理完成时间
}

impl EventMessage {
    pub fn new(
        event_type: EventType,
        data: Vec<u8>,
        serialization_format: SerializationProtocol,
        compression_format: CompressionLevel,
        is_compressed: bool,
    ) -> Self {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        // 🚀 CRITICAL FIX: Actually compress the data when is_compressed=true
        let (final_data, final_original_size, final_is_compressed) = if is_compressed && compression_format != CompressionLevel::None {
            let original_size = data.len();
            match compression_format {
                CompressionLevel::LZ4Fast | CompressionLevel::LZ4High => {
                    match lz4::block::compress(&data, None, false) {
                        Ok(compressed_data) => {
                            if compressed_data.len() < data.len() {
                                // Compression successful
                                (compressed_data, Some(original_size), true)
                            } else {
                                // Compression not beneficial, store uncompressed
                                (data, None, false)
                            }
                        }
                        Err(_) => {
                            // Compression failed, store uncompressed  
                            (data, None, false)
                        }
                    }
                }
                CompressionLevel::ZstdFast | CompressionLevel::ZstdMedium | CompressionLevel::ZstdHigh | CompressionLevel::ZstdMax => {
                    let compression_level = match compression_format {
                        CompressionLevel::ZstdFast => 1,
                        CompressionLevel::ZstdMedium => 6,
                        CompressionLevel::ZstdHigh => 15,
                        CompressionLevel::ZstdMax => 22,
                        _ => 1,
                    };
                    match zstd::encode_all(&data[..], compression_level) {
                        Ok(compressed_data) => {
                            if compressed_data.len() < data.len() {
                                // Compression successful
                                (compressed_data, Some(original_size), true)
                            } else {
                                // Compression not beneficial, store uncompressed
                                (data, None, false)
                            }
                        }
                        Err(_) => {
                            // Compression failed, store uncompressed  
                            (data, None, false)
                        }
                    }
                }
                CompressionLevel::None => (data, None, false),
            }
        } else {
            // No compression requested
            (data, None, false)
        };
        
        Self {
            event_id: uuid::Uuid::new_v4().to_string(),
            event_type,
            data: final_data,
            timestamp: now,
            serialization_format,
            compression_format,
            is_compressed: final_is_compressed,
            original_size: final_original_size,
            grpc_arrival_time: now,
            parsing_time: 0,
            completion_time: 0,
            client_processing_start: None,
            client_processing_end: None,
        }
    }

    /// 设置性能时间戳
    pub fn set_grpc_arrival_time(&mut self) {
        self.grpc_arrival_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
    }

    pub fn set_parsing_time(&mut self) {
        self.parsing_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
    }

    pub fn set_completion_time(&mut self) {
        self.completion_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;
    }

    pub fn set_client_processing_start(&mut self) {
        self.client_processing_start = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
        );
    }

    pub fn set_client_processing_end(&mut self) {
        self.client_processing_end = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
        );
    }

    /// 获取总处理时间（微秒）
    pub fn get_total_processing_time(&self) -> Option<u64> {
        if let (Some(start), Some(end)) = (self.client_processing_start, self.client_processing_end) {
            Some(end - start)
        } else {
            None
        }
    }

    /// 获取从grpc到达到客户端完成的总时间（微秒）
    pub fn get_end_to_end_latency(&self) -> Option<u64> {
        if let Some(end) = self.client_processing_end {
            Some(end - self.grpc_arrival_time)
        } else {
            None
        }
    }

    /// 获取服务器端处理时间（微秒）
    pub fn get_server_processing_time(&self) -> u64 {
        if self.completion_time > 0 && self.grpc_arrival_time > 0 {
            self.completion_time - self.grpc_arrival_time
        } else {
            0
        }
    }

    /// 获取事件大小（字节）
    pub fn get_data_size(&self) -> usize {
        self.data.len()
    }

    /// 获取原始大小或当前大小
    pub fn get_original_size(&self) -> usize {
        self.original_size.unwrap_or_else(|| self.data.len())
    }

    /// 获取压缩比率（如果有压缩）
    pub fn get_compression_ratio(&self) -> Option<f64> {
        if self.is_compressed && self.original_size.is_some() {
            let original = self.original_size.unwrap() as f64;
            let compressed = self.data.len() as f64;
            Some(compressed / original)
        } else {
            None
        }
    }

    /// 获取解压后的数据
    pub fn get_decompressed_data(&self) -> Result<Vec<u8>, anyhow::Error> {
        if self.is_compressed {
            match self.compression_format {
                CompressionLevel::None => Ok(self.data.clone()),
                CompressionLevel::LZ4Fast | CompressionLevel::LZ4High => {
                    lz4::block::decompress(&self.data, self.original_size.map(|s| s as i32))
                        .map_err(|e| anyhow::anyhow!("LZ4 decompression failed: {}", e))
                }
                CompressionLevel::ZstdFast | CompressionLevel::ZstdMedium | CompressionLevel::ZstdHigh | CompressionLevel::ZstdMax => {
                    zstd::decode_all(&self.data[..])
                        .map_err(|e| anyhow::anyhow!("Zstd decompression failed: {}", e))
                }
            }
        } else {
            Ok(self.data.clone())
        }
    }
    
    /// 获取客户端处理时间（毫秒）
    pub fn client_processing_time_ms(&self) -> Option<u64> {
        if let (Some(start), Some(end)) = (self.client_processing_start, self.client_processing_end) {
            Some((end - start) / 1000) // 转换为毫秒
        } else {
            None
        }
    }
}

/// 客户端接收到的交易事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEvent {
    pub signature: String,
    pub slot: u64,
    pub block_time: Option<i64>,
    pub data: Vec<u8>,  // 序列化的事件数据
}

/// 解析后的事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedEventData {
    pub event_type: EventType,
    pub protocol: String,
    pub data: serde_json::Value,
}

/// 事件统计信息
#[derive(Debug, Clone, Default)]
pub struct EventStats {
    pub total_events: u64,
    pub events_by_type: HashMap<String, u64>,
    pub average_processing_time: f64,
}

/// 事件类型过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTypeFilter {
    pub allowed_types: Vec<EventType>,
    pub blocked_types: Vec<EventType>,
    pub allow_all: bool,
}

impl Default for EventTypeFilter {
    fn default() -> Self {
        Self {
            allowed_types: Vec::new(),
            blocked_types: Vec::new(),
            allow_all: true,
        }
    }
}

impl EventTypeFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn allow_all() -> Self {
        Self {
            allowed_types: Vec::new(),
            blocked_types: Vec::new(),
            allow_all: true,
        }
    }

    pub fn allow_only(types: Vec<EventType>) -> Self {
        Self {
            allowed_types: types,
            blocked_types: Vec::new(),
            allow_all: false,
        }
    }

    pub fn block_types(types: Vec<EventType>) -> Self {
        Self {
            allowed_types: Vec::new(),
            blocked_types: types,
            allow_all: true,
        }
    }

    pub fn is_allowed(&self, event_type: &EventType) -> bool {
        // 首先检查是否在阻止列表中
        if self.blocked_types.contains(event_type) {
            return false;
        }

        // 如果允许所有类型，并且不在阻止列表中，则允许
        if self.allow_all {
            return true;
        }

        // 否则检查是否在允许列表中
        self.allowed_types.contains(event_type)
    }

    pub fn should_pass(&self, event_type: &EventType) -> bool {
        self.is_allowed(event_type)
    }

    pub fn disabled() -> Self {
        Self {
            allowed_types: Vec::new(),
            blocked_types: Vec::new(),
            allow_all: false,
        }
    }

    pub fn add_allowed_type(&mut self, event_type: EventType) {
        if !self.allowed_types.contains(&event_type) {
            self.allowed_types.push(event_type);
        }
        self.allow_all = false;
    }

    pub fn remove_allowed_type(&mut self, event_type: &EventType) {
        self.allowed_types.retain(|t| t != event_type);
    }

    pub fn add_blocked_type(&mut self, event_type: EventType) {
        if !self.blocked_types.contains(&event_type) {
            self.blocked_types.push(event_type);
        }
    }

    pub fn remove_blocked_type(&mut self, event_type: &EventType) {
        self.blocked_types.retain(|t| t != event_type);
    }

    pub fn clear(&mut self) {
        self.allowed_types.clear();
        self.blocked_types.clear();
        self.allow_all = true;
    }

    /// 获取过滤器的统计信息
    pub fn get_stats(&self) -> (usize, usize, bool) {
        (self.allowed_types.len(), self.blocked_types.len(), self.allow_all)
    }

    /// 检查过滤器是否为空（即允许所有）
    pub fn is_empty(&self) -> bool {
        self.allow_all && self.blocked_types.is_empty()
    }

    /// 获取所有允许的事件类型
    pub fn get_allowed_types(&self) -> Vec<EventType> {
        if self.allow_all {
            // 如果允许所有，返回除了blocked之外的所有标准类型
            // 这里可能需要根据实际的EventType enum来定义
            Vec::new() // 暂时返回空，实际应该返回所有标准类型
        } else {
            self.allowed_types.clone()
        }
    }

    /// 获取所有被阻止的事件类型
    pub fn get_blocked_types(&self) -> &Vec<EventType> {
        &self.blocked_types
    }

    /// 合并另一个过滤器
    pub fn merge(&mut self, other: &EventTypeFilter) {
        // 合并允许的类型
        for event_type in &other.allowed_types {
            self.add_allowed_type(event_type.clone());
        }

        // 合并阻止的类型
        for event_type in &other.blocked_types {
            self.add_blocked_type(event_type.clone());
        }

        // 如果任一过滤器不允许所有，则结果也不允许所有
        if !other.allow_all {
            self.allow_all = false;
        }
    }

    /// 创建一个交集过滤器（更严格的过滤）
    pub fn intersect(&self, other: &EventTypeFilter) -> EventTypeFilter {
        let mut result = EventTypeFilter::new();
        
        // 如果任一过滤器不允许所有，结果也不允许所有
        result.allow_all = self.allow_all && other.allow_all;
        
        // 合并阻止列表（任一阻止的都会被阻止）
        result.blocked_types = self.blocked_types.clone();
        for event_type in &other.blocked_types {
            result.add_blocked_type(event_type.clone());
        }
        
        // 如果两个过滤器都有具体的允许列表，取交集
        if !self.allow_all && !other.allow_all {
            result.allowed_types = self.allowed_types
                .iter()
                .filter(|t| other.allowed_types.contains(t))
                .cloned()
                .collect();
        } else if !self.allow_all {
            result.allowed_types = self.allowed_types.clone();
        } else if !other.allow_all {
            result.allowed_types = other.allowed_types.clone();
        }
        
        result
    }
    
    /// 获取过滤器摘要信息
    pub fn get_summary(&self) -> String {
        if self.allow_all && self.blocked_types.is_empty() {
            "Allow all events".to_string()
        } else if !self.allow_all && self.allowed_types.is_empty() {
            "Block all events".to_string()
        } else if self.allow_all {
            format!("Allow all except {} types", self.blocked_types.len())
        } else {
            format!("Allow only {} types", self.allowed_types.len())
        }
    }
}

