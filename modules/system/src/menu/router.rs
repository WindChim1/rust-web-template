use salvo::Router;

use crate::menu::handle::{add, delete, get_detail, get_menu_tree, list, update};

pub fn init_router() -> Router {
    Router::new()
        .path("menu")
        .push(Router::with_path("add").post(add))
        .push(Router::with_path("update").put(update))
        .push(Router::with_path("delete/{id}").delete(delete))
        .push(Router::with_path("list").get(list))
        .push(Router::with_path("{id}").get(get_detail))
        .push(Router::with_path("menu_tree").get(get_menu_tree))
}
