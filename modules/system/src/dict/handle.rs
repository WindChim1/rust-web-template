use common::AppError;
use common::AppResult;
use common::response::ResponseResult;
use salvo::Writer;
use salvo::handler;
use salvo::oapi::extract::QueryParam;

#[handler]
pub async fn get_type_list() -> AppResult<()> {
    Err(AppError::TokenInvalid)
}

#[handler]
pub async fn get_data_list_by_type_id(type_id: QueryParam<i64>) -> AppResult<ResponseResult<()>> {
    Ok(ResponseResult::success(()))
}
