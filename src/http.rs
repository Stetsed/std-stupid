use std::{
    borrow::Borrow,
    collections::HashMap,
    error::Error,
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    str, usize,
};

use crate::{
    errors_stupid::HttpServerError, findSubStringWithString,
    standard_stupid::findSubStringWithBytes,
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
            Ok(_) => self.TcpListener = Some(listenerReturn.unwrap()),
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

    pub fn parseConnection(
        &mut self,
        mut connectionData: Vec<u8>,
    ) -> Result<parseReturnData, HttpServerError> {
        // Find what method is being used
        let MethodlocationCall = findSubStringWithString(connectionData.clone(), "/".to_string());

        let MethodLocation = match MethodlocationCall {
            Ok(m) => MethodlocationCall.unwrap(),
            Err(_) => {
                return Err(HttpServerError {
                    source: "Failed to find the / as part of the method location".to_string(),
                });
            }
        };

        let snip =
            str::from_utf8(&connectionData[0..MethodLocation as usize - 1]).unwrap_or(panic!(
            "This shouldn't be possible, it failed to turn the connection data snip into a str..."
        ));

        let requestTypeGiven = match snip {
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
        let RequestPathLocationCall = findSubStringWithBytes(connectionData.clone(), &[0x48]);

        let RequestPathLocation = match MethodlocationCall {
            Ok(m) => MethodlocationCall.unwrap(),
            Err(_) => {
                return Err(HttpServerError {
                    source: "Failed to find the H as part of the Request Path location".to_string(),
                });
            }
        };

        let requestPathGiven = str::from_utf8(&connectionData[0..RequestPathLocation as usize - 1])
            .unwrap_or(panic!(
            "this shouldn't be possible, it failed to turn the connection data snip into a str..."
        ))
            .to_string();

        connectionData.drain(0..RequestPathLocation as usize);

        // Find the first CLRF
        let HTTPlocation = findSubStringWithBytes(connectionData.clone(), &[0x0a]).unwrap();

        let HTTPVersionGiven: f32 = str::from_utf8(&connectionData[5..HTTPlocation as usize - 1])
            .unwrap_or(panic!(
            "this shouldn't be possible, it failed to turn the connection data snip into a str..."
        ))
            .parse()?;

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
            let mut headerSupplied = connectionData[0..headerCLRFLocationGet as usize].to_vec();

            let headerDoublePeriodLocation =
                findSubStringWithBytes(headerSupplied.as_slice().to_vec(), &[0x3a]).unwrap();

            let headerName =
                str::from_utf8(&headerSupplied[0..headerDoublePeriodLocation as usize])
                    .unwrap()
                    .to_string();

            let headerContentCall = str::from_utf8(
                &headerSupplied
                    [headerDoublePeriodLocation as usize + 2..headerCLRFLocationGet as usize],
            );

            match headerContentCall {
                Ok(m) => headerHashMap.insert(headerName, m.to_string()),
                Err(_) => {
                    return Err(HttpServerError {
                        source: "It says fuck you bitch".to_string(),
                    });
                }
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
}
