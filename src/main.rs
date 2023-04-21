mod kv_store;
mod aof;
mod server;

fn main() {
    match server::start_server() {
        Ok(()) => { println!("Server exiting gracefully")},
        Err(err) => {panic!("[ERROR] starting server {}", err)},
    }
}
