// use common::{AppError, AppResult, response::ResponseResult};
// use framework::jwt::{JwtAuthUtil, TokenType};
// use salvo::Writer;
// use salvo::oapi::extract::JsonBody;
// use salvo::{Request, handler, oapi::extract::QueryParam};
// use serde_json::Value;

// // use common::{AppResult, response::ResponseResult};

// use crate::user::{handle, model::SysUserVO};

// #[handler]
// pub async fn add_user(user: JsonBody<SysUserVO>) -> AppResult<ResponseResult<()>> {
//     let user = user.into_inner();
//     todo!()
// }

// #[handler]
// pub async fn get_user(user: JsonBody<SysUserVO>) -> AppResult<ResponseResult<()>> {
//     let user = user.into_inner();
//     todo!()
// }
