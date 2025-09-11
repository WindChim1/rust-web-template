use common::{AppError, AppResult, response::ResponseResult};
use framework::jwt::{JwtAuthUtil, TokenType};
use salvo::Writer;
use salvo::{Request, handler, oapi::extract::QueryParam};
use serde_json::Value;

// 刷新接口的 Handler（仅处理刷新令牌校验和新 Access Token 生成）
#[handler]
pub async fn refresh_token_handler(req: &mut Request) -> AppResult<ResponseResult<Value>> {
    const REF_TOKEN: &str = "refresh_token";
    let jwt_auth_util = JwtAuthUtil::get()?;
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

#[handler]
pub async fn login(
    user_name: QueryParam<String>,
    password: QueryParam<String>,
) -> AppResult<ResponseResult<Value>> {
    let user_name = user_name.into_inner();
    if user_name == "wdc" && password.into_inner() == "123" {
        let jwt_auth_util = JwtAuthUtil::get()?;
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
