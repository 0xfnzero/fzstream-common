use serde::{Serialize, Deserialize};
use std::time::Duration;

/// 序列化协议
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SerializationProtocol {
    JSON,
    Bincode,
    Auto,
}

impl Default for SerializationProtocol {
    fn default() -> Self {
        SerializationProtocol::Auto
    }
}

/// 压缩级别
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionLevel {
    None,
    LZ4Fast,
    LZ4High,
    ZstdFast,
    ZstdMedium,
    ZstdHigh,
    ZstdMax,
}

impl Default for CompressionLevel {
    fn default() -> Self {
        CompressionLevel::LZ4Fast
    }
}

/// 压缩类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CompressionType {
    None,
    LZ4,
    Zstd,
}

impl From<CompressionLevel> for CompressionType {
    fn from(level: CompressionLevel) -> Self {
        match level {
            CompressionLevel::None => CompressionType::None,
            CompressionLevel::LZ4Fast | CompressionLevel::LZ4High => CompressionType::LZ4,
            CompressionLevel::ZstdFast | CompressionLevel::ZstdMedium 
            | CompressionLevel::ZstdHigh | CompressionLevel::ZstdMax => CompressionType::Zstd,
        }
    }
}

/// 性能配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
    pub name: String,
    pub description: String,
    pub serialization: SerializationProtocol,
    pub compression: CompressionLevel,
    pub priority: u8, // 0 = highest priority
    pub use_cases: Vec<String>,
}

/// 自定义设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomSettings {
    pub serialization_protocol: SerializationProtocol,
    pub compression_level: CompressionLevel,
    pub enable_metrics: bool,
    pub adaptive_protocol: bool, // Allow server to suggest protocol switching
    pub latency_target_us: Option<u64>, // Target latency for auto optimization
    pub bandwidth_limit_kbps: Option<u64>, // Bandwidth limit affecting compression choice
    pub message_size_threshold: usize, // Message size threshold for protocol selection
}

/// 自动优化配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoOptimizationConfig {
    pub enabled: bool,
    pub switch_threshold_pct: f64, // Switch protocol when performance difference exceeds this percentage
    pub min_samples: usize, // Minimum samples needed for decision making
    pub evaluation_window_secs: u64, // Evaluation window time
}

/// 客户端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamClientConfig {
    pub server_address: String,
    pub server_name: String,
    pub auth_token: Option<String>,
    pub protocol: SerializationProtocol,
    pub compression: CompressionLevel,
    pub auto_reconnect: bool,
    pub reconnect_interval: Duration,
    pub max_reconnect_attempts: u32,
    pub connection_timeout: Duration,
    pub keep_alive_interval: Duration,
}

/// 高级客户端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientConfig {
    pub client_id: String,
    pub current_profile: String,
    pub custom_settings: CustomSettings,
    pub available_profiles: std::collections::HashMap<String, PerformanceProfile>,
    pub auto_optimization: AutoOptimizationConfig,
}

impl Default for StreamClientConfig {
    fn default() -> Self {
        Self {
            server_address: "127.0.0.1:8080".to_string(),
            server_name: "localhost".to_string(),
            auth_token: None,
            protocol: SerializationProtocol::Auto,
            compression: CompressionLevel::LZ4Fast,
            auto_reconnect: true,
            reconnect_interval: Duration::from_secs(5),
            max_reconnect_attempts: 10,
            connection_timeout: Duration::from_secs(10),
            keep_alive_interval: Duration::from_secs(30),
        }
    }
}

/// 服务器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub num_workers: Option<usize>,
    pub auth_secret_key: Option<String>,
    pub enable_stats_reporter: bool,
    pub heartbeat_interval_secs: u64,
    pub idle_timeout_secs: u64,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: "127.0.0.1:8080".to_string(),
            num_workers: None,
            auth_secret_key: None,
            enable_stats_reporter: true,
            heartbeat_interval_secs: 30,
            idle_timeout_secs: 300,
        }
    }
}

/// 配置构建器
pub struct ConfigBuilder {
    config: StreamClientConfig,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            config: StreamClientConfig::default(),
        }
    }
    
    pub fn server_address(mut self, address: String) -> Self {
        self.config.server_address = address;
        self
    }
    
    pub fn server_name(mut self, name: String) -> Self {
        self.config.server_name = name;
        self
    }
    
    pub fn auth_token(mut self, token: String) -> Self {
        self.config.auth_token = Some(token);
        self
    }
    
    pub fn protocol(mut self, protocol: SerializationProtocol) -> Self {
        self.config.protocol = protocol;
        self
    }
    
    pub fn compression(mut self, compression: CompressionLevel) -> Self {
        self.config.compression = compression;
        self
    }
    
    pub fn auto_reconnect(mut self, enabled: bool) -> Self {
        self.config.auto_reconnect = enabled;
        self
    }
    
    pub fn reconnect_interval(mut self, interval: Duration) -> Self {
        self.config.reconnect_interval = interval;
        self
    }
    
    pub fn max_reconnect_attempts(mut self, attempts: u32) -> Self {
        self.config.max_reconnect_attempts = attempts;
        self
    }
    
    pub fn connection_timeout(mut self, timeout: Duration) -> Self {
        self.config.connection_timeout = timeout;
        self
    }
    
    pub fn keep_alive_interval(mut self, interval: Duration) -> Self {
        self.config.keep_alive_interval = interval;
        self
    }
    
    pub fn build(self) -> StreamClientConfig {
        self.config
    }
}
