#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case)]
mod errors_stupid;
mod http_stupid;
mod random_stupid;
mod standard_stupid;

use errors_stupid::*;
use http_stupid::httpCompose::*;
use http_stupid::httpStruct::*;
use http_stupid::*;

fn main() -> Result<(), StdStupidError> {
    let IpAddressToUse: String = "127.0.0.1".to_string();
    let portTouse: u16 = 9182;

    let mut HttpServer = HttpServer::new(
        ServerFunction::ServeFile,
        Some(IpAddressToUse),
        Some(portTouse),
    )?;

    HttpServer.setupListener();

    HttpServer.startListening()?;

    Ok(())
}
