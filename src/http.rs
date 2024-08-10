use std::{
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
    host: String,
    userAgent: String,
    dataType: String,
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
        let requestTypeGiven: requestType;

        // Find what method is being used
        let Methodlocation =
            findSubStringWithString(connectionData.clone(), "/".to_string()).unwrap();

        let snip = str::from_utf8(&connectionData[0..Methodlocation as usize - 1]).unwrap();

        match snip {
            "GET" => requestTypeGiven = requestType::GET,
            "POST" => requestTypeGiven = requestType::POST,
            "HEAD" => requestTypeGiven = requestType::HEAD,
            "PUT" => requestTypeGiven = requestType::PUT,
            "CONNECT" => requestTypeGiven = requestType::CONNECT,
            "DELETE" => requestTypeGiven = requestType::DELETE,
            "TRACE" => requestTypeGiven = requestType::TRACE,
            "OPTIONS" => requestTypeGiven = requestType::OPTIONS,
            _ => requestTypeGiven = requestType::INVALID,
        }

        connectionData.drain(0..Methodlocation as usize);

        // Find the H to know the entire path
        let PathLocation = findSubStringWithBytes(connectionData.clone(), &[0x48]).unwrap();

        let requestPathGiven = str::from_utf8(&connectionData[0..PathLocation as usize - 1])
            .unwrap()
            .to_string();

        connectionData.drain(0..PathLocation as usize);

        // Find the first CLRF
        let HTTPlocation = findSubStringWithBytes(connectionData.clone(), &[0x0a]).unwrap();

        let HTTPVersionGiven: f32 = str::from_utf8(&connectionData[5..HTTPlocation as usize - 1])
            .unwrap()
            .parse()
            .unwrap();

        connectionData.drain(0..PathLocation as usize + 1);

        println!("Left over data:");
        for i in 0..connectionData.len() {
            print!("{}", connectionData[i] as char);
        }
        println!("HTTP Version found is: {:?}", HTTPVersionGiven);
        println!("Request path found is: {:?}", requestPathGiven);
        println!("Request type found is: {:?}", requestTypeGiven);
        Ok(parseReturnData {
            httpVersion: HTTPVersionGiven,
            requestType: requestTypeGiven,
            requestPath: requestPathGiven,
            host: "127.0.0.1".to_owned().to_string(),
            userAgent: "Test".to_owned().to_string(),
            dataType: "Test".to_owned().to_string(),
        })
    }
}
