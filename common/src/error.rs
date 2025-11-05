use std::fmt::Debug;

use salvo::{
    Depot, Request, Response, Writer, async_trait, http::StatusCode, oapi::EndpointOutRegister,
    writing::Json,
};
use thiserror::Error;
use tracing::error;

use crate::response::ResponseResult;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database  error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Config  error: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("Jwt error:{0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error("Job scheduler error:{0}")]
    JobSchedulerError(String),
    #[error("Sso auth failed:{0}")]
    SsoAuthFailed(String),
    #[error("Invalid login credentials (username, password)")]
    InvalidCredentials,
    #[error("Captcha verification failed")]
    CaptchaError,
    #[error("The verification code expires")]
    CaptchaExpired,
    #[error("Record Not Found")]
    RecordNotFound,
    #[error("Validation Failed")]
    ValidationFailed(String),
    #[error("token is invalid or expired")]
    TokenInvalid,
    #[error("Permission denied")]
    PermissionDenied,
    #[error(transparent)]
    JsonParseError(#[from] serde_json::Error),
    #[error(transparent)]
    TimeParseError(#[from] time::error::InvalidFormatDescription),
    #[error("{0}")]
    Other(String),
}

pub type Result<T, E = AppError> = std::result::Result<T, E>;

#[async_trait]
impl Writer for AppError {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        error!("[AppError] An application error occurred: {}", self);
        // 1. 根据错误类型，映射到 (HTTP状态码, 业务码, 消息)
        let (http_status, business_code, message) = match self {
            // 系统级错误 -> 500
            AppError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                500,
                "服务器内部错误".to_string(),
            ),
            AppError::ConfigError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                500,
                "服务器配置错误".to_string(),
            ),
            AppError::JwtError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                500,
                "令牌处理异常".to_string(),
            ),
            AppError::JobSchedulerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                500,
                format!("定时任务调度失败: {}", msg),
            ),
            AppError::SsoAuthFailed(msg) => (StatusCode::BAD_REQUEST, 500, msg),
            AppError::InvalidCredentials => {
                (StatusCode::BAD_REQUEST, 500, "用户名或密码错误".to_string())
            }
            AppError::CaptchaError => (StatusCode::BAD_REQUEST, 400, "验证码错误".to_string()),
            AppError::CaptchaExpired => (StatusCode::BAD_REQUEST, 500, "验证码已过期".to_string()),
            AppError::RecordNotFound => {
                (StatusCode::BAD_REQUEST, 404, "请求的资源不存在".to_string())
            }
            AppError::ValidationFailed(msg) => (StatusCode::BAD_REQUEST, 400, msg),

            // 认证/授权错误
            AppError::TokenInvalid => (
                StatusCode::UNAUTHORIZED,
                401,
                "认证令牌无效或已过期".to_string(),
            ),
            AppError::PermissionDenied => (StatusCode::FORBIDDEN, 403, "权限不足".to_string()),

            AppError::JsonParseError(e) => {
                (StatusCode::BAD_REQUEST, 400, format!("JSON格式错误: {}", e))
            }
            AppError::TimeParseError(e) => {
                (StatusCode::BAD_REQUEST, 400, format!("时间格式错误: {}", e))
            }
            AppError::Other(e) => (StatusCode::INTERNAL_SERVER_ERROR, 500, e.to_owned()),
        };
        let reponse_result = ResponseResult::<()>::error(business_code, &message);
        res.status_code(http_status);
        res.render(Json(reponse_result));
    }
}

// 为 AppError 实现 EndpointOutRegister
impl EndpointOutRegister for AppError {
    fn register(components: &mut salvo::oapi::Components, operation: &mut salvo::oapi::Operation) {
        todo!()
    }
    // fn register(registry: &mut EndpointOutRegistry) {
    //     // 注册错误处理逻辑：将 AppError 转换为 HTTP 响应
    //     registry.register::<Self, _>(|err: Self, _req: &Request, res: &mut Response| {
    //         // 示例：根据错误内容设置响应状态码和 body
    //         res.status_code(StatusCode::BAD_REQUEST); // 可根据实际错误类型动态调整
    //         res.render(Json(ResponseResult::error(
    //             400,                      // 错误码
    //             err.to_string().as_str(), // 错误信息
    //         )));
    //     });
    // }
}

#[cfg(test)]
mod test {
    use crate::error::{AppError, Result};

    #[test]
    fn error_test() -> anyhow::Result<()> {
        Result::Err(AppError::CaptchaError)?
    }
}
