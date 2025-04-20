use actix_web::{web::{Data, Json}, HttpResponse, Responder};
use crate::connections::db_connection::DBPool;
use super::{dto::CreateTask, repo::TodoRepo};

pub struct TodoService {}

impl TodoService {

    pub async fn create_task(db:Data<DBPool>, req:Json<CreateTask>) -> impl Responder {
        match db.get() {
            Ok(mut conn) => {
                match TodoRepo::crate_task(&mut *conn, req.into_inner()) {
                    Ok(task) => {
                        HttpResponse::Ok().json(task)
                    },
                    Err(e) =>{
                        HttpResponse::BadRequest().json(e.to_string())
                    },
                }
            },
            Err(e) => {
                HttpResponse::InternalServerError().json(e.to_string())
            },
        }
    }

    pub async fn list_task(db: Data<DBPool>) -> impl Responder {
        match db.get() {
            Ok(mut conn) => {
                match TodoRepo::list_task(&mut *conn) {
                    Ok(tasks) => {
                        HttpResponse::Ok().json(tasks)
                    },
                    Err(e) => {
                        HttpResponse::BadRequest().json(e.to_string())
                    },
                }
            },
            Err(e) => {
                HttpResponse::InternalServerError().json(e.to_string())
            },
        }
    }
}