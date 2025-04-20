use diesel::{PgConnection, QueryResult, RunQueryDsl};
use crate::schema::my_task::dsl::*;
use super::{dto::CreateTask, model::MyTask};


pub struct TodoRepo{}

impl TodoRepo {
    pub fn crate_task(db:&mut PgConnection, req:CreateTask) -> QueryResult<MyTask> {
        diesel::insert_into(my_task)
            .values(&req)
            .get_result(db)
    }

    pub fn list_task(db:&mut PgConnection) -> QueryResult<Vec<MyTask>> {
        my_task.load::<MyTask>(db)
    }
}