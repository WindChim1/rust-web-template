use std::{collections::HashMap, fs, path::Path};

use common::{AppError, AppResult, response::ResponseResult};
use framework::config::Upload;
use salvo::{fs::NamedFile, http::form::FilePart, oapi::extract::QueryParam, prelude::*};
use time::OffsetDateTime;
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
    <form action="/sys/file/upload" method="post" enctype="multipart/form-data">
        <input type="file" name="file" />
        <input type="submit" value="上传" />
    </form>
</body>
</html>"#,
    ));
}

#[endpoint(tags("上传"), summary = "上传文件")]
pub async fn upload(req: &mut Request, res: &mut Response) {
    const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

    let mut map = HashMap::new();
    let ul = UploadTool::get()
        .map_err(|e| res.render(Text::Plain(e.to_string())))
        .unwrap();

    let file_path_prf = &ul.path;
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

                        let sf = sanitize_filename(file_name);

                        let now = OffsetDateTime::now_utc();
                        let date = now.date();
                        let path_prf_str = format!("{}{}", file_path_prf, date);
                        info!("Saving uploaded file to: {}", path_prf_str);

                        let path_prf = Path::new(&path_prf_str);
                        if !path_prf.exists() {
                            fs::create_dir_all(path_prf).expect("创建文件夹失败");
                        }

                        let pb = path_prf.join(&sf);

                        match std::fs::copy(file.path(), &pb) {
                            Ok(_) => {
                                map.insert(file_name, pb.clone());
                                info!("文件上传成功: {:?}", pb);
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

#[endpoint(tags("上传"), summary = "获取文件")]
pub async fn get(path: QueryParam<String>, req: &mut Request, res: &mut Response) {
    let file_path = path.into_inner();
    info!("[HANDLER] Entering get file,file_path:{:?}", file_path);
    // 1. (可选) 这里可以写权限判断逻辑
    // if !user.is_login() { return Err(...) }

    // 2. 发送文件
    // NamedFile 会自动识别 Content-Type，处理 Range 请求（视频拖动）等
    match NamedFile::open(file_path).await {
        Ok(file) => {
            // 【进阶】如果想强制浏览器下载，而不是预览，请加上这行 Header
            // 如果不加这行，浏览器会尝试直接打开 PDF 或 图片
            // res.headers_mut().insert(
            //     "Content-Disposition",
            //     "attachment; filename=\"contract.pdf\"".parse().unwrap(),
            // );

            file.send(req.headers(), res).await;
        }
        Err(_) => {
            res.status_code(StatusCode::NOT_FOUND);
            res.render("文件不存在");
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

#[test]
fn time_utc() {
    let now = OffsetDateTime::now_utc();
    let day = now.date();
    println!("{}", day);
    assert_eq!(1, 2)
}
