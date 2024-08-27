#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case)]
mod random_stupid;

use errors_stupid::*;
use http_stupid::http_struct::*;
use http_stupid::HttpServer;

fn main() -> Result<(), StdStupidError> {
    let IpAddressToUse: String = "0.0.0.0".to_string();
    let portTouse: u16 = 9182;

    let mut HttpServer = HttpServer::new(
        server_function::ServeFile,
        Some(IpAddressToUse),
        Some(portTouse),
    )?;

    HttpServer.setup_listener()?;

    HttpServer.start_listening()?;

    Ok(())
}
