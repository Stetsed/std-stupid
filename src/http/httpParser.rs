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

pub fn parseHTTPConnection(
    mut connectionData: Vec<u8>,
) -> Result<ParseReturnData, HttpReturnError> {
    // Find the / to find the method being used
    let MethodLocation = match findSubStringWithBytes(connectionData.clone(), b"/") {
        Ok(o) => o,
        Err(_) => {
            return Err(HttpReturnError::HttpServerError(HttpServerError::new(
                "Failed to find / to find method",
            )))
        }
    };

    let snip = match str::from_utf8(&connectionData[0..MethodLocation as usize - 1]) {
        Ok(o) => o,
        Err(e) => return Err(HttpReturnError::Utf8ParsingError(e)),
    };

    let RequestTypeGiven: RequestType = match snip {
        "GET" => RequestType::GET,
        "POST" => RequestType::POST,
        "HEAD" => RequestType::HEAD,
        "PUT" => RequestType::PUT,
        "CONNECT" => RequestType::CONNECT,
        "DELETE" => RequestType::DELETE,
        "TRACE" => RequestType::TRACE,
        "OPTIONS" => RequestType::OPTIONS,
        _ => RequestType::INVALID,
    };

    connectionData.drain(0..MethodLocation as usize);

    // Find the H to know the entire path
    let RequestPathLocation = match findSubStringWithBytes(connectionData.clone(), &[0x48]) {
        Ok(o) => o,
        Err(e) => return Err(HttpReturnError::SubStringError(e)),
    };

    let requestPathGiven: String =
        match str::from_utf8(&connectionData[0..RequestPathLocation as usize - 1]) {
            Ok(o) => o.to_string(),
            Err(e) => return Err(HttpReturnError::Utf8ParsingError(e)),
        };

    connectionData.drain(0..RequestPathLocation as usize);

    // Find the first CLRF
    let HTTPlocation = match findSubStringWithBytes(connectionData.clone(), &[0x0a]) {
        Ok(o) => o,
        Err(e) => return Err(HttpReturnError::SubStringError(e)),
    };

    let HTTPVersionGiven: f32 = match str::from_utf8(&connectionData[5..HTTPlocation as usize - 1])
    {
        Ok(o) => o.parse().unwrap(),
        Err(e) => return Err(HttpReturnError::Utf8ParsingError(e)),
    };

    let mut headerHashMap: HashMap<String, String> = HashMap::new();

    connectionData.drain(0..RequestPathLocation as usize + 1);

    while connectionData.len() >= 3 {
        let headerCLRFLocation =
            findSubStringWithBytes(connectionData.clone(), &[0x0a]).unwrap_or(0);

        let headerCLRFLocationGet: u32;
        let headerCLRFLocationDrain: usize;

        if headerCLRFLocation == 0 {
            headerCLRFLocationGet = connectionData.len() as u32;
            headerCLRFLocationDrain = connectionData.len();
        } else {
            headerCLRFLocationGet = headerCLRFLocation - 1;
            headerCLRFLocationDrain = (headerCLRFLocation + 1) as usize;
        }
        let headerSupplied = connectionData[0..headerCLRFLocationGet as usize].to_vec();

        let headerDoublePeriodLocation =
            findSubStringWithBytes(headerSupplied.as_slice().to_vec(), &[0x3a]).unwrap();

        let headerName =
            match str::from_utf8(&headerSupplied[0..headerDoublePeriodLocation as usize]) {
                Ok(o) => o.to_string(),
                Err(e) => return Err(HttpReturnError::Utf8ParsingError(e)),
            };

        match str::from_utf8(
            &headerSupplied
                [headerDoublePeriodLocation as usize + 2..headerCLRFLocationGet as usize],
        ) {
            Ok(o) => headerHashMap.insert(headerName, o.to_string()),
            Err(e) => return Err(HttpReturnError::Utf8ParsingError(e)),
        };

        connectionData.drain(0..headerCLRFLocationDrain);
    }

    drop(connectionData);
    #[cfg(debug_assertions)]
    println!("HTTP Version found is: {:?}", HTTPVersionGiven);
    #[cfg(debug_assertions)]
    println!("Request path found is: {:?}", requestPathGiven);
    #[cfg(debug_assertions)]
    println!("Request type found is: {:?}", RequestTypeGiven);
    #[cfg(debug_assertions)]
    for (header, content) in &headerHashMap {
        println!("Header: {header} : {content}");
    }
    Ok(ParseReturnData {
        httpVersion: HTTPVersionGiven,
        RequestType: RequestTypeGiven,
        requestPath: requestPathGiven,
        headers: headerHashMap,
    })
}
