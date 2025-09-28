use sqlx::{Encode, QueryBuilder, Type};

pub struct SqlBuilder<'a, DB>
where
    DB: sqlx::Database,
{
    pub query_builder: QueryBuilder<'a, DB>,
    pub count_builder: Option<QueryBuilder<'a, DB>>,
    // mhantom: std::marker::PhantomData<T>,
}

impl<'a, DB> SqlBuilder<'a, DB>
where
    DB: sqlx::Database,
{
    pub fn new(
        query_builder: QueryBuilder<'a, DB>,
        count_builder: Option<QueryBuilder<'a, DB>>,
    ) -> Self {
        Self {
            query_builder,
            count_builder,
        }
    }

    /// 构建sqluery's where条件
    pub fn build_condition<T, F>(
        &mut self,
        param: Option<T>,
        condition_template: &str,
        bind_process: F,
    ) -> &mut Self
    where
        T: ToString + Clone + 'a,
        T: Encode<'a, DB> + Type<DB>,
        F: FnOnce(T) -> T, // 处理值的闭包
    {
        if let Some(value) = param {
            let bind = bind_process(value);
            if !bind.to_string().trim().is_empty() {
                if let Some(count_builder) = self.count_builder.as_mut() {
                    count_builder.push(condition_template);
                    count_builder.push_bind(bind.clone());
                }
                self.query_builder.push(condition_template);
                self.query_builder.push_bind(bind);
            }
        }
        self
    }
    pub fn build_query_condition<T, F>(
        &mut self,
        param: Option<T>,
        condition_template: &str,
        bind_process: F,
    ) -> &mut Self
    where
        T: ToString + Clone + 'a,
        T: Encode<'a, DB> + Type<DB>,
        F: FnOnce(T) -> T, // 处理值的闭包
    {
        if let Some(value) = param {
            let bind = bind_process(value);
            if !bind.to_string().trim().is_empty() {
                self.query_builder.push(condition_template);
                self.query_builder.push_bind(bind);
            }
        }
        self
    }

    pub fn build_count_condition<T, F>(
        &mut self,
        param: Option<T>,
        condition_template: &str,
        bind_process: F,
    ) -> &mut Self
    where
        T: ToString + Clone + 'a,
        T: Encode<'a, DB> + Type<DB>,
        F: FnOnce(T) -> T, // 处理值的闭包
    {
        if let Some(value) = param {
            let bind = bind_process(value);
            if let Some(count_builder) = self.count_builder.as_mut() {
                count_builder.push(condition_template);
                count_builder.push_bind(bind);
            }
        }
        self
    }
}
