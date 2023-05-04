mod aof;
mod cli;
mod kv_store;
mod server;

use threadpool::ThreadPool;

fn main() {
    let pool = ThreadPool::new(5);

    let server = || match server::start_server() {
        Ok(()) => {
            println!("Server exiting gracefully")
        }
        Err(err) => {
            panic!("[ERROR] starting server {}", err)
        }
    };

    let cli = || {
        cli::start();
    };

    pool.execute(server);
    pool.execute(cli);
    pool.join();
}
