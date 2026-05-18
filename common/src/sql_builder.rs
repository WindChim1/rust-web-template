use sqlx::{Encode, Execute, Pool, Postgres, QueryBuilder, Type, postgres::PgHasArrayType};
use std::{fmt::Display, marker::PhantomData};

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

    /// WHERE条件（自动处理AND连接）
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

            // 检查是否需要 WHERE 或 AND
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

            // --- 核心逻辑 ---
            // 这里的 condition 传入的是 "id = ANY"
            // push_bind 会生成 "$n"
            // 最终拼接结果是 "id = ANY $n"

            if let Some(count_builder) = self.count_builder.as_mut() {
                count_builder.push(condition);
                count_builder.push_bind(processed.clone());
            }

            self.query_builder.push(condition);
            self.query_builder.push_bind(processed);
        }

        self
    }

    /// LIKE条件（自动处理通配符）
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

    /// 不区分大小写的LIKE条件（使用ILIKE）
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

    //等于条件
    pub fn where_eq<T>(&mut self, column: &str, value: Option<T>) -> &mut Self
    where
        T: Encode<'a, Postgres> + Type<Postgres> + Clone + std::fmt::Debug + IsQueryEmpty + 'a,
    {
        // 1. 拦截 None
        let Some(v) = value.as_ref() else {
            return self;
        };

        // 2. 优雅地判断是否为空（编译期已确定调用哪个逻辑）
        if !v.is_query_empty() {
            let clause = format!("{} = ", column);
            self.where_clause(&clause, value, |v| v.clone());
        }

        self
    }

    ///in 条件
    pub fn where_in<T>(&mut self, column: &str, value: Option<Vec<T>>) -> &mut Self
    where
        T: Encode<'a, Postgres> + Type<Postgres> + Clone + 'a + Display,
        T: PgHasArrayType,
    {
        if let Some(v) = value {
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

            let sql_part = format!("{} = ANY(", column);

            // 处理 count 语句
            if let Some(count_builder) = self.count_builder.as_mut() {
                count_builder.push(&sql_part);
                count_builder.push_bind(v.clone());
                count_builder.push(")");
            }

            // 处理主查询语句
            self.query_builder.push(sql_part);
            self.query_builder.push_bind(v);
            self.query_builder.push(")");
        }

        self
    }

    /// 大于等于条件
    pub fn where_ge<T>(&mut self, column: &str, value: Option<T>) -> &mut Self
    where
        T: Encode<'a, Postgres> + Type<Postgres> + Clone + 'a,
    {
        self.where_clause(&format!("{} >=  ", column), value, |v| v.clone())
    }

    /// 小于等于条件
    pub fn where_le<T>(&mut self, column: &str, value: Option<T>) -> &mut Self
    where
        T: Encode<'a, Postgres> + Type<Postgres> + Clone + 'a,
    {
        self.where_clause(&format!("{} <=  ", column), value, |v| v.clone())
    }

    /// 排序
    pub fn order_by(&mut self, column: &str, direction: Option<&str>) -> &mut Self {
        let direction = direction.unwrap_or("ASC");
        self.query_builder
            .push(format!(" ORDER BY {} {}", column, direction));
        self
    }

    pub fn group_by(&mut self, column: &str) -> &mut Self {
        self.query_builder.push(format!(" GROUP BY {} ", column));
        let count_builder = self.count_builder.as_mut().map(|b| {
            b.push(format!(" GROUP BY {} ", column));
            let sql = b.sql();
            let final_sql = format!("SELECT COUNT(*) FROM ({})", sql);
            let mut query = b.build();
            let args = query.take_arguments().unwrap().unwrap();
            QueryBuilder::<Postgres>::with_arguments(final_sql, args)
        });
        self.count_builder = count_builder;
        self
    }

    ///连续排序
    pub fn and_order_by(&mut self, column: &str, direction: Option<&str>) -> &mut Self {
        let direction = direction.unwrap_or("ASC");
        self.query_builder
            .push(format!(", {} {}", column, direction));
        self
    }

    /// 分页
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

        // 分页条件
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
// 1. 定义 Trait
pub trait IsQueryEmpty {
    fn is_query_empty(&self) -> bool {
        false
    }
}

// 2. 为 String 基础类型实现
impl IsQueryEmpty for String {
    fn is_query_empty(&self) -> bool {
        self.trim().is_empty()
    }
}

// 3. 为 str (注意是 str 而不是 &str) 基础类型实现
impl IsQueryEmpty for str {
    fn is_query_empty(&self) -> bool {
        self.trim().is_empty()
    }
}

// 4. ⭐ 核心修复：为所有引用类型提供通用实现！
// 它的意思是：“只要 T 实现了该 Trait，那么 T 的引用 &T 也自动实现该 Trait”
impl<T: ?Sized + IsQueryEmpty> IsQueryEmpty for &T {
    fn is_query_empty(&self) -> bool {
        // 解引用一层，继续调用底层的实现
        (*self).is_query_empty()
    }
}

// 5. 其他数字/布尔类型的默认实现
macro_rules! impl_not_empty {
    ($($t:ty),*) => {
        $(impl IsQueryEmpty for $t {})*
    };
}
impl_not_empty!(i8, i16, i32, i64, f32, f64, bool);
