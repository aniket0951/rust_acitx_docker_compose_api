use std::env;
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> DBPool {
    dotenv::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("faild to get DB Url");
    let db = ConnectionManager::new(db_url);

    r2d2::Pool::builder().max_size(5).build(db).expect("faild to create db pool")
}