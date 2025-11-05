use std::{sync::OnceLock, time::Duration};

use common::{AppError, AppResult};
use moka::future::Cache;
use salvo::oapi::ToSchema;
use serde::{Deserialize, Serialize};
use user_agent_parser::UserAgentParser;

#[derive(Debug, Serialize)]
pub struct CaptchaVO {
    pub id: String,
    pub img: String,
}

#[derive(Debug, Deserialize)]
pub struct CaptchaDTO {
    pub uuid: String,
    pub code: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenVO {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    pub username: String,
    pub password: String,
    pub captcha: CaptchaDTO,
}

pub static CACHE: OnceLock<Cache<String, String>> = OnceLock::new();
/// 验证码缓存
#[derive(Debug, Clone, Copy)]
pub struct CapCache;
impl CapCache {
    pub fn init_cache() -> &'static Cache<String, String> {
        CACHE.get_or_init(|| {
            Cache::builder()
                .max_capacity(100)
                .time_to_live(Duration::from_secs(300))
                .build()
        })
    }
    pub fn get_cache() -> AppResult<&'static Cache<String, String>> {
        let cache = CACHE
            .get()
            .ok_or(AppError::Other("验证码缓存获取失败".to_string()))?;
        Ok(cache)
    }
    pub async fn insert(k: &str, v: &str) -> AppResult<()> {
        Self::get_cache()?
            .insert(k.to_string(), v.to_string())
            .await;
        Ok(())
    }

    pub async fn get(k: &str) -> AppResult<Option<String>> {
        let cache = Self::get_cache()?;
        Ok(cache.get(k).await)
    }

    pub async fn remove(k: &str) -> AppResult<Option<String>> {
        let cache = Self::get_cache()?;
        Ok(cache.remove(k).await)
    }
}

pub static PASER: OnceLock<UserAgentParser> = OnceLock::new();
