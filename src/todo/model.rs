use diesel::{prelude::{AsChangeset, Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize,Queryable, Selectable, Insertable,AsChangeset)]
#[diesel(table_name=crate::schema::my_task)]
pub struct MyTask {
    pub id: i32 ,
    pub task_name: String,
    pub task_description: String
}