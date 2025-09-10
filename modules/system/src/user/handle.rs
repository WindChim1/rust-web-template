use common::{AppError, AppResult, response::ResponseResult};
use framework::jwt::{JWTONCELOCK, TokenType};
use salvo::oapi::extract::QueryParam;
use salvo::{Writer, handler};
use serde_json::Value;

// 刷新接口的 Handler（仅处理刷新令牌校验和新 Access Token 生成）
#[handler]
async fn refresh_token_handler(ref_token: QueryParam<String>) -> AppResult<ResponseResult<Value>> {
    let jwt_auth_util = JWTONCELOCK
        .get()
        .ok_or(AppError::Other("JWT 工具初始化失败".to_string()))?;

    // 校验 Refresh Token
    let ref_claims = jwt_auth_util.verify_ref_token(&ref_token)?;
    // 若 Refresh Token 有效，生成新的 Access Token（刷新令牌可复用，或按需轮换）
    let new_acc_token = jwt_auth_util.generate_token(ref_claims.sub.clone(), TokenType::Access)?;
    let new_ref_token = jwt_auth_util.generate_token(ref_claims.sub.clone(), TokenType::Refresh)?;

    // 步骤4：返回新的 Access Token 给前端（可按需返回新的 Refresh Token，实现“令牌轮换”）
    let data = serde_json::json!({
        "data": {
            "access_token": new_acc_token,
            "refresh_token": new_ref_token,
        }
    });
    Ok(ResponseResult::success_with_msg("令牌刷新成功", data))
}
