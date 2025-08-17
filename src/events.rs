use serde::{Serialize, Deserialize};
use std::collections::HashMap;

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

/// QUIC服务器和客户端之间传输的事件消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMessage {
    pub event_id: String,
    pub event_type: EventType,
    pub data: Vec<u8>,  // bincode序列化的数据
    pub timestamp: u64,
}

/// Solana事件包装器（用于bincode序列化）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolanaEventWrapper {
    pub event_type: EventType,
    pub event_id: String,
    pub timestamp: u64,
    pub source: String,
    pub data: Vec<u8>, // Bincode序列化的事件数据
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
