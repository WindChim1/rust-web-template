use serde::Serialize;

/// 分页查询结果
#[derive(Debug, Serialize)]
pub struct PageReponse<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 分页元数据
    pub meta: PageMeta,
}

/// 分页元数据
#[derive(Debug, Serialize)]
pub struct PageMeta {
    /// 当前页码
    pub page: u64,
    /// 每页条数
    pub page_size: u64,
    /// 总条数
    pub total: u64,
    /// 总页数
    pub total_pages: u64,
}
impl<T> PageReponse<T> {
    /// 创建分页响应
    pub fn new(items: Vec<T>, page: u64, page_size: u64, total: u64) -> Self {
        let total_pages = if total == 0 {
            0
        } else {
            total.div_ceil(page_size)
        };

        Self {
            items,
            meta: PageMeta {
                page,
                page_size,
                total,
                total_pages,
            },
        }
    }
}
