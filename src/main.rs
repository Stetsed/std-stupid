#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case, unused_imports, unused_must_use)]
mod errors_stupid;
mod http;
mod random_stupid;
mod standard_stupid;

use core::fmt;
use std::default::Default;
use std::error;
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, stdin, Error};
use std::net::Ipv4Addr;
use std::process::{exit, Termination};

use errors_stupid::*;
use http::*;
use random_stupid::*;
use standard_stupid::*;

fn main() -> Result<(), Error> {
    let IpAddressToUse: String = "127.0.0.1".to_string();
    let portTouse: u16 = 9182;

    let HttpServerSetup = HttpServer::new(Some(IpAddressToUse), Some(portTouse));

    let mut HttpServer: HttpServer;

    match HttpServerSetup {
        Ok(_) => HttpServer = HttpServerSetup.unwrap(),
        Err(e) => panic!("{e:?}"),
    };
    HttpServer.setupListener();

    let mut connection = HttpServer.acceptConnection().unwrap();

    let mut recieveBuffer = Vec::new();
    let b = connection.0.read_to_end(&mut recieveBuffer).unwrap();

    // println!("Bytes read: {}", b);
    // print!("{}", String::from_utf8(recieveBuffer.clone()).unwrap());
    //
    // for i in 0..recieveBuffer.len() {
    //     println!(
    //         "Raw data: {} Char: {}",
    //         recieveBuffer[i],
    //         char::from(recieveBuffer[i])
    //     );
    // }

    // let mut buffer = "19281828821".as_bytes();
    //
    // connection.0.write_all(buffer);

    HttpServer.parseConnection(recieveBuffer);

    Ok(())
}
