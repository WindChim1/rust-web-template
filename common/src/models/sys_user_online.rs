use serde::Serialize;
use time::OffsetDateTime;

/// 在线用户信息实体，用于缓存和前端展示。
/// 这个结构体不对应数据库表，而是内存中的数据模型。
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SysUserOnline {
    /// 会话编号 (我们使用 JWT 的 jti)
    pub token_id: String,
    /// 用户名称
    pub user_name: String,
    /// 登录IP地址
    pub ipaddr: Option<String>,
    /// 登录地点
    pub login_location: Option<String>,
    /// 浏览器类型
    pub browser: Option<String>,
    /// 操作系统
    pub os: Option<String>,
    /// 登录时间 (使用带时区的时间，序列化为 "2023-07-03T10:30:00+08:00" 格式)
    pub login_time: OffsetDateTime,

    /// 隐藏字段：存储完整的 JWT，用于实现“强退”功能。 这个字段不返回给前端，所以使用 #[serde(skip_serializing)]
    #[serde(skip_serializing)]
    pub token: String,
}

#[cfg(test)]
mod test {
    use time::OffsetDateTime;
    #[test]
    fn time_test() -> anyhow::Result<()> {
        let now_local = OffsetDateTime::now_local()?;
        println!("{now_local}");
        assert_eq!(1, 2);
        Ok(())
    }
}
