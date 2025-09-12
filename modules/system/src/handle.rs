use argon2::{Argon2, PasswordHash, PasswordVerifier};
use captcha::Captcha;
use common::response::ResponseResult;
use common::{AppError, AppResult};
use framework::db::DBPool;
use framework::jwt::{JWTTool, TokenType};
use monitor::login_info;
use salvo::Writer;
use salvo::handler;
use salvo::oapi::extract::{JsonBody, QueryParam};
use salvo::{Depot, Request};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;
use time::OffsetDateTime;
use tracing::{error, info};
use uuid::Uuid;

use crate::user::service;

#[derive(Debug, Serialize)]
struct CaptchaVO {
    id: String,
    img: String,
}

#[derive(Debug, Deserialize)]
struct CaptchaDTO {
    id: String,
    value: String,
}

/// 处理获取验证码图片的请求
#[handler]
pub async fn get_captcha_image(depot: &mut Depot) -> AppResult<ResponseResult<CaptchaVO>> {
    // 生成 4 位验证码
    let mut captcha = Captcha::new();
    captcha
        .add_chars(4) // 验证码长度为4个字符
        // .apply_filter(Noise::new(0.2)) // 添加噪声干扰
        // .apply_filter(Wave::new(2.0, 10.0).horizontal()) // 添加水平扭曲
        // .apply_filter(Wave::new(2.0, 10.0).vertical()) // 添加垂直扭曲
        .set_color([20, 40, 80])
        .view(200, 70); // 设置字符颜色

    let code_string: String = captcha.chars().iter().collect();
    let uuid = Uuid::new_v4().to_string();
    // 将 UUID 和验证码答案存入缓存
    depot.insert(&uuid, code_string);
    // 生成 PNG 图片
    let code_img = match captcha.as_base64() {
        Some(i) => i,
        None => Err(AppError::Other("验证码图片生成失败".to_string()))?,
    }; // 获取图片的 Base64 编码
    let captcha_vo = CaptchaVO {
        id: uuid,
        img: code_img,
    };

    ResponseResult::success(captcha_vo).into()
}

// 刷新接口的 Handler（仅处理刷新令牌校验和新 Access Token 生成）
#[handler]
pub async fn refresh_token_handler(req: &mut Request) -> AppResult<ResponseResult<Value>> {
    const REF_TOKEN: &str = "refresh_token";
    let jwt_auth_util = JWTTool::get()?;
    let ref_token = req
        .cookies()
        .get(REF_TOKEN)
        .map(|c| c.value())
        .ok_or_else(|| AppError::RefTokenInvalid)?;

    // 校验 Refresh Token
    let ref_claims = jwt_auth_util.verify_ref_token(ref_token)?;
    // 若 Refresh Token 有效，生成新的 Access Token（刷新令牌可复用，或按需轮换）
    let new_acc_token = jwt_auth_util.generate_token(ref_claims.sub.clone(), TokenType::Access)?;
    let new_ref_token = jwt_auth_util.generate_token(ref_claims.sub.clone(), TokenType::Refresh)?;

    // 返回新的 Access Token 给前端（可按需返回新的 Refresh Token，实现“令牌轮换”）
    let data = serde_json::json!({
        "data": {
            "access_token": new_acc_token,
            "refresh_token": new_ref_token,
        }
    });
    Ok(ResponseResult::success_with_msg("令牌刷新成功", data))
}

#[derive(Debug, Deserialize)]
pub struct LoginDTO {
    user_name: String,
    password: String,
    captcha: CaptchaDTO,
}

///登录
#[handler]
pub async fn login(
    login_dto: JsonBody<LoginDTO>,
    depot: &mut Depot,
    req: &mut Request,
) -> AppResult<ResponseResult<Value>> {
    // 获取客户端地址
    let ipaddr = req
        .remote_addr()
        .as_ipv4()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_default();

    let LoginDTO {
        user_name,
        password,
        captcha: CaptchaDTO { id, value },
    } = login_dto.into_inner();

    //验证码判断
    let captcha = depot
        .get::<String>(&id)
        //验证码时效
        .map_err(|_| AppError::CaptchaExpired)?;
    //验证码错误
    if captcha.as_str() != value {
        Err(AppError::CaptchaError)?;
    }

    let db_pool = DBPool::get().await?;
    //账号密码校验
    let user = service::select_user_by_user_name(&user_name).await?;
    let user = match user {
        Some(user) => user,
        None => {
            error!("[LOGIN_HANDLER] 用户 '{}' 不存在.", &user_name);
            record_login_log(
                db_pool.clone(),
                user_name.clone(),
                ipaddr.clone(),
                "1",
                "用户密码未设置".to_string(),
            )
            .await;

            return Err(AppError::RecordNotFound);
        }
    };

    // let db_user_clone = db_user.clone();
    let password_from_db = if let Some(pwd) = user.password {
        pwd
    } else {
        // 如果密码是 None，记录日志并返回错误
        error!(
            "[LOGIN_HANDLER] 数据库中用户 '{}' 的密码字段为 NULL!",
            user_name
        );

        tokio::spawn(record_login_log(
            db_pool.clone(),
            user_name.clone(),
            ipaddr.clone(),
            "1",
            "服务端密码处理错误".to_string(),
        ));

        return Err(AppError::InvalidCredentials);
    };

    let parsed_hash = PasswordHash::new(&password_from_db).map_err(|e| {
        error!(
            "[LOGIN_HANDLER] 密码哈希字符串解析失败! Error: {}. Hash: '{}'",
            e, password_from_db
        );
        tokio::spawn(record_login_log(
            db_pool.clone(),
            user_name.clone(),
            ipaddr.clone(),
            "1",
            "服务端密码处理错误".to_string(),
        ));
        AppError::Other(e.to_string())
    })?;

    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        record_login_log(
            db_pool.clone(),
            user_name.clone(),
            ipaddr,
            "1",
            "密码验证失败".to_string(),
        )
        .await;
        Err(AppError::InvalidCredentials)?
    }
    // 生成jwt
    let jwt_auth_util = JWTTool::get()?;
    let acc_token = jwt_auth_util.generate_token(user_name.clone(), TokenType::Access)?;
    let ref_token = jwt_auth_util.generate_token(user_name, TokenType::Refresh)?;

    let data = serde_json::json!({
        "data": {
            "access_token": acc_token,
            "refresh_token": ref_token,
        }
    });
    Ok(ResponseResult::success_with_msg("登录成功", data))
}

#[handler]
pub async fn register(
    user_name: QueryParam<String>,
    password: QueryParam<String>,
) -> AppResult<ResponseResult<Value>> {
    let user_name = user_name.into_inner();
    if user_name == "wdc" && password.into_inner() == "123" {
        let jwt_auth_util = JWTTool::get()?;
        let acc_token = jwt_auth_util.generate_token(user_name.clone(), TokenType::Access)?;
        let ref_token = jwt_auth_util.generate_token(user_name, TokenType::Refresh)?;

        let data = serde_json::json!({
            "data": {
                "access_token": acc_token,
                "refresh_token": ref_token,
            }
        });
        Ok(ResponseResult::success_with_msg("登录成功", data))
    } else {
        Err(AppError::InvalidCredentials)
    }
}

async fn record_login_log(
    db_pool: PgPool,
    user_name: String,
    ipaddr: String,
    status: &'static str,
    msg: String,
) {
    let log = login_info::model::SysLoginInfor {
        info_id: 0,
        user_name: Some(user_name),
        ipaddr: Some(ipaddr),
        // 以下字段可以后续通过 User-Agent 解析库或 IP 地址库来填充
        login_location: None,
        browser: None,
        os: None,
        status: Some(status.to_string()),
        msg: Some(msg),
        // login_time: Some(Local::now().naive_local()),
        login_time: Some(OffsetDateTime::now_utc()),
    };

    // 在一个独立的后台任务中执行数据库写入
    tokio::spawn(async move {
        if let Err(e) = login_info::service::add_logininfor(&db_pool, log).await {
            // 这里的错误只会打印到服务器日志，不会影响主登录流程
            error!("[LOG_TASK] 记录登录日志失败: {:?}", e);
        } else {
            info!("[LOG_TASK] 登录日志记录成功。");
        }
    });
}

#[cfg(test)]
mod test {
    use argon2::{
        Argon2,
        password_hash::{
            self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
        },
    };
    #[test]
    fn argon2_test() -> password_hash::Result<()> {
        let password = b"admin"; // Bad password; don't actually use!
        let salt = SaltString::generate(&mut OsRng);

        // Argon2 with default params (Argon2id v19)
        let argon2 = Argon2::default();

        // Hash password to PHC string ($argon2id$v=19$...)
        let password_hash = argon2.hash_password(password, &salt)?.to_string();
        println!("{password_hash}");

        // Verify password against PHC string.
        //
        // NOTE: hash params from `parsed_hash` are used instead of what is configured in the
        // `Argon2` instance.
        let parsed_hash = PasswordHash::new(&password_hash)?;
        assert!(
            Argon2::default()
                .verify_password(password, &parsed_hash)
                .is_ok()
        );
        Ok(())
    }
}
