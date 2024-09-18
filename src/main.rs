#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case)]

use errors_stupid::*;
use http_stupid::http_struct::*;
use http_stupid::HttpServer;
use standard_stupid::thread_manager::ThreadPool;
use tracing::Level;

fn main() -> Result<(), StdStupidError> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let IpAddressToUse = "0.0.0.0";
    let portTouse: u16 = 9182;

    let mut HttpServer = HttpServer::new(
        server_function::Debug,
        Some(IpAddressToUse),
        Some(portTouse),
        256,
    )?;

    HttpServer.setup_listener()?;

    HttpServer.start_listening()?;

    Ok(())
}
