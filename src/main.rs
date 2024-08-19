#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case, unused_imports, unused_must_use)]
mod errors_stupid;
mod http;
mod random_stupid;
mod standard_stupid;

use core::fmt;
use std::default::Default;
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, stdin, Error};
use std::net::Ipv4Addr;
use std::process::{exit, Termination};
use std::{error, io};

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

    for stream in HttpServer.TcpListener.as_ref().unwrap().incoming() {
        match stream {
            Ok(mut o) => {
                let mut recieveBuffer: Vec<u8> = Vec::new();
                o.read_to_end(&mut recieveBuffer).unwrap();

                parseHTTPConnection(recieveBuffer);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => panic!("wtf"),
        }
    }

    Ok(())
}
