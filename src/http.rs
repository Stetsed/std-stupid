use std::{
    borrow::Borrow,
    collections::HashMap,
    error::Error,
    fmt::Debug,
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    str,
};

use crate::{
    errors_stupid::HttpServerError, findSubStringWithString,
    standard_stupid::findSubStringWithBytes, HttpReturnError,
};

#[derive(Debug)]
pub struct HttpServer {
    ListeningAddress: Ipv4Addr,
    pub TcpListener: Option<TcpListener>,
    Port: u16,
}

#[derive(Debug)]
pub enum connectionReturn {
    TcpStream,
    SocketAddr,
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum requestType {
    GET,
    POST,
    OPTIONS,
    HEAD,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
    INVALID,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct parseReturnData {
    httpVersion: f32,
    requestType: requestType,
    requestPath: String,
    headers: HashMap<String, String>,
}

impl HttpServer {
    pub fn new(
        IpAddressGiven: Option<String>,
        PortGiven: Option<u16>,
    ) -> Result<Self, HttpServerError> {
        // Attempt to get port given, if not given then use default port 8080.
        let PortToUse: u16 = PortGiven.unwrap_or(8080);

        let IpAddressToUse: Ipv4Addr = IpAddressGiven
            .unwrap_or("127.0.0.1".parse().unwrap())
            .parse()
            .unwrap();

        // Checks if the address is multicast/Documentation range, if yes rejects.
        if IpAddressToUse.is_multicast() {
            return Err::<Self, HttpServerError>(HttpServerError {
                source: String::from("IP Address Given is designated as Multicast"),
            });
        }
        if IpAddressToUse.is_documentation() {
            return Err::<Self, HttpServerError>(HttpServerError {
                source: String::from("IP Address Given is designated as Documentation IP range."),
            });
        }

        Ok(Self {
            ListeningAddress: IpAddressToUse,
            TcpListener: None,
            Port: PortToUse,
        })
    }

    pub fn setupListener(&mut self) {
        let socketAddress: SocketAddrV4 = SocketAddrV4::new(self.ListeningAddress, self.Port);
        let listenerReturn = TcpListener::bind(socketAddress);

        match listenerReturn {
            Ok(o) => {
                o.set_nonblocking(true);
                self.TcpListener = Some(o)
            }
            Err(e) => panic!("{e:?}"),
        }
    }

    pub fn acceptConnection(&mut self) -> Result<(TcpStream, SocketAddr), HttpServerError> {
        let TcpConnection = self.TcpListener.as_ref().unwrap().accept();

        match TcpConnection {
            Ok(v) => Ok(v),
            Err(e) => panic!("{e:?}"),
        }
    }
}

pub fn parseHTTPConnection(
    mut connectionData: Vec<u8>,
) -> Result<parseReturnData, HttpReturnError> {
    // Find the / to find the method being used
    let MethodLocation = match findSubStringWithString(connectionData.clone(), "/".to_string()) {
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

    let requestTypeGiven: requestType = match snip {
        "GET" => requestType::GET,
        "POST" => requestType::POST,
        "HEAD" => requestType::HEAD,
        "PUT" => requestType::PUT,
        "CONNECT" => requestType::CONNECT,
        "DELETE" => requestType::DELETE,
        "TRACE" => requestType::TRACE,
        "OPTIONS" => requestType::OPTIONS,
        _ => requestType::INVALID,
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

    connectionData.drain(..);

    println!("HTTP Version found is: {:?}", HTTPVersionGiven);
    println!("Request path found is: {:?}", requestPathGiven);
    println!("Request type found is: {:?}", requestTypeGiven);
    for (header, content) in &headerHashMap {
        println!("Header: {header} : {content}");
    }
    Ok(parseReturnData {
        httpVersion: HTTPVersionGiven,
        requestType: requestTypeGiven,
        requestPath: requestPathGiven,
        headers: headerHashMap,
    })
}
