use actix_files::Files;
use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;

mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("→ http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // logs: method, path, status, time
            .service(Files::new("/static", "./static").prefer_utf8(true))
            // pages
            .route("/", web::get().to(routes::index::index))
            
    })
    .workers(2)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
