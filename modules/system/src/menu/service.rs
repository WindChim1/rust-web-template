use common::AppResult;
use sqlx::PgPool;
use tracing::info;

use crate::menu::model::{MenuDTO, MenuTreeVo, SysMenu};

/// 新增菜单
pub async fn add_menu(db: &PgPool, menu: MenuDTO) -> AppResult<u8> {
    info!("[SERVICE] Entering add_menu with  dto: {:?}", menu);
    let MenuDTO {
        parent_id,
        menu_name,
        order_num,
        path,
        menu_type,
        status,
        icon,
        remark,
        ..
    } = menu;
    let result = sqlx::query!(
            r#"
                INSERT INTO sys_menu (menu_name, parent_id, order_num, path, menu_type,  status,  icon, remark, create_by, create_time)
                VALUES ($1, $2, $3,$4, $5, $6, $7, $8, 'admin', NOW())

        "#,
            menu_name,
            parent_id,
            order_num,
            path,
            menu_type,
            status,
            icon,
            remark
        ).execute(db)
            .await?;
    Ok(result.rows_affected() as u8)
}

///修改菜单
pub async fn update_menu(db: &PgPool, menu: MenuDTO) -> AppResult<u64> {
    let result = sqlx::query!(
        r#"
            UPDATE sys_menu
            SET menu_name = $1, parent_id = $2, order_num = $3, path = $4, component = $5,  menu_type = $6,  status = $7,  icon = $8, remark = $9, update_by = 'admin', update_time = NOW()
            WHERE menu_id = $10
        "#,
        menu.menu_name, menu.parent_id, menu.order_num, menu.path, menu.component,
        menu.menu_type,  menu.status,  menu.icon, menu.remark, menu.menu_id
    )
        .execute(db)
        .await?;
    Ok(result.rows_affected())
}

/// 删除菜单
pub async fn delete_menu_by_id(db: &PgPool, menu_id: i32) -> AppResult<u64> {
    // RuoYi 删除菜单时会检查是否有子菜单，我们暂时简化
    let result = sqlx::query!("DELETE FROM sys_menu WHERE menu_id = $1", menu_id)
        .execute(db)
        .await?;
    Ok(result.rows_affected())
}

///根据用户id查询目录与菜单
pub async fn select_menu_tree_by_user_id(db: &PgPool, user_id: i32) -> AppResult<Vec<MenuTreeVo>> {
    info!(
        "[SERVICE] Entering  select menu tree by user id: {:?}",
        user_id
    );
    //1.检查是否为管理员
    let menus = if user_id == 1 {
        info!("[AUTH] User is admin (user_id=1), fetching all enabled menus.");
        select_all_menu_list(db).await?
    } else {
        sqlx::query_as(
            "
                select sm.*
                from sys_menu sm
                        left join sys_role_menu srm on sm.menu_id = srm.menu_id
                        left join  sys_role  sr  on sr.role_id = srm.role_id
                        left join sys_user_role sur on sur.user_id = sr.role_id
                where user_id = $1
                AND sm.menu_type IN ('M', 'C')
                AND sm.status = '0'
                AND sr.status = '0'
                ORDER BY sm.parent_id, sm.order_num",
        )
        .bind(user_id)
        .fetch_all(db)
        .await?
    };

    info!(
        "[DB_RESULT] Found {} menus for user_id {}.",
        menus.len(),
        user_id
    );
    Ok(MenuTreeVo::build_menu_tree(menus))
}

///查询目录与菜单
pub async fn select_all_menu_list(db: &PgPool) -> AppResult<Vec<SysMenu>> {
    let menus = sqlx::query_as(
        "SELECT * FROM sys_menu WHERE menu_type IN ('M', 'C') ORDER BY parent_id, order_num",
    )
    .fetch_all(db)
    .await?;
    Ok(menus)
}

/// 根据菜单ID查询菜单详情
pub async fn select_menu_by_id(db: &PgPool, id: i32) -> AppResult<SysMenu> {
    let menu = sqlx::query_as("SELECT * FROM sys_menu WHERE id  = $1")
        .bind(id)
        .fetch_one(db)
        .await?;
    Ok(menu)
}

/// 查询所有菜单，用于构建菜单选择树
pub async fn select_menu_list_for_treeselect(db: &PgPool) -> AppResult<Vec<SysMenu>> {
    // 关键区别：这里需要获取所有类型的菜单（M, C, F），而不仅仅是 M 和 C
    // 并且只选择状态正常的菜单
    info!("[SERVICE] Entering select_menu_list_for_treeselect");
    let menus =
        sqlx::query_as("SELECT * FROM sys_menu WHERE status = '0' ORDER BY parent_id, order_num")
            .fetch_all(db)
            .await?;
    info!("[DB_RESULT] Found {} menus for treeselect.", menus.len());
    Ok(menus)
}

#[cfg(test)]
mod test {
    use framework::db::DBPool;

    use crate::menu::{model::RouterVo, service};

    #[tokio::test]
    async fn select_menu_tree_by_user_id_test() -> anyhow::Result<()> {
        let url = "postgres://postgres:Sky%402024@172.16.100.200:25432/project?options=-c%20search_path%3Dsky_website";
        let db = DBPool::inint(url).await?;
        let tree = service::select_menu_tree_by_user_id(db, 1).await?;
        // println!("{tree:?}");
        let router_tree = RouterVo::build_from_menu_tree(tree);
        println!("{router_tree:?}");
        assert_eq!(1, 2);
        Ok(())
    }
}
