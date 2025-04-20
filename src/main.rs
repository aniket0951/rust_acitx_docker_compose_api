use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer};
use connections::db_connection;
use todo::routes::to_do_routes;

mod connections;
mod todo;
mod schema;

#[actix_web::get("/index")]
async fn index(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(include_str!("../static/index.html"))
}

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
            .service(actix_files::Files::new("/static", "static").show_files_listing())
            .service(index)
            .service(to_do_routes())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await

}
