use crate::config::JWT;
use common::{AppError, AppResult};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use salvo::{Request, http::header::AUTHORIZATION};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

static JWTONCELOCK: OnceLock<JwtAuthUtil> = OnceLock::new();
pub const CLAIMS: &str = "claims";

pub struct JWTTool;
impl JWTTool {
    pub fn init(jwt_config: JwtConfig) {
        JWTONCELOCK.get_or_init(|| JwtAuthUtil::new(jwt_config));
    }

    pub fn get() -> AppResult<&'static JwtAuthUtil> {
        JWTONCELOCK
            .get()
            .ok_or(AppError::Other("JWT 工具初始化失败".to_string()))
    }
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    //密钥
    secret: String,
    //令牌时效时间
    acc_exp: u64,
    //刷新令牌时效时间
    ref_exp: u64,
    //加密算法
    algorithm: Algorithm,
    //签发者
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

impl From<&JWT> for JwtConfig {
    fn from(setting: &JWT) -> Self {
        Self {
            secret: setting.secret.clone(),
            acc_exp: setting.acc_expiration_hour as u64 * 60,
            ref_exp: setting.ref_expiration_hour as u64 * 60,
            algorithm: Algorithm::HS256,
            issuer: setting.issuer.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")] // 关键注解
pub enum TokenType {
    Access,  // 访问令牌（短期）
    Refresh, // 刷新令牌（长期）
}

// 1. 定义JWT载荷结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// 用户账号
    pub sub: String,
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

impl Claims {
    /// 创建新的默认声明
    pub fn new(sub: String, config: &JwtConfig, token_type: TokenType) -> Self {
        let now = time::OffsetDateTime::now_utc().unix_timestamp() as u64;
        let iss = config.issuer.clone();
        match token_type {
            TokenType::Access => Self {
                sub,
                exp: now + config.acc_exp,
                iat: now,
                iss,
                token_type,
            },
            TokenType::Refresh => Self {
                sub,
                exp: now + config.ref_exp,
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
    pub fn generate_token(&self, subject: String, token_type: TokenType) -> AppResult<String> {
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
        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_bytes()),
            &self.config.validation(),
        )
        .map(|data| data.claims)?;
        Ok(claims)
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

#[cfg(test)]
mod test {
    use common::AppResult;

    use crate::{
        Setting,
        jwt::{JWTTool, TokenType},
    };

    #[test]
    fn jwt_test() -> AppResult<()> {
        // Initialize config subsystem
        let setting = Setting::init()?;
        // Initialize jwt auth util
        JWTTool::init((&setting.jwt).into());
        let jwt_auth_util = JWTTool::get()?;
        let acc_token = jwt_auth_util.generate_token("wdc".to_string(), TokenType::Access)?;
        println!("{acc_token}");
        let claims = jwt_auth_util.verify_acc_token(&acc_token)?;
        println!("{claims:?}");
        Ok(())
    }
}
