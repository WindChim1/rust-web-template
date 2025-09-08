use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct PageRequest {
    /// 页码，默认为1
    #[serde(default = "default_page")]
    pub page: u64,
    /// 每页条数，默认为10，最大100
    #[serde(default = "default_page_size")]
    pub page_size: u64,
}
impl PageRequest {
    /// 计算偏移量（用于数据库查询）
    pub fn offset(&self) -> u64 {
        (self.page - 1) * self.page_size
    }

    /// 确保page_size在合理范围内
    pub fn normalize(&mut self) {
        if self.page_size == 0 {
            self.page_size = 10;
        } else if self.page_size > 100 {
            self.page_size = 100;
        }

        if self.page == 0 {
            self.page = 1;
        }
    }
}

// 默认页码
fn default_page() -> u64 {
    1
}

// 默认每页条数
fn default_page_size() -> u64 {
    10
}
