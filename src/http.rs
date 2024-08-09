use std::{
    error::Error,
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
};

use crate::errors_stupid::HttpServerError;

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
    OPTIONS,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    TRACE,
    CONNECT,
}

#[derive(Debug)]
pub struct parseReturnData {
    httpVersion: u8,
    requestType: requestType,
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
        connectionData: Vec<u8>,
    ) -> Result<parseReturnData, HttpServerError> {
        let mut startRead: usize = 0;
        let mut endRead: usize = 0;
        let mut counter: usize = 0;

        while true {
            if connectionData[counter] == 13 {
                endRead = counter;
                break;
            }
            counter += 1;
        }

        for i in startRead..endRead {
            print!("{}", connectionData[i] as char)
        }
        Ok(parseReturnData {
            httpVersion: 1,
            requestType: requestType::GET,
            host: "127.0.0.1".to_owned().to_string(),
            userAgent: "Test".to_owned().to_string(),
            dataType: "Test".to_owned().to_string(),
        })
    }
}
