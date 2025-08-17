use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// 认证消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthMessage {
    pub auth_token: String,
    pub client_id: String,
    pub timestamp: u64,
}

/// 认证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthResponse {
    Success {
        message: String,
        client_id: String,
        permissions: Vec<String>,
    },
    Failure {
        error: String,
        code: u32,
    },
}

/// 令牌声明
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub user_id: String,
    pub permissions: Vec<String>,
    pub issued_at: u64,
    pub expires_at: u64,
    pub client_info: Option<String>,
}

/// 认证令牌验证器
pub struct AuthTokenValidator {
    secret_key: String,
    predefined_tokens: HashMap<String, Vec<String>>,
}

impl AuthTokenValidator {
    /// 创建新的认证令牌验证器
    pub fn new(secret_key: String) -> Self {
        let mut predefined_tokens = HashMap::new();
        
        // 添加预定义的认证令牌
        predefined_tokens.insert(
            "demo_token_12345".to_string(),
            vec!["read".to_string(), "stream".to_string()]
        );
        predefined_tokens.insert(
            "test_auth_token".to_string(),
            vec!["read".to_string(), "stream".to_string(), "write".to_string()]
        );
        predefined_tokens.insert(
            "admin_super_token_999".to_string(),
            vec!["read".to_string(), "stream".to_string(), "write".to_string(), "admin".to_string()]
        );
        
        Self {
            secret_key,
            predefined_tokens,
        }
    }
    
    /// 验证认证令牌
    pub fn validate_token(&self, token: &str) -> Result<Vec<String>, String> {
        // 检查预定义令牌
        if let Some(permissions) = self.predefined_tokens.get(token) {
            return Ok(permissions.clone());
        }
        
        // 检查JWT格式令牌
        if token.contains('.') {
            return self.validate_jwt_token(token);
        }
        
        // 检查API密钥格式
        if token.starts_with("sk_") {
            return self.validate_api_key(token);
        }
        
        // 检查会话令牌格式
        if token.starts_with("sess_") {
            return self.validate_session_token(token);
        }
        
        Err("无效的认证令牌".to_string())
    }
    
    /// 验证JWT令牌
    fn validate_jwt_token(&self, token: &str) -> Result<Vec<String>, String> {
        // 这里应该实现JWT验证逻辑
        // 为了简化，我们只做基本检查
        if token.split('.').count() == 3 {
            Ok(vec!["read".to_string(), "stream".to_string()])
        } else {
            Err("无效的JWT令牌格式".to_string())
        }
    }
    
    /// 验证API密钥
    fn validate_api_key(&self, token: &str) -> Result<Vec<String>, String> {
        // 检查API密钥格式: sk_userid_timestamp_random
        let parts: Vec<&str> = token.split('_').collect();
        if parts.len() >= 3 && parts[0] == "sk" {
            Ok(vec!["read".to_string(), "stream".to_string()])
        } else {
            Err("无效的API密钥格式".to_string())
        }
    }
    
    /// 验证会话令牌
    fn validate_session_token(&self, token: &str) -> Result<Vec<String>, String> {
        // 检查会话令牌格式: sess_timestamp_userid_random
        let parts: Vec<&str> = token.split('_').collect();
        if parts.len() >= 4 && parts[0] == "sess" {
            // 检查令牌是否过期（1小时）
            if let Ok(timestamp) = parts[1].parse::<u64>() {
                let current_time = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                
                if current_time - timestamp < 3600 {
                    Ok(vec!["read".to_string(), "stream".to_string()])
                } else {
                    Err("会话令牌已过期".to_string())
                }
            } else {
                Err("无效的会话令牌时间戳".to_string())
            }
        } else {
            Err("无效的会话令牌格式".to_string())
        }
    }
}
