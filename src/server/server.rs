use crate::kv_store::KEY_VALUE_STORE;

use super::routes::*;
use actix_web::{App, HttpServer};

pub fn startup_tasks() {
    let mut store = KEY_VALUE_STORE.lock().unwrap();
    store.process_actions();
}

#[actix_web::main]
pub async fn start_server() -> std::io::Result<()> {
    startup_tasks();

    HttpServer::new(|| {
        App::new()
            .service(get_key)
            .service(set_key)
            .service(delete_key)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}