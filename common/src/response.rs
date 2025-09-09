use serde::Serialize;

/// 通用响应体，模仿 RuoYi 的 AjaxResult
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
