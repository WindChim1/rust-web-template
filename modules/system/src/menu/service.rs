use common::AppResult;
use sqlx::PgPool;
use tracing::info;

use crate::menu::model::MenuDTO;

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
