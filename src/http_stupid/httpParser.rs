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
    errors_stupid::HttpServerError, findSubStringWithString, httpStruct::*, http_stupid::*,
    standard_stupid::findSubStringWithBytes,
};

pub fn parseHTTPConnection(mut connectionData: Vec<u8>) -> Result<ParseReturnData, StdStupidError> {
    // Find the / to find the method being used
    let MethodLocation = findSubStringWithBytes(connectionData.clone(), b"/")?;

    let RequestTypeRaw = str::from_utf8(&connectionData[0..MethodLocation as usize - 1])?;

    let HttpRequestTypeGiven: HttpRequestType = parseHttpRequestType(RequestTypeRaw);

    connectionData.drain(0..MethodLocation as usize);

    // Find the H to know the entire path
    let RequestPathLocation = findSubStringWithBytes(connectionData.clone(), &[0x48])?;

    let requestPathGiven: String =
        str::from_utf8(&connectionData[0..RequestPathLocation as usize - 1])?.to_string();

    connectionData.drain(0..RequestPathLocation as usize);

    // Find the first CLRF
    let HTTPlocation = findSubStringWithBytes(connectionData.clone(), &[0x0a])?;

    let HTTPVersionGiven: f32 =
        str::from_utf8(&connectionData[5..HTTPlocation as usize - 1])?.parse::<f32>()?;

    let mut headerHashMap: HashMap<String, String> = HashMap::new();

    connectionData.drain(0..HTTPlocation as usize + 1);

    while connectionData.len() > 2 {
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
            findSubStringWithBytes(headerSupplied.as_slice().to_vec(), b":").unwrap();

        let headerName =
            str::from_utf8(&headerSupplied[0..headerDoublePeriodLocation as usize])?.to_string();

        let headerContent = str::from_utf8(
            &headerSupplied
                [headerDoublePeriodLocation as usize + 2..headerCLRFLocationGet as usize],
        )?;
        headerHashMap.insert(headerName, headerContent.to_string());

        connectionData.drain(0..headerCLRFLocationDrain);
    }

    drop(connectionData);
    #[cfg(debug_assertions)]
    println!("HTTP Version found is: {:?}", HTTPVersionGiven);
    #[cfg(debug_assertions)]
    println!("Request path found is: {:?}", requestPathGiven);
    #[cfg(debug_assertions)]
    println!("Request type found is: {:?}", HttpRequestTypeGiven);
    #[cfg(debug_assertions)]
    for (header, content) in &headerHashMap {
        println!("Header: {header} : {content}");
    }
    Ok(ParseReturnData {
        httpVersion: HTTPVersionGiven,
        HttpRequestType: HttpRequestTypeGiven,
        requestPath: requestPathGiven,
        headers: headerHashMap,
    })
}

pub fn parseHttpRequestType<T: AsRef<str>>(toParse: T) -> HttpRequestType {
    match toParse.as_ref() {
        "GET" => HttpRequestType::GET,
        "POST" => HttpRequestType::POST,
        "HEAD" => HttpRequestType::HEAD,
        "PUT" => HttpRequestType::PUT,
        "CONNECT" => HttpRequestType::CONNECT,
        "DELETE" => HttpRequestType::DELETE,
        "TRACE" => HttpRequestType::TRACE,
        "OPTIONS" => HttpRequestType::OPTIONS,
        _ => HttpRequestType::INVALID,
    }
}
