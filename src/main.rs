use actix_web::{middleware::Logger, web, App, HttpServer};
use connections::db_connection;
use todo::routes::to_do_routes;

mod connections;
mod todo;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let con = db_connection::connect();
    
    let pool_data = web::Data::new(con);
    println!("server is ready...");
    HttpServer::new(move || {
        App::new()
            .app_data(pool_data.clone())
            .wrap(Logger::default())
            .service(to_do_routes())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

}
