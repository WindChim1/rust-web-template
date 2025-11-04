use salvo::Router;

use crate::file::handle::{index, upload};

pub fn init_router() -> Router {
    Router::with_path("upload")
        .get(index) // 处理 GET 请求，显示上传表单
        .post(upload) // 处理 POST 请求，处理文件上传
}
