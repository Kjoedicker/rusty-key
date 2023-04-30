use crate::kv_store::KEY_VALUE_STORE;
use actix_web::{get, web, Responder, put,  HttpResponse, delete};
use serde::Deserialize;

#[get("/{key}")]
async fn get_key(key: web::Path<String>) -> HttpResponse {
    println!("Received GET request for Key: {}", key);

    let mut teller = KEY_VALUE_STORE.lock().unwrap(); 
    
    match teller.get(&key) {
        Some(value) => {
            HttpResponse::Ok().json(value)
        },
        None => HttpResponse::NotFound().body("Key not found")
    }
}

#[derive(Deserialize, Debug)]
struct Payload {
    value: String,
}

#[put("/{key}")]
async fn set_key(params: web::Path<String>, payload: web::Json<Payload>) -> impl Responder {
    let key = params.into_inner();
    let value = payload.value.to_owned();
    
    println!("Received SET request Key: {} Value: {:?}", key, value);

    let mut teller = KEY_VALUE_STORE.lock().unwrap();

    match teller.set(key, value, Some(true)) {
        // TODO: revisit this when its a little 
        // more clear when an issue occurs
        _ => HttpResponse::Created()
    }
}

#[delete("/{key}")]
async fn delete_key(params: web::Path<String>) -> impl Responder {
    let key = params.into_inner();
    
    println!("Received DELETE request Key: {}", key);

    let mut teller = KEY_VALUE_STORE.lock().unwrap();

    match teller.delete(&key, Some(true)) {
        Some(_) => {
            HttpResponse::Ok().body("Key deleted")
        },
        None => {
            HttpResponse::NotFound().body("Key not found")
        }
    }
}