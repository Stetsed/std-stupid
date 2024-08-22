use std::{
    fmt::Debug,
    io::{self, prelude::*, BufReader, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

use crate::{httpCompose::composeHttpResponse, httpParser::*, httpStruct::*};

use errors_stupid::HttpServerError;
use errors_stupid::StdStupidError;

#[derive(Debug)]
pub struct HttpServer {
    ListeningAddress: Ipv4Addr,
    ServerFunction: ServerFunction,
    Port: u16,
    TcpListener: Option<TcpListener>,
}

pub mod httpCompose;
pub mod httpParser;
pub mod httpStruct;

impl HttpServer {
    pub fn new(
        ServerFunctionType: ServerFunction,
        IpAddressGiven: Option<String>,
        PortGiven: Option<u16>,
    ) -> Result<Self, StdStupidError> {
        // Attempt to get port given, if not given then use default port 8080.
        let PortToUse: u16 = PortGiven.unwrap_or(8080);

        let IpAddressToUse: Ipv4Addr = IpAddressGiven
            .unwrap_or("127.0.0.1".parse().unwrap())
            .parse()
            .unwrap();

        // Checks if the address is multicast/Documentation range, if yes rejects.
        if IpAddressToUse.is_multicast() {
            let _ = Into::<StdStupidError>::into(HttpServerError::new(
                "IP Address Given is designated as Multicast",
            ));
        }
        if IpAddressToUse.is_documentation() {
            let _ = Into::<StdStupidError>::into(HttpServerError::new(
                "IP Address Given is designated as Documentation IP range.",
            ));
        }

        Ok(Self {
            ListeningAddress: IpAddressToUse,
            ServerFunction: ServerFunctionType,
            TcpListener: None,
            Port: PortToUse,
        })
    }

    pub fn setupListener(&mut self) -> Result<(), StdStupidError> {
        let socketAddress: SocketAddrV4 = SocketAddrV4::new(self.ListeningAddress, self.Port);
        let listenerReturn = TcpListener::bind(socketAddress);

        match listenerReturn {
            Ok(o) => {
                o.set_nonblocking(true)?;
                self.TcpListener = Some(o);
                Ok(())
            }
            Err(e) => panic!("{e:?}"),
        }
    }

    pub fn startListening(&mut self) -> Result<(), StdStupidError> {
        for stream in self.TcpListener.as_ref().unwrap().incoming() {
            match stream {
                Ok(mut o) => {
                    let mut reader = BufReader::new(&o);

                    let recieveBuffer = reader.fill_buf().unwrap().to_vec();

                    let data = parseHTTPConnection(recieveBuffer)?;

                    o.write_all(composeHttpResponse(self.GetServerFunction(), data).as_slice())?;
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => panic!("Something went very wrong... {:?}", e),
            }
        }
        Ok(())
    }

    pub fn GetServerFunction(&self) -> ServerFunction {
        self.ServerFunction
    }

    pub fn getServerPort(&self) -> u16 {
        self.Port
    }

    pub fn getServerIP(&self) -> Ipv4Addr {
        self.ListeningAddress
    }
}
