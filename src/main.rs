#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case)]
#![warn(missing_docs)]
mod random_stupid;

use errors_stupid::*;
use http_stupid::httpStruct::*;
use http_stupid::HttpServer;

fn main() -> Result<(), StdStupidError> {
    let IpAddressToUse: String = "127.0.0.1".to_string();
    let portTouse: u16 = 9182;

    let mut HttpServer = HttpServer::new(
        ServerFunction::ServeFile,
        Some(IpAddressToUse),
        Some(portTouse),
    )?;

    HttpServer.setupListener()?;

    HttpServer.startListening()?;

    Ok(())
}
