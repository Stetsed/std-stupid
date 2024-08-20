#![feature(addr_parse_ascii, ip, tcplistener_into_incoming)]
#![allow(non_snake_case, unused_imports, unused_must_use)]
mod errors_stupid;
mod http_stupid;
mod random_stupid;
mod standard_stupid;

use core::fmt;
use std::default::Default;
use std::fmt::Display;
use std::fs::File;
use std::io::{prelude::*, stdin, BufReader, BufWriter, Error};
use std::net::Ipv4Addr;
use std::process::{exit, Termination};
use std::time::Duration;
use std::{error, io};

use errors_stupid::*;
use http_stupid::httpCompose::*;
use http_stupid::httpParser::*;
use http_stupid::httpStruct::*;
use http_stupid::*;
use random_stupid::*;
use standard_stupid::*;

fn main() -> Result<(), StdStupidError> {
    let IpAddressToUse: String = "127.0.0.1".to_string();
    let portTouse: u16 = 9182;

    let mut HttpServer = HttpServer::new(
        ServerFunction::ServeFile,
        Some(IpAddressToUse),
        Some(portTouse),
    )?;

    HttpServer.setupListener();

    HttpServer.startListening();

    Ok(())
}
