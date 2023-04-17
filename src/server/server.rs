use super::routes::*;
use actix_web::{App, HttpServer};

#[actix_web::main] // or #[tokio::main]
pub async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_key)
            .service(set_key)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}