use argon2::{Argon2, PasswordHash, PasswordVerifier};
use captcha::Captcha;
use common::response::ResponseResult;
use common::{AppError, AppResult};
use framework::db::DBPool;
use framework::jwt::{JWTTool, TokenType};
use monitor::login_info;
use salvo::Request;
use salvo::oapi::extract::{JsonBody, QueryParam};
use salvo::{Writer, handler};
use serde_json::Value;
use sqlx::PgPool;
use time::OffsetDateTime;
use tracing::{error, info};
use user_agent_parser::UserAgentParser;
use uuid::Uuid;

use crate::model::{CapCache, CaptchaDTO, CaptchaVO, LoginDTO, PASER, TokenVO};
use crate::user::service;

/// 处理获取验证码图片
#[handler]
pub async fn get_captcha_image() -> AppResult<ResponseResult<CaptchaVO>> {
    info!("[HANDLER] Entering get  captcha image");
    // 生成 4 位验证码
    let captcha_string: String;
    let captcha_img: String;
    //NOTE：注意：限制Captcha生命周期
    {
        let mut captcha = Captcha::new();
        captcha
            .add_chars(4) // 验证码长度为4个字符
            // .apply_filter(Noise::new(0.2)) // 添加噪声干扰
            // .apply_filter(Wave::new(2.0, 10.0).horizontal()) // 添加水平扭曲
            // .apply_filter(Wave::new(2.0, 10.0).vertical()) // 添加垂直扭曲
            .set_color([20, 40, 80])
            .view(200, 70); // 设置字符颜色

        captcha_string = captcha.chars().iter().collect();
        // 将 UUID 和验证码答案存入缓存
        // 生成 PNG 图片
        captcha_img = match captcha.as_base64() {
            Some(i) => i,
            None => Err(AppError::Other("验证码图片生成失败".to_string()))?,
        }; // 获取图片的 Base64 编码
    }

    let uuid = Uuid::new_v4().to_string();
    CapCache::init_cache()
        .insert(uuid.clone(), captcha_string)
        .await;
    let captcha_vo = CaptchaVO {
        id: uuid,
        img: captcha_img,
    };

    Ok(ResponseResult::success(captcha_vo))
}

/// 刷新接口令牌的
#[handler]
pub async fn refresh_token_handler(req: &mut Request) -> AppResult<ResponseResult<TokenVO>> {
    info!("[HANDLER]  Entering  refresh token");
    const REFTOKEN: &str = "refreshToken";
    let jwt_auth_util = JWTTool::get()?;
    let ref_token = req
        .cookies()
        .get(REFTOKEN)
        .map(|c| c.value())
        .ok_or_else(|| AppError::TokenInvalid)?;

    // 校验 Refresh Token
    let ref_claims = jwt_auth_util.verify_ref_token(ref_token)?;
    // 若 Refresh Token 有效，生成新的 Access Token（刷新令牌可复用，或按需轮换）
    let new_acc_token = jwt_auth_util.generate_token(ref_claims.sub, TokenType::Access)?;
    let new_ref_token = jwt_auth_util.generate_token(ref_claims.sub, TokenType::Refresh)?;

    // 返回新的 Access Token 给前端
    let token_vo = TokenVO {
        access_token: new_acc_token,
        refresh_token: new_ref_token,
    };
    Ok(ResponseResult::success_with_msg("令牌刷新成功", token_vo))
}

///登录
#[handler]
pub async fn login(
    login_dto: JsonBody<LoginDTO>,
    req: &mut Request,
) -> AppResult<ResponseResult<TokenVO>> {
    info!("[HANDLER]  Entering login::with body:{:?}", login_dto);
    // 获取客户端地址
    let ipaddr = req
        .remote_addr()
        .as_ipv4()
        .map(|addr| addr.ip().to_string())
        .unwrap_or_default();
    let mut os = None;
    let mut browser = None;
    req.headers()
        .get("User-Agent")
        .and_then(|agent| agent.to_str().ok())
        .map(|s| {
            let user_agent_parser =
                PASER.get_or_init(|| UserAgentParser::from_path("regexes.yaml").expect(""));

            os = user_agent_parser.parse_os(s).name.map(|s| s.to_string());
            browser = user_agent_parser
                .parse_product(s)
                .name
                .map(|s| s.to_string());
        })
        .unwrap();
    println!("{os:?}");
    println!("{browser:?}");

    let LoginDTO {
        username,
        password,
        captcha: CaptchaDTO { uuid, code },
    } = login_dto.into_inner();

    let db_pool = DBPool::get().await?;
    // 1.1 验证码校验
    match CapCache::get(&uuid).await? {
        Some(cache_code) if cache_code.to_lowercase() == code.to_lowercase() => {
            //移除captcha
            CapCache::remove(&uuid).await?;
        }
        _ => {
            record_login_log(
                db_pool.clone(),
                username.clone(),
                ipaddr.clone(),
                os,
                browser,
                "1",
                "验证码错误或已过期".to_string(),
            )
            .await;
            return Err(AppError::CaptchaError);
        }
    }
    let db = DBPool::get().await?;

    //账号密码校验
    let user = service::select_user_by_username(db, &username).await?;
    let user = match user {
        Some(user) => user,
        None => {
            error!("[LOGIN_HANDLER] 用户 '{}' 不存在.", &username);
            record_login_log(
                db_pool.clone(),
                username.clone(),
                ipaddr.clone(),
                os,
                browser,
                "1",
                "用户密码未设置".to_string(),
            )
            .await;
            return Err(AppError::RecordNotFound)?;
        }
    };

    let password_from_db = if let Some(pwd) = user.password {
        pwd
    } else {
        // 如果密码是 None，记录日志并返回错误
        error!(
            "[LOGIN_HANDLER] 数据库中用户 '{}' 的密码字段为 NULL!",
            username
        );

        tokio::spawn(record_login_log(
            db_pool.clone(),
            username.clone(),
            ipaddr.clone(),
            os,
            browser,
            "1",
            "服务端密码处理错误".to_string(),
        ));

        return Err(AppError::InvalidCredentials)?;
    };

    let parsed_hash = PasswordHash::new(&password_from_db).map_err(|e| {
        error!(
            "[LOGIN_HANDLER] 密码哈希字符串解析失败! Error: {}. Hash: '{}'",
            e, password_from_db
        );
        tokio::spawn(record_login_log(
            db_pool.clone(),
            username.clone(),
            ipaddr.clone(),
            os.clone(),
            browser.clone(),
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
            username.clone(),
            ipaddr.clone(),
            os.clone(),
            browser.clone(),
            "1",
            "密码验证失败".to_string(),
        )
        .await;
        Err(AppError::InvalidCredentials)?
    }
    // 生成jwt
    let jwt_auth_util = JWTTool::get()?;
    let acc_token = jwt_auth_util.generate_token(user.user_id, TokenType::Access)?;
    let ref_token = jwt_auth_util.generate_token(user.user_id, TokenType::Refresh)?;

    let token_vo = TokenVO {
        access_token: acc_token,
        refresh_token: ref_token,
    };

    record_login_log(
        db_pool.clone(),
        username.clone(),
        ipaddr,
        os,
        browser,
        "0",
        "登录成功".to_string(),
    )
    .await;
    Ok(ResponseResult::success_with_msg("登录成功", token_vo))
}

#[handler]
pub async fn register(
    _username: QueryParam<String>,
    _password: QueryParam<String>,
) -> AppResult<ResponseResult<Value>> {
    //TODO:
    todo!()
}

async fn record_login_log(
    db_pool: PgPool,
    username: String,
    ipaddr: String,
    os: Option<String>,
    browser: Option<String>,
    status: &'static str,
    msg: String,
) {
    let log = login_info::model::SysLoginInfor {
        info_id: 0,
        user_name: Some(username),
        ipaddr: Some(ipaddr),
        // 以下字段可以后续通过 User-Agent 解析库或 IP 地址库来填充
        login_location: None,
        browser,
        os,
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

/// test
#[cfg(test)]
mod test {
    use argon2::{
        Argon2,
        password_hash::{
            self, PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng,
        },
    };
    use user_agent_parser::UserAgentParser;

    use crate::handle::PASER;
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
    #[test]
    fn user_agent_parser() {
        let user_agent_parser =
            PASER.get_or_init(|| UserAgentParser::from_path("../../regexes.yaml").expect(""));
        let s = "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:142.0) Gecko/20100101 Firefox/142.0";
        let os = user_agent_parser
            .parse_os(s)
            .name
            .map(|s| s.to_string())
            .unwrap();
        let product_name = user_agent_parser.parse_product(s).name.unwrap();

        let engine = user_agent_parser
            .parse_engine(s)
            .name
            .map(|s| s.to_string())
            .unwrap();
        let device_name = user_agent_parser.parse_device(s).name.unwrap();
        println!("{product_name}");
        println!("{device_name}");
        println!("{os}");
        println!("{engine}");
        assert_eq!(1, 2)
    }
}
