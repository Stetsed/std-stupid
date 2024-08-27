use std::{
    fmt::Debug,
    io::{self, prelude::*, BufReader, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
};

use crate::{http_compose::compose_http_response, http_parser::*, http_struct::*};

use errors_stupid::HttpServerError;
use errors_stupid::StdStupidError;
use http_compose::compose_server_error;

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
    listening_address: Ipv4Addr,
    server_function: server_function,
    port: u16,
    tcp_listener: Option<TcpListener>,
}

pub mod http_compose;
pub mod http_parser;
pub mod http_struct;

impl HttpServer {
    /// Creates the HTTP server struct making sure the IP is valid and not inside of the
    /// multicast/documentation range and if not provided goes with default Port and IP, and if so returns the created struct
    pub fn new(
        server_function_type: server_function,
        ip_address_given: Option<String>,
        port_given: Option<u16>,
    ) -> Result<Self, StdStupidError> {
        // Attempt to get port given, if not given then use default port 8080.
        let port_to_use: u16 = port_given.unwrap_or(8080);

        let ip_address_to_use: Ipv4Addr = ip_address_given
            .unwrap_or("127.0.0.1".parse().unwrap())
            .parse()
            .unwrap();

        // Checks if the address is multicast/Documentation range, if yes rejects.
        if ip_address_to_use.is_multicast() {
            return Err(HttpServerError::new("IP Address Given is designated as Multicast").into());
        }
        if ip_address_to_use.is_documentation() {
            return Err(HttpServerError::new(
                "IP Address Given is designated as Documentation IP range.",
            )
            .into());
        }

        Ok(Self {
            listening_address: ip_address_to_use,
            server_function: server_function_type,
            tcp_listener: None,
            port: port_to_use,
        })
    }

    /// Starts the listener for the HTTP server, if succesful returns nothing, if not panics, most
    /// likley to happen if port is already in use and panic message will be displayed. Also sets
    /// the port to be non-blocking to allow simultanious connection proccesing.
    pub fn setup_listener(&mut self) -> Result<(), StdStupidError> {
        let socket_address: SocketAddrV4 = SocketAddrV4::new(self.listening_address, self.port);
        let listener_return = TcpListener::bind(socket_address);

        match listener_return {
            Ok(o) => {
                o.set_nonblocking(true)?;
                self.tcp_listener = Some(o);
                Ok(())
            }
            Err(e) => panic!("{e:?}"),
        }
    }

    /// Starts the listening loop on the listener created in [`HttpServer::setupListener()`], it takes a stream and accepts it, assuming the stream is ready and it is a valid TCP stream it will read it into the buffer to be parsed by [`httpParser::parse_http_connection()`].
    /// After this it calls [`httpCompose::composeHttpResponse()`] with the data gotten to get the
    /// response to be used for the HTTP request and writes this back to the TcpStream.
    pub fn start_listening(&mut self) -> Result<(), StdStupidError> {
        for stream in self.tcp_listener.as_ref().unwrap().incoming() {
            match stream {
                Ok(mut o) => {
                    let mut reader = BufReader::new(&o);

                    let recieve_buffer = reader.fill_buf().unwrap().to_vec();

                    match parse_http_connection(recieve_buffer) {
                        Ok(d) => o.write_all(
                            compose_http_response(self.get_server_function(), d).as_slice(),
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

    pub fn get_server_function(&self) -> server_function {
        self.server_function
    }

    pub fn get_server_port(&self) -> u16 {
        self.port
    }

    pub fn get_server_ip(&self) -> Ipv4Addr {
        self.listening_address
    }
}
