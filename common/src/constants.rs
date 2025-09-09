use strum::{AsRefStr, EnumString};

/// # 系统权限枚举
///
/// 定义了系统中所有可用的API权限点。
/// - `#[strum(serialize = "...")]`: 这个属性将每个枚举成员与其在数据库中对应的
#[derive(Debug, Clone, Copy, AsRefStr, EnumString)]
pub enum Permission {
    // 用户管理
    #[strum(serialize = "system:user:list")]
    UserList,
    #[strum(serialize = "system:user:query")]
    UserQuery,
    #[strum(serialize = "system:user:add")]
    UserAdd,
    #[strum(serialize = "system:user:edit")]
    UserEdit,
    #[strum(serialize = "system:user:remove")]
    UserRemove,
    #[strum(serialize = "system:user:resetPwd")]
    UserResetPwd,

    // 角色管理
    #[strum(serialize = "system:role:list")]
    RoleList,
    #[strum(serialize = "system:role:query")]
    RoleQuery,
    #[strum(serialize = "system:role:add")]
    RoleAdd,
    #[strum(serialize = "system:role:edit")]
    RoleEdit,
    #[strum(serialize = "system:role:remove")]
    RoleRemove,

    // 菜单管理
    #[strum(serialize = "system:menu:list")]
    MenuList,
    #[strum(serialize = "system:menu:query")]
    MenuQuery,
    #[strum(serialize = "system:menu:add")]
    MenuAdd,
    #[strum(serialize = "system:menu:edit")]
    MenuEdit,
    #[strum(serialize = "system:menu:remove")]
    MenuRemove,
}
