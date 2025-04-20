use diesel::{prelude::Insertable, Selectable};
use serde::Deserialize;

#[derive(Debug,Insertable, Selectable,Deserialize)]
#[diesel(table_name=crate::schema::my_task)]
pub struct CreateTask {
    pub task_name : String,
    pub task_description: String
}