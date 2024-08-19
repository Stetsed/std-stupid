use std::{
    borrow::Borrow,
    collections::HashMap,
    error::Error,
    fmt::Debug,
    io::{self, BufWriter, Write},
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    str,
    time::SystemTime,
};

use crate::{
    errors_stupid::HttpServerError, findSubStringWithString, http::*, httpStruct::*,
    standard_stupid::findSubStringWithBytes, HttpReturnError,
};

pub fn composeHttpResponse(
    HttpServerFunction: ServerFunction,
    parseReturnData: ParseReturnData,
) -> Vec<u8> {
    if ServerFunction::Debug == HttpServerFunction {
        let mut responseBody: Vec<u8> = Vec::new();
        responseBody.extend_from_slice(b"<html>");

        for i in parseReturnData.headers {
            let header = format!(
                "Header Name: {} <br/>Header Content: {} <br/><br/>",
                i.0, i.1
            );
            responseBody.extend_from_slice(header.as_bytes())
        }

        responseBody.extend_from_slice(b"<html/>");

        let mut responseVector: Vec<u8> = b"HTTP/1.1 200 OK\r\n".to_vec();

        responseVector.extend_from_slice(b"Server: std-stupid-http\r\n");

        responseVector.extend_from_slice(b"Content-Type: text/html\r\n");

        responseVector.extend_from_slice(b"Accept-Ranges: bytes\r\n");

        responseVector.extend_from_slice(b"Connection: close\r\n");

        responseVector.extend_from_slice(b"Cache-Control: no-cache\r\n");

        responseVector
            .extend_from_slice(format!("Content-Length: {} \r\n", responseBody.len()).as_bytes());

        responseVector.extend_from_slice(b"\r\n");

        responseVector.extend(responseBody);

        responseVector
    } else {
        todo!()
    }
}

fn addHeader<T: AsRef<String>>(header: T, mut vector: Vec<u8>) -> Vec<u8> {
    vector.extend_from_slice(header.as_ref().as_bytes());

    vector
}
