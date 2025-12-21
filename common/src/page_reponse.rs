use salvo::oapi::ToSchema;
use serde::Serialize;

/// 分页查询结果
#[derive(Debug, Serialize, ToSchema)]
pub struct PageReponse<T> {
    /// 数据列表
    pub items: Vec<T>,
    /// 分页元数据
    #[serde(flatten)]
    pub meta: PageMeta,
}

/// 分页元数据
#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
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

impl<U> PageReponse<U> {
    // 定义一个 map 方法，接收一个闭包或者利用类型推导
    pub fn map<T>(self, f: impl FnMut(U) -> T) -> PageReponse<T> {
        PageReponse {
            meta: self.meta,
            items: self.items.into_iter().map(f).collect(),
        }
    }

    pub fn convert<T>(self) -> PageReponse<T>
    where
        T: From<U>,
    {
        PageReponse {
            meta: self.meta,
            // 这里自动调用 T::from(u)
            items: self.items.into_iter().map(T::from).collect(),
        }
    }
}
