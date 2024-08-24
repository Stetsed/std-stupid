use std::{
    fmt::Debug,
    io::{self, prelude::*, BufReader, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

use crate::{httpCompose::composeHttpResponse, httpParser::*, httpStruct::*};

use errors_stupid::HttpServerError;
use errors_stupid::StdStupidError;
use httpCompose::compose_server_error;

/// Struct that is used to define our HTTP server, given a Function, an optional IP and an optional
/// port, and if not given will run by default on 127.0.0.1:8080. And has functions to start using
/// the HTTP server with it's defined function
///
/// ## Example Code
///
/// ```rust
/// fn main() -> Result<(), StdStupidError> {
///     // Start a HTTP server listening on 127.0.0.1 on port 9182, with the ServeFile Function
///     let IpAddressToUse: String = "127.0.0.1".to_string();
///     let portTouse: u16 = 9182;
///
///     let mut HttpServer = HttpServer::new(
///         ServerFunction::ServeFile,
///         Some(IpAddressToUse),
///         Some(portTouse),
///     )?;
///
///     // Start the TCP listening device.
///     HttpServer.setupListener()?;
///
///     // Start the listening loop for the HTTP server function given
///     HttpServer.startListening()?;
///
///     Ok(())
/// }
/// ```
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
    /// Creates the HTTP server struct making sure the IP is valid and not inside of the
    /// multicast/documentation range and if not provided goes with default Port and IP, and if so returns the created struct
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

    /// Starts the listener for the HTTP server, if succesful returns nothing, if not panics, most
    /// likley to happen if port is already in use and panic message will be displayed. Also sets
    /// the port to be non-blocking to allow simultanious connection proccesing.
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

    /// Starts the listening loop on the listener created in [`HttpServer::setupListener()`], it takes a stream and accepts it, assuming the stream is ready and it is a valid TCP stream it will read it into the buffer to be parsed by [`httpParser::parse_http_connection()`].
    /// After this it calls [`httpCompose::composeHttpResponse()`] with the data gotten to get the
    /// response to be used for the HTTP request and writes this back to the TcpStream.
    pub fn startListening(&mut self) -> Result<(), StdStupidError> {
        for stream in self.TcpListener.as_ref().unwrap().incoming() {
            match stream {
                Ok(mut o) => {
                    let mut reader = BufReader::new(&o);

                    let recieveBuffer = reader.fill_buf().unwrap().to_vec();

                    match parse_http_connection(recieveBuffer) {
                        Ok(d) => o.write_all(
                            composeHttpResponse(self.GetServerFunction(), d).as_slice(),
                        )?,
                        Err(_) => o.write_all(compose_server_error().as_slice())?,
                    };
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
