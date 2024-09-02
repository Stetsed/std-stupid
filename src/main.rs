#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case)]

use async_http_stupid::http_struct::*;
use async_http_stupid::HttpServer;
use errors_stupid::*;

#[async_std::main]
async fn main() -> Result<(), StdStupidError> {
    let IpAddressToUse: String = "0.0.0.0".to_string();
    let portTouse: u16 = 9182;

    let mut HttpServer = HttpServer::new(
        server_function::Debug,
        Some(IpAddressToUse),
        Some(portTouse),
    )?;

    HttpServer.setup_listener().await?;

    HttpServer.start_listening().await?;

    Ok(())
}
