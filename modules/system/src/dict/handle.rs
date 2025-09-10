use common::AppError;
use common::AppResult;
use common::response::ResponseResult;
use salvo::Writer;
use salvo::handler;
use salvo::oapi::extract::QueryParam;

#[handler]
pub async fn get_type_list() -> AppResult<()> {
    Err(AppError::AccTokenInvalid)
}

#[handler]
pub async fn get_data_list_by_type_id(type_id: QueryParam<i64>) -> AppResult<ResponseResult<i64>> {
    println!("{type_id}");
    Ok(ResponseResult::success(type_id.into_inner()))
}
