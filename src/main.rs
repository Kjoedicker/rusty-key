mod kv_store;
mod aof;
mod server;

fn main() {
    server::start_server();
}
