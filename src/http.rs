use core::str;
use std::{
    error::Error,
    fmt::Debug,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream},
    usize,
};

use crate::{errors_stupid::HttpServerError, standard_stupid::findSubStringWithBytes};

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
        let mut endRead: usize;
        let mut counter: usize = 0;

        let toFind: Vec<u8> = vec![0x0d, 0x0a];

        let location = findSubStringWithBytes(connectionData.clone(), toFind).unwrap();

        let snip = connectionData[0..location as usize].to_vec();

        for i in 0..snip.len() {
            println!("Charachter {} is {}", i, snip[i] as char)
        }

        println!("Location is: {}", location);

        Ok(parseReturnData {
            httpVersion: 1,
            requestType: requestType::GET,
            host: "127.0.0.1".to_owned().to_string(),
            userAgent: "Test".to_owned().to_string(),
            dataType: "Test".to_owned().to_string(),
        })
    }
}
