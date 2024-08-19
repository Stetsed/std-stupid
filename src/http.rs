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
    errors_stupid::HttpServerError, findSubStringWithString, httpParser::*, httpStruct::*,
    standard_stupid::findSubStringWithBytes,
};

#[derive(Debug)]
pub struct HttpServer {
    ListeningAddress: Ipv4Addr,
    ServerFunction: ServerFunction,
    pub TcpListener: Option<TcpListener>,
    Port: u16,
}

pub mod httpCompose;
pub mod httpParser;
pub mod httpStruct;

impl HttpServer {
    pub fn new(
        IpAddressGiven: Option<String>,
        PortGiven: Option<u16>,
    ) -> Result<Self, HttpServerError> {
        let ServerFunctionType: ServerFunction = ServerFunction::Debug;

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
            ServerFunction: ServerFunctionType,
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

    pub fn GetServerFunction(&self) -> ServerFunction {
        self.ServerFunction
    }
}
