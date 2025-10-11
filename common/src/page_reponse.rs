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
    pub page: u32,
    /// 每页条数
    pub page_size: u32,
    /// 总条数
    pub total: u32,
    /// 总页数
    pub total_pages: u32,
}
impl<T> PageReponse<T> {
    /// 创建分页响应
    pub fn new(items: Vec<T>, page: u32, page_size: u32, total: u32) -> Self {
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
