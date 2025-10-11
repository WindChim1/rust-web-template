use sqlx::{Encode, Pool, Postgres, QueryBuilder, Type};
use std::marker::PhantomData;

use crate::{AppResult, page_reponse::PageReponse};

/// SQL查询构建器，支持条件构建和分页查询（专为PostgreSQL优化）
pub struct SqlBuilder<'a> {
    db: &'static Pool<Postgres>,
    query_builder: QueryBuilder<'a, Postgres>,
    count_builder: Option<QueryBuilder<'a, Postgres>>,
    has_where_clause: bool,
    _phantom: PhantomData<&'a ()>,
}

impl<'a> SqlBuilder<'a> {
    /// 创建新的SQL构建器
    pub fn new(db: &'static Pool<Postgres>, query_sql: &str) -> Self {
        let query_builder = QueryBuilder::new(query_sql);

        Self {
            db,
            query_builder,
            count_builder: None,
            has_where_clause: query_sql.to_lowercase().contains("where"),
            _phantom: PhantomData,
        }
    }

    /// 创建用于分页查询的SQL构建器
    pub fn for_pagination(
        db: &'static Pool<Postgres>,
        select_clauses: &str,
        table_name: &str,
        default_condition: Option<&str>,
    ) -> Self {
        let mut query_sql = format!("select {} from {}", select_clauses, table_name);
        let mut count_sql = format!("SELECT COUNT(*) FROM {}", table_name);

        // 处理默认条件
        if let Some(condition) = default_condition {
            query_sql.push_str(&format!(" WHERE {}", condition));
            count_sql.push_str(&format!(" WHERE {}", condition));
        }

        Self {
            db,
            query_builder: QueryBuilder::new(&query_sql),
            count_builder: Some(QueryBuilder::new(&count_sql)),
            has_where_clause: query_sql.to_lowercase().contains("where"),
            _phantom: PhantomData,
        }
    }

    /// 添加WHERE条件（自动处理AND连接）
    pub fn where_clause<T, F>(
        &mut self,
        condition: &str,
        param: Option<T>,
        processor: F,
    ) -> &mut Self
    where
        T: Encode<'a, Postgres> + Type<Postgres> + 'a + Clone,
        F: FnOnce(T) -> T,
    {
        if let Some(value) = param {
            let processed = processor(value);

            // 检查是否需要添加WHERE或AND
            if !self.has_where_clause {
                self.query_builder.push(" WHERE ");
                if let Some(count_builder) = self.count_builder.as_mut() {
                    count_builder.push(" WHERE ");
                }
                self.has_where_clause = true;
            } else {
                self.query_builder.push(" AND ");
                if let Some(count_builder) = self.count_builder.as_mut() {
                    count_builder.push(" AND ");
                }
            }
            if let Some(count_builder) = self.count_builder.as_mut() {
                count_builder.push(condition);
                count_builder.push_bind(processed.clone());
            }
            // 添加条件条件到两个构建器
            self.query_builder.push(condition);
            self.query_builder.push_bind(processed);
        }

        self
    }

    /// 添加LIKE条件（自动处理通配符）
    pub fn where_like(&mut self, column: &str, value: Option<&str>) -> &mut Self {
        if let Some(val) = value {
            let trimmed = val.trim();
            if !trimmed.is_empty() {
                self.where_clause(
                    &format!("{} LIKE  ", column),
                    Some(trimmed.to_string()),
                    |v| format!("%{}%", v),
                );
            }
        }
        self
    }

    pub fn where_right_like(&mut self, column: &str, value: Option<&str>) -> &mut Self {
        if let Some(val) = value {
            let trimmed = val.trim();
            if !trimmed.is_empty() {
                self.where_clause(
                    &format!("{} LIKE  ", column),
                    Some(trimmed.to_string()),
                    |v| format!("{}%", v),
                );
            }
        }
        self
    }

    pub fn where_left_like(&mut self, column: &str, value: Option<&str>) -> &mut Self {
        if let Some(val) = value {
            let trimmed = val.trim();
            if !trimmed.is_empty() {
                self.where_clause(
                    &format!("{} LIKE  ", column),
                    Some(trimmed.to_string()),
                    |v| format!("%{}", v),
                );
            }
        }
        self
    }

    /// 添加不区分大小写的LIKE条件（使用ILIKE）
    pub fn where_ilike(&mut self, column: &str, value: Option<&str>) -> &mut Self {
        if let Some(val) = value {
            let trimmed = val.trim();
            if !trimmed.is_empty() {
                self.where_clause(
                    &format!("{} ILIKE  ", column),
                    Some(trimmed.to_string()),
                    |v| format!("%{}%", v),
                );
            }
        }
        self
    }

    /// 添加等于条件
    pub fn where_eq<T>(&mut self, column: &str, value: Option<T>) -> &mut Self
    where
        T: Encode<'a, Postgres> + Type<Postgres> + Clone + 'a,
    {
        self.where_clause(&format!("{} =  ", column), value, |v| v.clone())
    }

    /// 添加大于等于条件
    pub fn where_ge<T>(&mut self, column: &str, value: Option<T>) -> &mut Self
    where
        T: Encode<'a, Postgres> + Type<Postgres> + Clone + 'a,
    {
        self.where_clause(&format!("{} >=  ", column), value, |v| v.clone())
    }

    /// 添加小于等于条件
    pub fn where_le<T>(&mut self, column: &str, value: Option<T>) -> &mut Self
    where
        T: Encode<'a, Postgres> + Type<Postgres> + Clone + 'a,
    {
        self.where_clause(&format!("{} <=  ", column), value, |v| v.clone())
    }

    /// 添加排序
    pub fn order_by(&mut self, column: &str, direction: Option<&str>) -> &mut Self {
        let direction = direction.unwrap_or("ASC");
        self.query_builder
            .push(format!(" ORDER BY {} {}", column, direction));
        self
    }

    /// 添加分页
    pub fn paginate(&mut self, page: u32, page_size: u32) -> &mut Self {
        let offset = (page - 1) * page_size;
        self.query_builder
            .push(format!(" LIMIT {} OFFSET {}", page_size, offset));
        self
    }

    /// 执行COUNT查询
    pub async fn count(&mut self) -> AppResult<u32> {
        if let Some(count_builder) = self.count_builder.as_mut() {
            let total: (i64,) = count_builder.build_query_as().fetch_one(self.db).await?;
            return Ok(total.0 as u32);
        }
        Ok(0)
    }

    /// 执行查询并返回单个结果
    pub async fn fetch_one<U>(&mut self) -> AppResult<U>
    where
        U: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let result = self
            .query_builder
            .build_query_as::<U>()
            .fetch_one(self.db)
            .await?;
        Ok(result)
    }

    /// 执行查询并返回多个结果
    pub async fn fetch_all<U>(&mut self) -> AppResult<Vec<U>>
    where
        U: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let results = self
            .query_builder
            .build_query_as::<U>()
            .fetch_all(self.db)
            .await?;
        Ok(results)
    }

    /// 执行分页查询并返回分页结果
    pub async fn fetch_paged<U>(&mut self, page: u32, page_size: u32) -> AppResult<PageReponse<U>>
    where
        U: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        // 先查询总数
        let total = self.count().await?;

        // 添加分页条件
        self.paginate(page, page_size);

        // 查询数据
        let data = self.fetch_all().await?;
        let page = PageReponse::new(data, page, page_size, total);

        Ok(page)
    }
    pub fn quer_sql(&self) -> &str {
        self.query_builder.sql()
    }
    pub fn count_sql(&self) -> Option<&str> {
        self.count_builder.as_ref().map(|c| c.sql())
    }
}
