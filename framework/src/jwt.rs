use std::sync::OnceLock;

use common::{AppError, AppResult};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use salvo::{Request, http::header::AUTHORIZATION};
use serde::{Deserialize, Serialize};

use crate::config::JWT;
#[derive(Debug, Clone)]
pub struct JwtConfig {
    secret: String,
    acc_exp: u64,
    ref_exp: u64,
    algorithm: Algorithm,
    issuer: String,
}
impl JwtConfig {
    pub fn new(
        secret: String,
        acc_expiration_hour: u8,
        ref_expiration_hour: u8,
        issuer: String,
    ) -> Self {
        Self {
            secret,
            acc_exp: acc_expiration_hour as u64 * 60,
            ref_exp: ref_expiration_hour as u64 * 60,
            algorithm: Algorithm::HS256,
            issuer,
        }
    }
    /// 生成验证配置
    fn validation(&self) -> Validation {
        let mut validation = Validation::new(self.algorithm);
        validation.set_issuer(&[&self.issuer]);
        validation
    }
}

impl From<JWT> for JwtConfig {
    fn from(setting: JWT) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp() as u64;
        Self {
            secret: setting.secret,
            acc_exp: now + (setting.acc_expiration_hour as u64 * 60),
            ref_exp: now + (setting.ref_expiration_hour as u64 * 60),
            algorithm: Algorithm::HS256,
            issuer: setting.issuer,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TokenType {
    Access,  // 访问令牌（短期）
    Refresh, // 刷新令牌（长期）
}

// 1. 定义JWT载荷结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// 用户名/用户ID
    pub sub: CustomClaims,
    /// 验证过期时间（Unix时间戳，秒）
    pub exp: u64,
    /// 刷新过期时间（Unix时间戳，秒）
    /// 签发时间
    pub iat: u64,
    /// 发行人
    pub iss: String,
    ///token 类型
    pub token_type: TokenType,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomClaims {
    pub user_id: u64,
    pub user_name: String,
}

impl CustomClaims {
    pub fn new(user_id: u64, user_name: String) -> Self {
        Self { user_id, user_name }
    }
}

impl Claims {
    /// 创建新的默认声明
    pub fn new(sub: CustomClaims, config: &JwtConfig, token_type: TokenType) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp() as u64;
        let iss = config.issuer.clone();
        match token_type {
            TokenType::Access => Self {
                sub,
                exp: config.acc_exp,
                iat: now,
                iss,
                token_type,
            },
            TokenType::Refresh => Self {
                sub,
                exp: config.ref_exp,
                iat: now,
                iss,
                token_type,
            },
        }
    }
}

/// JWT工具类
pub struct JwtAuthUtil {
    config: JwtConfig,
}

impl JwtAuthUtil {
    /// 创建新的JWT工具实例
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    /// 生成令牌
    pub fn generate_token(
        &self,
        subject: CustomClaims,
        token_type: TokenType,
    ) -> AppResult<String> {
        let claims = Claims::new(subject, &self.config, token_type);
        let token = encode(
            &Header::new(self.config.algorithm),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_bytes()),
        )?;
        Ok(token)
    }

    /// 验证令牌
    pub fn verify_acc_token(&self, token: &str) -> AppResult<Claims> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_bytes()),
            &self.config.validation(),
        )
        .map(|data| data.claims)
        {
            Ok(claims) => Ok(claims),
            Err(_) => Err(AppError::AccTokenInvalid),
        }
    }

    /// 验证令牌
    pub fn verify_ref_token(&self, token: &str) -> AppResult<Claims> {
        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_bytes()),
            &self.config.validation(),
        )
        .map(|data| data.claims)
        {
            Ok(claims) => Ok(claims),
            Err(_) => Err(AppError::RefTokenInvalid),
        }
    }

    pub fn extract_token(&self, req: &Request) -> AppResult<String> {
        const BEARER_PREFIX: &str = "Bearer ";
        let token = req
            .headers()
            .get(AUTHORIZATION)
            .ok_or(AppError::AccTokenInvalid)?
            .to_str()
            .map_err(|e| AppError::Other(e.to_string()))?
            .strip_prefix(BEARER_PREFIX)
            .ok_or(AppError::AccTokenInvalid)?;
        if token.is_empty() {
            Err(AppError::AccTokenInvalid)
        } else {
            Ok(token.to_string())
        }
    }
}

pub static JWTONCELOCK: OnceLock<JwtAuthUtil> = OnceLock::new();
pub const CLAIMS: &str = "claims";
