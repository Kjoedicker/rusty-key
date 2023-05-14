use crate::kv_store::KEY_VALUE_STORE;

use super::routes::*;
use actix_web::{App, HttpServer};

pub fn startup_tasks() {
    let mut store = KEY_VALUE_STORE.lock().unwrap();
    store.sync_aof();
}

#[actix_web::main]
pub async fn start_server(port: u16) -> std::io::Result<()> {
    startup_tasks();

    println!("[STARTUP] server on port: {}", port);

    HttpServer::new(|| {
        App::new()
            .service(get_key)
            .service(set_key)
            .service(delete_key)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
