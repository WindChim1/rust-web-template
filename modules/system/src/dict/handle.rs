use common::AppResult;
use common::response::ResponseResult;
use salvo::Writer;
use salvo::handler;
use salvo::oapi::extract::QueryParam;

#[handler]
pub async fn get_type_list() -> AppResult<ResponseResult<()>> {
    ResponseResult::success(()).into()
}

#[handler]
pub async fn get_data_list_by_type_id(type_id: QueryParam<i64>) -> AppResult<ResponseResult<i64>> {
    println!("{type_id}");
    ResponseResult::success(type_id.into_inner()).into()
}
