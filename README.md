# FZ Stream Common Library

这个库包含了 `fz-stream-server` 和 `fz-stream-client` 之间共享的数据结构和类型定义。

## 概述

`fz-stream-common` 是一个公共库，用于确保服务器和客户端之间使用相同的数据结构和类型定义，避免代码重复和不一致。

## 主要功能

### 1. 事件类型定义 (`events.rs`)

#### EventType 枚举
定义了所有支持的Solana事件类型：

```rust
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
    
    // Raydium事件
    RaydiumCpmmSwap,
    RaydiumClmmSwap,
    RaydiumAmmV4Swap,
    // ... 更多事件类型
}
```

#### 核心数据结构

- **EventMessage**: QUIC服务器和客户端之间传输的事件消息
- **SolanaEventWrapper**: Solana事件包装器（用于bincode序列化）
- **TransactionEvent**: 客户端接收到的交易事件
- **ParsedEvent**: 解析后的事件数据
- **EventMetadata**: 事件元数据
- **EventStats**: 事件统计信息

### 2. 认证系统 (`auth.rs`)

#### 认证类型
- **AuthMessage**: 认证消息
- **AuthResponse**: 认证响应
- **TokenClaims**: 令牌声明
- **AuthTokenValidator**: 认证令牌验证器

#### 支持的令牌类型
- 预定义令牌
- JWT令牌
- API密钥
- 会话令牌

### 3. 配置系统 (`config.rs`)

#### 序列化协议
```rust
pub enum SerializationProtocol {
    JSON,
    Bincode,
    Auto,
}
```

#### 压缩级别
```rust
pub enum CompressionLevel {
    None,
    LZ4Fast,
    LZ4High,
    ZstdFast,
    ZstdMedium,
    ZstdHigh,
    ZstdMax,
}
```

#### 配置结构
- **StreamClientConfig**: 客户端配置
- **ServerConfig**: 服务器配置
- **ConfigBuilder**: 配置构建器

### 4. 压缩系统 (`compression.rs`)

#### 压缩类型
```rust
pub enum CompressionType {
    None,
    LZ4,
    Zstd,
}
```

#### 压缩统计
- **CompressionStats**: 压缩统计信息
- **CompressionInfo**: 压缩算法信息
- **CompressionConfig**: 压缩配置

## 使用方法

### 在服务器中使用

```rust
use fzstream_common::{
    EventMessage, EventType, EventMetadata, 
    AuthTokenValidator, AuthMessage, AuthResponse
};

// 创建事件消息
let event = EventMessage {
    event_type: "PumpFunTrade".to_string(),
    event_id: "trade_123".to_string(),
    data: bincode::serialize(&trade_data)?,
    timestamp: 1234567890,
};

// 验证认证令牌
let validator = AuthTokenValidator::new("secret_key".to_string());
let permissions = validator.validate_token("demo_token_12345")?;
```

### 在客户端中使用

```rust
use fzstream_common::{
    TransactionEvent, EventType, EventMetadata,
    SerializationProtocol, CompressionLevel
};

// 处理接收到的交易事件
let event = TransactionEvent {
    event_id: "trade_123".to_string(),
    event_type: "PumpFunTrade".to_string(),
    timestamp: 1234567890,
    data: serde_json::json!({ "user": "abc", "amount": 1000 }),
    metadata: Some(EventMetadata::default()),
};

// 使用事件类型枚举
let event_type = EventType::from_str(&event.event_type);
if event_type.is_transaction() {
    println!("这是一个交易事件");
}
```

## 事件类型分类

### 交易事件
```rust
// 检查是否为交易事件
if event_type.is_transaction() {
    // 处理交易逻辑
}
```

### 池创建事件
```rust
// 检查是否为池创建事件
if event_type.is_pool_create() {
    // 处理池创建逻辑
}
```

### 账户状态事件
```rust
// 检查是否为账户状态事件
if event_type.is_account_event() {
    // 处理账户状态变化
}
```

## 性能特性

1. **类型安全**: 所有事件都有明确的类型定义
2. **序列化优化**: 支持高效的bincode序列化
3. **内存效率**: 使用Arc和智能指针优化内存使用
4. **并发安全**: 所有数据结构都是线程安全的

## 扩展性

### 添加新的事件类型

1. 在 `EventType` 枚举中添加新变体
2. 在 `from_str` 和 `to_string` 方法中添加映射
3. 根据需要添加新的分类方法

### 添加新的配置选项

1. 在相应的配置结构体中添加字段
2. 更新默认实现
3. 在构建器中添加相应的方法

## 版本兼容性

- 所有公共API都使用语义化版本控制
- 向后兼容的更改只增加补丁版本
- 破坏性更改会增加主版本号

## 贡献指南

1. 确保所有更改都经过测试
2. 更新相关文档
3. 遵循现有的代码风格
4. 添加适当的错误处理

## 许可证

MIT License
