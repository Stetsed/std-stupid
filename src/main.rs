#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case)]

use errors_stupid::*;
use http_stupid::http_struct::*;
use http_stupid::HttpServer;

fn main() -> Result<(), StdStupidError> {
    let IpAddressToUse = "0.0.0.0";
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
