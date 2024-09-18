#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case)]

use errors_stupid::*;
use http_stupid::http_struct::*;
use http_stupid::HttpServer;
use standard_stupid::thread_manager::ThreadPool;
use tracing::Level;

fn main() -> Result<(), StdStupidError> {
    let pool = ThreadPool::new(8);
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    for i in 0..1000 {
        pool.execute(move || {
            println!("{}", i);
        });
    }

    Ok(())
}
