mod aof;
mod kv_store;
mod server;

use clap::Parser;

/// A simple key/value store
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Port to start the server on
    #[arg(short, long, default_value_t = 8081)]
    port: u16,
}

fn main() {
    let args = Args::parse();

    match server::start_server(args.port) {
        Ok(()) => {
            println!("[INFO] Server exiting gracefully")
        }
        Err(err) => {
            panic!("[ERROR] starting server {}", err)
        }
    };
}
