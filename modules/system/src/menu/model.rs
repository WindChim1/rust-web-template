use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;
use tracing::info;

/// 菜单权限表对应的结构体
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct SysMenu {
    /// 菜单ID
    pub menu_id: i32, // 对应serial类型（本质为i32）

    /// 菜单名称
    pub menu_name: String, // varchar(50)，非空

    /// 父菜单ID
    #[serde(default = "default_parent_id")]
    pub parent_id: i32, // bigint，默认0

    /// 显示顺序
    #[serde(default = "default_order_num")]
    pub order_num: i32, // integer，默认0

    /// 路由地址
    #[serde(default = "default_string")]
    pub path: String, // varchar(200)，默认空字符串
    // 组件路径
    pub component: Option<String>,

    /// 路由参数
    pub query: Option<String>, // varchar(255)，可选字段

    /// 路由名称
    #[serde(default = "default_string")]
    pub route_name: String, // varchar(50)，默认空字符串

    /// 菜单类型（M目录 C菜单 F按钮）
    #[serde(default = "default_menu_type")]
    pub menu_type: String, // char，默认空字符

    /// 菜单状态（0正常 1停用）
    #[serde(default = "default_status")]
    pub status: String, // char，默认'0'

    #[serde(default = "default_string")]
    pub perm: String,
    /// 菜单图标
    #[serde(default = "default_icon")]
    pub icon: String, // varchar(100)，默认'#'

    /// 创建者
    #[serde(default = "default_string")]
    pub create_by: String, // varchar(64)，默认空字符串

    /// 创建时间
    #[serde(default = "default_create_time")]
    pub create_time: Option<OffsetDateTime>, // timestamp with time zone，默认当前时间

    /// 更新者
    #[serde(default = "default_string")]
    pub update_by: String, // varchar(64)，默认空字符串

    /// 更新时间
    pub update_time: Option<OffsetDateTime>, // timestamp with time zone，可选字段

    /// 备注
    #[serde(default = "default_string")]
    pub remark: String, // varchar(500)，默认空字符串
}

// 默认值函数，用于serde序列化/反序列化时提供默认值
fn default_parent_id() -> i32 {
    0
}

fn default_order_num() -> i32 {
    0
}

fn default_string() -> String {
    String::new()
}

fn default_menu_type() -> String {
    String::new()
}

fn default_status() -> String {
    "0".to_string()
}

fn default_icon() -> String {
    "#".to_string()
}

fn default_create_time() -> Option<OffsetDateTime> {
    Some(OffsetDateTime::now_utc())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MenuTreeVo {
    #[serde(flatten)]
    pub menu: SysMenu,
    pub children: Vec<MenuTreeVo>,
}

impl MenuTreeVo {
    pub fn build_menu_tree(all_menus: Vec<SysMenu>) -> Vec<Self> {
        info!("build menu  tree  from menu list");
        //1.获取所有root 菜单
        let roots: Vec<&SysMenu> = all_menus.iter().filter(|m| m.parent_id == 0).collect();
        //递归构建菜单树
        roots
            .into_iter()
            .map(|menu| Self::build_inner(menu, &all_menus))
            .collect::<Vec<_>>()
    }

    fn build_children_for_menu(parent: &mut Self, all_menus: &Vec<SysMenu>) {
        all_menus
            .iter()
            .filter(|menu| menu.parent_id == parent.menu.menu_id)
            .for_each(|menu| {
                let child = Self::build_inner(menu, all_menus);
                parent.children.push(child);
            });
    }
    fn build_inner(menu: &SysMenu, all_menus: &Vec<SysMenu>) -> MenuTreeVo {
        let mut child = MenuTreeVo {
            menu: menu.clone(),
            children: Vec::new(),
        };
        Self::build_children_for_menu(&mut child, all_menus);
        child
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MenuDTO {
    pub menu_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub menu_name: String,
    pub order_num: Option<i32>,
    pub path: Option<String>,
    pub component: Option<String>,
    pub menu_type: String,
    pub status: String,
    pub perms: Option<String>,
    pub icon: Option<String>,
    pub remark: Option<String>,
}

/// 路由显示信息 VO
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RouterVo {
    pub name: String,
    pub path: String,
    //表示顶级菜单点击后不重定向
    pub redirect: Option<String>,
    // 对应加载的 Vue 组件路径
    pub component: String,
    // alwaysShow: true 确保即使只有一个子菜单，父级也会显示
    pub always_show: Option<bool>,
    pub meta: MetaVo,
    // 子菜单
    pub children: Option<Vec<RouterVo>>,
}

/// 路由的 meta 信息 VO
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetaVo {
    pub title: String,
    pub icon: String,
}

impl RouterVo {
    pub fn build_from_menu_tree(menu_tree: Vec<MenuTreeVo>) -> Vec<Self> {
        menu_tree
            .into_iter()
            .map(|menu_tree| {
                let menu = &menu_tree.menu;
                let mut router = Self {
                    name: Self::get_rout_name(menu),
                    path: Self::get_root_path(menu),
                    component: Self::get_component(menu),
                    redirect: None,
                    always_show: None,
                    meta: MetaVo {
                        title: menu.menu_name.clone(),
                        icon: menu.menu_name.clone(),
                    },
                    children: None,
                };
                let has_children = menu_tree.children.is_empty();

                if has_children {
                    // 1. 如果有子菜单，递归构建
                    router.children = Some(Self::build_from_menu_tree(menu_tree.children));
                    // 确保父级 component 是 Layout
                    if menu.parent_id == 0 {
                        router.component = "Layout".to_string();
                    }
                }

                // 2.RuoYi 逻辑：处理一级菜单 (类型C)
                // 如果一个菜单是顶级的(parent_id=0)，类型是C，并且它没有子菜单，
                // 它需要被包装在一个Layout中，并拥有一个path为'index'的子路由。
                if menu.parent_id == 0 && menu.menu_type == "C" && !has_children {
                    // a. 父路由（外壳）的 component 必须是 Layout
                    router.component = "Layout".to_string();

                    // b. 创建一个代表实际页面的子路由
                    let child_router = RouterVo {
                        // 1: 子路由的 path 硬编码为 "index"
                        path: "index".to_string(),
                        // 2: 子路由的 name 根据新 path "index" 生成，得到 "Index"
                        name: Self::get_rout_name(&SysMenu {
                            path: "index".to_string(),
                            ..Default::default()
                        }),
                        component: Self::get_component(menu), // 真实组件路径
                        meta: router.meta.clone(),            // meta 信息继承自逻辑父级
                        // 其他字段为默认值
                        redirect: None,
                        always_show: None,
                        children: None,
                    };

                    // c. 3: 父路由的 redirect 指向子路由的完整路径
                    router.redirect = Some(format!("{}/index", router.path));
                    router.children = Some(vec![child_router]);
                }
                // 3. 处理目录 (类型M)
                else if menu.menu_type == "M" {
                    router.redirect = Some("noRedirect".to_string());
                    router.always_show = Some(true);
                    // 顶级目录的 component 也应该是 Layout
                    if menu.parent_id == 0 {
                        router.component = "Layout".to_string();
                    }
                }

                router
            })
            .collect()
    }
    ///获取路由名称
    fn get_rout_name(menu: &SysMenu) -> String {
        let path = if menu.path.starts_with(':') {
            menu.path.replacen(':', "", 1)
        } else {
            menu.path.clone()
        };

        let mut c = path.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
    ///获取路由地址
    fn get_root_path(menu: &SysMenu) -> String {
        let mut path = menu.path.clone();
        // 2. 合并核心条件：只要是顶级节点（parent_id=0），就确保路径以 "/" 开头
        if menu.parent_id == 0 && !path.starts_with('/') {
            path = format!("/{}", path);
        }
        path
    }
    ///获取组件信息
    fn get_component(menu: &SysMenu) -> String {
        if let Some(comp_str) = &menu.component
            && !comp_str.is_empty()
        {
            return comp_str.clone();
        }

        if menu.parent_id == 0 && menu.menu_type == "M" {
            // 顶级目录，使用 ParentView
            "ParentView".to_string()
        } else {
            // 对于其他情况（如顶级菜单C，或子菜单C），component 字段为空时，就让它为空
            // 这样 build_routers 函数就能知道需要特殊处理
            "".to_string()
        }
    }
}
