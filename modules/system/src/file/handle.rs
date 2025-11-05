use std::{collections::HashMap, path::Path};

use common::{AppError, AppResult, response::ResponseResult};
use framework::config::Upload;
use salvo::{http::form::FilePart, prelude::*};
use tracing::{error, info};

use crate::file::UploadTool;
#[handler]
pub async fn index(res: &mut Response) {
    res.render(Text::Html(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>文件上传</title>
</head>
<body>
    <h1>上传文件</h1>
    <form action="/sys/upload" method="post" enctype="multipart/form-data">
        <input type="file" name="file" />
        <input type="submit" value="上传" />
    </form>
</body>
</html>"#,
    ));
}

#[handler]
pub async fn upload(req: &mut Request, res: &mut Response) {
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

    let mut map = HashMap::new();
    let ul = UploadTool::get()
        .map_err(|e| res.render(Text::Plain(e.to_string())))
        .unwrap();
    match req.form_data().await {
        Ok(data) => {
            // 处理文件
            if !data.files.is_empty() {
                for (_, files) in data.files.iter_all() {
                    for file in files {
                        let file_name = match file.name() {
                            Some(s) => s,
                            None => {
                                error!("文件名解析失败");
                                res.status_code(StatusCode::BAD_REQUEST);
                                res.render(Text::Plain("文件名解析失败"));
                                return;
                            }
                        };
                        if file.size() > MAX_FILE_SIZE {
                            res.status_code(StatusCode::PAYLOAD_TOO_LARGE);
                            res.render(Text::Plain(format!("{} 文件大小超过限制", file_name)));
                            break;
                        }
                        if let Err(e) = check(file, ul) {
                            println!("校验失败: {}", e);
                            return res.render(Json(ResponseResult::error(400, &e.to_string())));
                        }

                        //TODO: 文件上传地址
                        let dest = format!("uploads/{}", sanitize_filename(file_name));
                        info!("Saving uploaded file to: {}", dest);
                        let path = Path::new(&dest);
                        match std::fs::copy(file.path(), path) {
                            Ok(_) => {
                                // let path = path.to_str().unwrap().clone();
                                map.insert(file_name, dest.clone());
                                info!("文件上传成功: {:?}", path);
                            }
                            Err(e) => {
                                error!("{}", e);
                                res.status_code(StatusCode::BAD_REQUEST);
                                res.render(Text::Plain(e.to_string()));
                                break;
                            }
                        }
                    }
                }

                res.status_code(StatusCode::OK);
                res.render(Json(ResponseResult::success(map)));
            }
        }
        Err(e) => {
            res.status_code(StatusCode::BAD_REQUEST);
            res.render(Text::Plain(format!("表单解析错误: {}", e)));
        }
    }
}

fn check(file: &FilePart, upload_setting: &Upload) -> AppResult<()> {
    // 获取文件 Content-Type，无类型则返回错误
    let content_type = file
        .content_type()
        .ok_or_else(|| AppError::Other("文件类型解析失败".to_string()))?;

    let content_type_str = content_type.to_string();

    // 检查是否在允许的类型列表中
    if !upload_setting.allowed_types.contains(&content_type_str) {
        return Err(AppError::Other("文件类型不支持上传".to_string()));
    }

    // 提取类型的主类别（如 "image"、"video"）
    let key = content_type_str
        .split('/')
        .next()
        .ok_or_else(|| AppError::Other("文件类型格式无效".to_string()))?;

    // 检查该类型是否有对应的大小限制
    let max_size = upload_setting
        .max_size
        .get(key)
        .ok_or_else(|| AppError::Other("文件类型不支持上传".to_string()))?;

    // 检查文件大小是否超过限制
    if file.size() > *max_size as u64 {
        return Err(AppError::Other(format!(
            "{} 文件大小超过限制",
            file.name().unwrap_or("未知文件名") // 处理文件名可能为 None 的情况
        )));
    }

    Ok(())
}

fn sanitize_filename(filename: &str) -> String {
    // 移除路径遍历字符
    filename
        .replace("..", "")
        .replace("/", "_")
        .replace("\\", "_")
        .replace(":", "_")
}
