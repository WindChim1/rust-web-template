use salvo::{Depot, Request, Response, Writer, async_trait, http::StatusCode, writing::Json};
use serde::Serialize;
use serde_json::{Value, json};

/// T 是具体的业务数据类型，必须也能被序列化
#[derive(Debug, Serialize)]
pub struct ResponseResult<T: Serialize> {
    pub code: u16,
    pub msg: String,
    // 业务数据
    pub data: T,
}

impl<T: Serialize> ResponseResult<T> {
    /// 创建一个成功的响应，包含业务数据
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            msg: "sucess".to_string(),
            data,
        }
    }

    pub fn success_with_msg(msg: &str, data: T) -> Self {
        Self {
            code: 200,
            msg: msg.to_string(),
            data,
        }
    }

    /// 创建一个成功的响应，不包含业务数据
    /// `()` 在 Rust 中是一个有效的类型，并且序列化为空
    pub fn success_msg(msg: &str) -> ResponseResult<()> {
        ResponseResult {
            code: 200,
            msg: msg.to_string(),
            data: (),
        }
    }
    /// 创建一个失败的响应
    /// 这里的泛型 T 通常是 `()`，因为失败时没有业务数据
    pub fn error(code: u16, msg: &str) -> ResponseResult<()> {
        ResponseResult {
            code,
            msg: msg.to_string(),
            data: (),
        }
    }
}

#[test]
fn response_result_test() -> serde_json::Result<()> {
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Debug, Serialize, Deserialize)]
    struct TestUser {
        name: String,
        age: u8,
    }

    let test_user = TestUser {
        name: "".to_owned(),
        age: 18,
    };
    let resutl = ResponseResult::success(test_user);
    let json_value = serde_json::to_value(&resutl)?;
    assert_eq!(
        json_value,
        json!({
            "code": 200,
            "msg": "success",
            "data": {
                "name": "",
                "age": 18
            }
        })
    );

    Ok(())
}

// 为ResponseResult实现Writer trait
#[async_trait]
impl<T: Serialize + Send + Sync> Writer for ResponseResult<T> {
    async fn write(self, _req: &mut Request, _depot: &mut Depot, res: &mut Response)
    where
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: 'async_trait,
    {
        // 设置默认HTTP状态码（成功响应通常用200）
        res.status_code(StatusCode::OK);
        // 将ResponseResult序列化为JSON并写入响应
        res.render(Json(self));
    }
}

impl<T: Serialize> From<ResponseResult<T>> for Value {
    fn from(value: ResponseResult<T>) -> Self {
        json!(value)
    }
}
