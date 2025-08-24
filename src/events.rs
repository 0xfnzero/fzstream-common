use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use crate::{CompressionLevel, SerializationProtocol};

/// 事件类型枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EventType {
    // 区块事件
    BlockMeta,
    
    // Bonk协议事件
    BonkPoolCreate,
    BonkTrade,
    BonkMigrateToAmm,
    BonkMigrateToCpswap,
    
    // PumpFun协议事件
    PumpFunTrade,
    PumpFunMigrate,
    PumpFunCreate,
    
    // PumpSwap协议事件
    PumpSwapBuy,
    PumpSwapSell,
    PumpSwapCreate,
    PumpSwapDeposit,
    PumpSwapWithdraw,
    
    // Raydium CPMM事件
    RaydiumCpmmSwap,
    RaydiumCpmmDeposit,
    RaydiumCpmmInitialize,
    RaydiumCpmmWithdraw,
    
    // Raydium CLMM事件
    RaydiumClmmSwap,
    RaydiumClmmSwapV2,
    RaydiumClmmClosePosition,
    RaydiumClmmDecreaseLiquidityV2,
    RaydiumClmmCreatePool,
    RaydiumClmmIncreaseLiquidityV2,
    RaydiumClmmOpenPositionWithToken22Nft,
    RaydiumClmmOpenPositionV2,
    
    // Raydium AMM V4事件
    RaydiumAmmV4Swap,
    RaydiumAmmV4Deposit,
    RaydiumAmmV4Initialize,
    RaydiumAmmV4Withdraw,
    RaydiumAmmV4WithdrawPnl,
    
    // 账户状态事件
    BonkPoolStateAccount,
    BonkGlobalConfigAccount,
    BonkPlatformConfigAccount,
    PumpSwapGlobalConfigAccount,
    PumpSwapPoolAccount,
    PumpFunBondingCurveAccount,
    PumpFunGlobalAccount,
    RaydiumAmmV4InfoAccount,
    RaydiumClmmConfigAccount,
    RaydiumClmmPoolStateAccount,
    RaydiumClmmTickArrayAccount,
    RaydiumCpmmConfigAccount,
    RaydiumCpmmPoolStateAccount,
    
    // 自定义事件
    Custom(String),
}

impl EventType {
    /// 从字符串创建事件类型
    pub fn from_str(s: &str) -> Self {
        match s {
            "BlockMeta" => EventType::BlockMeta,
            "BonkPoolCreate" => EventType::BonkPoolCreate,
            "BonkTrade" => EventType::BonkTrade,
            "BonkMigrateToAmm" => EventType::BonkMigrateToAmm,
            "BonkMigrateToCpswap" => EventType::BonkMigrateToCpswap,
            "PumpFunTrade" => EventType::PumpFunTrade,
            "PumpFunMigrate" => EventType::PumpFunMigrate,
            "PumpFunCreate" => EventType::PumpFunCreate,
            "PumpSwapBuy" => EventType::PumpSwapBuy,
            "PumpSwapSell" => EventType::PumpSwapSell,
            "PumpSwapCreate" => EventType::PumpSwapCreate,
            "PumpSwapDeposit" => EventType::PumpSwapDeposit,
            "PumpSwapWithdraw" => EventType::PumpSwapWithdraw,
            "RaydiumCpmmSwap" => EventType::RaydiumCpmmSwap,
            "RaydiumCpmmDeposit" => EventType::RaydiumCpmmDeposit,
            "RaydiumCpmmInitialize" => EventType::RaydiumCpmmInitialize,
            "RaydiumCpmmWithdraw" => EventType::RaydiumCpmmWithdraw,
            "RaydiumClmmSwap" => EventType::RaydiumClmmSwap,
            "RaydiumClmmSwapV2" => EventType::RaydiumClmmSwapV2,
            "RaydiumClmmClosePosition" => EventType::RaydiumClmmClosePosition,
            "RaydiumClmmDecreaseLiquidityV2" => EventType::RaydiumClmmDecreaseLiquidityV2,
            "RaydiumClmmCreatePool" => EventType::RaydiumClmmCreatePool,
            "RaydiumClmmIncreaseLiquidityV2" => EventType::RaydiumClmmIncreaseLiquidityV2,
            "RaydiumClmmOpenPositionWithToken22Nft" => EventType::RaydiumClmmOpenPositionWithToken22Nft,
            "RaydiumClmmOpenPositionV2" => EventType::RaydiumClmmOpenPositionV2,
            "RaydiumAmmV4Swap" => EventType::RaydiumAmmV4Swap,
            "RaydiumAmmV4Deposit" => EventType::RaydiumAmmV4Deposit,
            "RaydiumAmmV4Initialize" => EventType::RaydiumAmmV4Initialize,
            "RaydiumAmmV4Withdraw" => EventType::RaydiumAmmV4Withdraw,
            "RaydiumAmmV4WithdrawPnl" => EventType::RaydiumAmmV4WithdrawPnl,
            "BonkPoolStateAccount" => EventType::BonkPoolStateAccount,
            "BonkGlobalConfigAccount" => EventType::BonkGlobalConfigAccount,
            "BonkPlatformConfigAccount" => EventType::BonkPlatformConfigAccount,
            "PumpSwapGlobalConfigAccount" => EventType::PumpSwapGlobalConfigAccount,
            "PumpSwapPoolAccount" => EventType::PumpSwapPoolAccount,
            "PumpFunBondingCurveAccount" => EventType::PumpFunBondingCurveAccount,
            "PumpFunGlobalAccount" => EventType::PumpFunGlobalAccount,
            "RaydiumAmmV4InfoAccount" => EventType::RaydiumAmmV4InfoAccount,
            "RaydiumClmmConfigAccount" => EventType::RaydiumClmmConfigAccount,
            "RaydiumClmmPoolStateAccount" => EventType::RaydiumClmmPoolStateAccount,
            "RaydiumClmmTickArrayAccount" => EventType::RaydiumClmmTickArrayAccount,
            "RaydiumCpmmConfigAccount" => EventType::RaydiumCpmmConfigAccount,
            "RaydiumCpmmPoolStateAccount" => EventType::RaydiumCpmmPoolStateAccount,
            _ => EventType::Custom(s.to_string()),
        }
    }
    
    /// 转换为字符串
    pub fn to_string(&self) -> String {
        match self {
            EventType::BlockMeta => "BlockMeta".to_string(),
            EventType::BonkPoolCreate => "BonkPoolCreate".to_string(),
            EventType::BonkTrade => "BonkTrade".to_string(),
            EventType::BonkMigrateToAmm => "BonkMigrateToAmm".to_string(),
            EventType::BonkMigrateToCpswap => "BonkMigrateToCpswap".to_string(),
            EventType::PumpFunTrade => "PumpFunTrade".to_string(),
            EventType::PumpFunMigrate => "PumpFunMigrate".to_string(),
            EventType::PumpFunCreate => "PumpFunCreate".to_string(),
            EventType::PumpSwapBuy => "PumpSwapBuy".to_string(),
            EventType::PumpSwapSell => "PumpSwapSell".to_string(),
            EventType::PumpSwapCreate => "PumpSwapCreate".to_string(),
            EventType::PumpSwapDeposit => "PumpSwapDeposit".to_string(),
            EventType::PumpSwapWithdraw => "PumpSwapWithdraw".to_string(),
            EventType::RaydiumCpmmSwap => "RaydiumCpmmSwap".to_string(),
            EventType::RaydiumCpmmDeposit => "RaydiumCpmmDeposit".to_string(),
            EventType::RaydiumCpmmInitialize => "RaydiumCpmmInitialize".to_string(),
            EventType::RaydiumCpmmWithdraw => "RaydiumCpmmWithdraw".to_string(),
            EventType::RaydiumClmmSwap => "RaydiumClmmSwap".to_string(),
            EventType::RaydiumClmmSwapV2 => "RaydiumClmmSwapV2".to_string(),
            EventType::RaydiumClmmClosePosition => "RaydiumClmmClosePosition".to_string(),
            EventType::RaydiumClmmDecreaseLiquidityV2 => "RaydiumClmmDecreaseLiquidityV2".to_string(),
            EventType::RaydiumClmmCreatePool => "RaydiumClmmCreatePool".to_string(),
            EventType::RaydiumClmmIncreaseLiquidityV2 => "RaydiumClmmIncreaseLiquidityV2".to_string(),
            EventType::RaydiumClmmOpenPositionWithToken22Nft => "RaydiumClmmOpenPositionWithToken22Nft".to_string(),
            EventType::RaydiumClmmOpenPositionV2 => "RaydiumClmmOpenPositionV2".to_string(),
            EventType::RaydiumAmmV4Swap => "RaydiumAmmV4Swap".to_string(),
            EventType::RaydiumAmmV4Deposit => "RaydiumAmmV4Deposit".to_string(),
            EventType::RaydiumAmmV4Initialize => "RaydiumAmmV4Initialize".to_string(),
            EventType::RaydiumAmmV4Withdraw => "RaydiumAmmV4Withdraw".to_string(),
            EventType::RaydiumAmmV4WithdrawPnl => "RaydiumAmmV4WithdrawPnl".to_string(),
            EventType::BonkPoolStateAccount => "BonkPoolStateAccount".to_string(),
            EventType::BonkGlobalConfigAccount => "BonkGlobalConfigAccount".to_string(),
            EventType::BonkPlatformConfigAccount => "BonkPlatformConfigAccount".to_string(),
            EventType::PumpSwapGlobalConfigAccount => "PumpSwapGlobalConfigAccount".to_string(),
            EventType::PumpSwapPoolAccount => "PumpSwapPoolAccount".to_string(),
            EventType::PumpFunBondingCurveAccount => "PumpFunBondingCurveAccount".to_string(),
            EventType::PumpFunGlobalAccount => "PumpFunGlobalAccount".to_string(),
            EventType::RaydiumAmmV4InfoAccount => "RaydiumAmmV4InfoAccount".to_string(),
            EventType::RaydiumClmmConfigAccount => "RaydiumClmmConfigAccount".to_string(),
            EventType::RaydiumClmmPoolStateAccount => "RaydiumClmmPoolStateAccount".to_string(),
            EventType::RaydiumClmmTickArrayAccount => "RaydiumClmmTickArrayAccount".to_string(),
            EventType::RaydiumCpmmConfigAccount => "RaydiumCpmmConfigAccount".to_string(),
            EventType::RaydiumCpmmPoolStateAccount => "RaydiumCpmmPoolStateAccount".to_string(),
            EventType::Custom(s) => s.clone(),
        }
    }
    
    /// 检查是否为交易事件
    pub fn is_transaction(&self) -> bool {
        matches!(self,
            EventType::BonkTrade |
            EventType::PumpFunTrade |
            EventType::PumpSwapBuy |
            EventType::PumpSwapSell |
            EventType::RaydiumCpmmSwap |
            EventType::RaydiumClmmSwap |
            EventType::RaydiumClmmSwapV2 |
            EventType::RaydiumAmmV4Swap
        )
    }
    
    /// 检查是否为池创建事件
    pub fn is_pool_create(&self) -> bool {
        matches!(self,
            EventType::BonkPoolCreate |
            EventType::PumpSwapCreate |
            EventType::RaydiumClmmCreatePool |
            EventType::RaydiumCpmmInitialize |
            EventType::RaydiumAmmV4Initialize
        )
    }
    
    /// 检查是否为账户状态事件
    pub fn is_account_event(&self) -> bool {
        matches!(self,
            EventType::BonkPoolStateAccount |
            EventType::BonkGlobalConfigAccount |
            EventType::BonkPlatformConfigAccount |
            EventType::PumpSwapGlobalConfigAccount |
            EventType::PumpSwapPoolAccount |
            EventType::PumpFunBondingCurveAccount |
            EventType::PumpFunGlobalAccount |
            EventType::RaydiumAmmV4InfoAccount |
            EventType::RaydiumClmmConfigAccount |
            EventType::RaydiumClmmPoolStateAccount |
            EventType::RaydiumClmmTickArrayAccount |
            EventType::RaydiumCpmmConfigAccount |
            EventType::RaydiumCpmmPoolStateAccount
        )
    }
}

/// 事件优先级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for EventPriority {
    fn default() -> Self {
        EventPriority::Normal
    }
}

/// 事件元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMetadata {
    pub block_time: Option<u64>,
    pub slot: Option<u64>,
    pub signature: Option<String>,
    pub source: Option<String>,
    pub priority: Option<String>,
    pub additional_fields: HashMap<String, serde_json::Value>,
}

impl Default for EventMetadata {
    fn default() -> Self {
        Self {
            block_time: None,
            slot: None,
            signature: None,
            source: None,
            priority: None,
            additional_fields: HashMap::new(),
        }
    }
}

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
    /// 创建新的事件消息（服务器端使用）
    pub fn new(
        event_id: String,
        event_type: EventType,
        raw_data: Vec<u8>,
        serialization_format: SerializationProtocol,
        compression_format: CompressionLevel,
        grpc_arrival_time: u64,
        parsing_time: u64,
        completion_time: u64,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let original_size = raw_data.len();
        
        // 应用压缩（如果启用）
        let (data, is_compressed, final_original_size) = if matches!(compression_format, CompressionLevel::None) {
            (raw_data, false, None)
        } else {
            match crate::compression::compress_data(&raw_data, compression_format.clone()) {
                Ok(compressed) => {
                    if compressed.len() < raw_data.len() {
                        // 压缩有效果，使用压缩版本
                        (compressed, true, Some(original_size))
                    } else {
                        // 压缩没有效果，使用原始数据
                        (raw_data, false, None)
                    }
                },
                Err(_) => {
                    // 压缩失败，使用原始数据
                    (raw_data, false, None)
                }
            }
        };

        Ok(Self {
            event_id,
            event_type,
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            serialization_format,
            compression_format,
            is_compressed,
            original_size: final_original_size,
            grpc_arrival_time,
            parsing_time,
            completion_time,
            client_processing_start: None,
            client_processing_end: None,
        })
    }

    /// 获取解压缩后的数据（客户端使用）
    pub fn get_decompressed_data(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if self.is_compressed {
            crate::compression::decompress_data(&self.data, self.compression_format.clone())
                .map_err(|e| e.into())
        } else {
            Ok(self.data.clone())
        }
    }
    
    /// 检查压缩效果
    pub fn compression_ratio(&self) -> Option<f64> {
        if let Some(original_size) = self.original_size {
            if original_size > 0 {
                Some(self.data.len() as f64 / original_size as f64)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// 设置客户端处理开始时间
    pub fn set_client_processing_start(&mut self) {
        self.client_processing_start = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
        );
    }

    /// 设置客户端处理结束时间
    pub fn set_client_processing_end(&mut self) {
        self.client_processing_end = Some(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64
        );
    }

    /// 计算服务器端总耗时（毫秒）
    pub fn server_total_time_ms(&self) -> u64 {
        if self.completion_time > 0 && self.grpc_arrival_time > 0 {
            self.completion_time - self.grpc_arrival_time
        } else {
            0
        }
    }

    /// 计算客户端处理耗时（毫秒）
    pub fn client_processing_time_ms(&self) -> Option<u64> {
        if let (Some(start), Some(end)) = (self.client_processing_start, self.client_processing_end) {
            Some(end - start)
        } else {
            None
        }
    }

    /// 计算端到端总耗时（毫秒）
    pub fn end_to_end_time_ms(&self) -> Option<u64> {
        if let Some(client_time) = self.client_processing_time_ms() {
            Some(self.server_total_time_ms() + client_time)
        } else {
            None
        }
    }

    /// 获取详细的时间分析
    pub fn get_timing_analysis(&self) -> String {
        let server_time = self.server_total_time_ms();
        let client_time = self.client_processing_time_ms().unwrap_or(0);
        let end_to_end = self.end_to_end_time_ms().unwrap_or(0);
        
        format!(
            "Timing Analysis for {}:\n\
             • GRPC Arrival: {}ms\n\
             • Parsing Time: {}ms\n\
             • Completion Time: {}ms\n\
             • Server Total: {}ms\n\
             • Client Processing: {}ms\n\
             • End-to-End: {}ms",
            self.event_id,
            self.grpc_arrival_time,
            self.parsing_time,
            self.completion_time,
            server_time,
            client_time,
            end_to_end
        )
    }
}

/// 客户端接收到的交易事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionEvent {
    pub event_id: String,
    pub event_type: EventType,
    pub timestamp: u64,
    pub data: serde_json::Value,
    pub metadata: Option<EventMetadata>,
}

/// 解析后的事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedEvent {
    pub event_id: String,
    pub event_type: EventType,
    pub timestamp: u64,
    pub data: serde_json::Value,
    pub metadata: EventMetadata,
}

/// 事件统计信息
#[derive(Debug, Clone, Default)]
pub struct EventStats {
    pub total_events: u64,
    pub events_by_type: HashMap<EventType, u64>,
    pub last_event_time: Option<u64>,
}

/// 事件类型过滤器
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EventTypeFilter {
    /// 包含的事件类型列表 (如果为空，则包含所有事件)
    pub include: Vec<EventType>,
    /// 排除的事件类型列表 (优先级高于include)
    pub exclude: Vec<EventType>,
    /// 是否启用过滤器 (如果为false，则忽略所有过滤规则)
    pub enabled: bool,
}

impl Default for EventTypeFilter {
    fn default() -> Self {
        Self {
            include: Vec::new(),
            exclude: Vec::new(),
            enabled: true,
        }
    }
}

impl EventTypeFilter {
    /// 创建新的过滤器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 创建只包含指定事件类型的过滤器
    pub fn include_only(event_types: Vec<EventType>) -> Self {
        Self {
            include: event_types,
            exclude: Vec::new(),
            enabled: true,
        }
    }
    
    /// 创建排除指定事件类型的过滤器
    pub fn exclude_only(event_types: Vec<EventType>) -> Self {
        Self {
            include: Vec::new(),
            exclude: event_types,
            enabled: true,
        }
    }
    
    /// 创建包含和排除事件类型的复合过滤器
    pub fn with_include_exclude(include: Vec<EventType>, exclude: Vec<EventType>) -> Self {
        Self {
            include,
            exclude,
            enabled: true,
        }
    }
    
    /// 创建禁用的过滤器（允许所有事件通过）
    pub fn disabled() -> Self {
        Self {
            include: Vec::new(),
            exclude: Vec::new(),
            enabled: false,
        }
    }
    
    /// 检查事件类型是否应该被过滤掉
    pub fn should_filter_out(&self, event_type: &EventType) -> bool {
        if !self.enabled {
            return false; // 过滤器未启用，不过滤任何事件
        }
        
        // 优先检查排除列表
        if self.exclude.contains(event_type) {
            return true; // 在排除列表中，应该过滤掉
        }
        
        // 如果include列表不为空，检查是否在包含列表中
        if !self.include.is_empty() {
            return !self.include.contains(event_type); // 不在包含列表中，应该过滤掉
        }
        
        // include列表为空且不在exclude列表中，不过滤
        false
    }
    
    /// 检查事件类型是否应该通过过滤器
    pub fn should_pass(&self, event_type: &EventType) -> bool {
        !self.should_filter_out(event_type)
    }
    
    /// 添加包含的事件类型
    pub fn add_include(&mut self, event_type: EventType) -> &mut Self {
        if !self.include.contains(&event_type) {
            self.include.push(event_type);
        }
        self
    }
    
    /// 添加排除的事件类型
    pub fn add_exclude(&mut self, event_type: EventType) -> &mut Self {
        if !self.exclude.contains(&event_type) {
            self.exclude.push(event_type);
        }
        self
    }
    
    /// 移除包含的事件类型
    pub fn remove_include(&mut self, event_type: &EventType) -> &mut Self {
        self.include.retain(|t| t != event_type);
        self
    }
    
    /// 移除排除的事件类型
    pub fn remove_exclude(&mut self, event_type: &EventType) -> &mut Self {
        self.exclude.retain(|t| t != event_type);
        self
    }
    
    /// 清空所有过滤规则
    pub fn clear(&mut self) -> &mut Self {
        self.include.clear();
        self.exclude.clear();
        self
    }
    
    /// 启用过滤器
    pub fn enable(&mut self) -> &mut Self {
        self.enabled = true;
        self
    }
    
    /// 禁用过滤器
    pub fn disable(&mut self) -> &mut Self {
        self.enabled = false;
        self
    }
    
    /// 获取过滤器统计信息
    pub fn get_summary(&self) -> String {
        if !self.enabled {
            return "Filter: DISABLED (all events pass)".to_string();
        }
        
        let include_summary = if self.include.is_empty() {
            "all".to_string()
        } else {
            format!("{} types", self.include.len())
        };
        
        let exclude_summary = if self.exclude.is_empty() {
            "none".to_string()
        } else {
            format!("{} types", self.exclude.len())
        };
        
        format!("Filter: Include={}, Exclude={}", include_summary, exclude_summary)
    }
}
