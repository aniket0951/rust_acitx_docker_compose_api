use actix_web::{web, Scope};
use super::service::TodoService;

pub fn to_do_routes() -> Scope {
    web::scope("/todo")
        .route("/create", web::post().to(TodoService::create_task))
        .route("/list", web::get().to(TodoService::list_task))
}