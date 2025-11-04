pub mod handle;
pub mod model;
pub mod router;
use std::sync::OnceLock;

use common::{AppError, AppResult};
use framework::config::Upload;
pub use router::init_router;

static UPLOAD_SETTING: OnceLock<Upload> = OnceLock::new();

pub struct UploadTool;
impl UploadTool {
    pub fn init(setting: Upload) {
        UPLOAD_SETTING.get_or_init(|| setting);
    }
    pub fn get() -> AppResult<&'static Upload> {
        UPLOAD_SETTING
            .get()
            .ok_or(AppError::Other("文件上传配置初始化失败".to_string()))
    }
}
