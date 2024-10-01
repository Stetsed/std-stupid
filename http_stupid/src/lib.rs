use base64::prelude::*;
use std::{
    fmt::Debug,
    fs::write,
    io::{self, prelude::*, BufReader, BufWriter, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    time::{Duration, Instant},
};
use tracing::{debug, error, field::debug, info, trace};

use crate::{http_compose::compose_http_response, http_parser::*, http_struct::*};

use errors_stupid::HttpServerError;
use errors_stupid::StdStupidError;
use http_compose::compose_server_error;
use standard_stupid::{hash_text_sha1, thread_manager::*};

const MAX_RECIEVE_BUFFER: usize = 2048;
const DEFAULT_LISTEN_TO_PORT: u16 = 8080;
const DEFAULT_LISTEN_TO_IP: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);

/// Struct that is used to define our HTTP server, given a Function, an optional IP and an optional
/// port, and if not given will run by default on 127.0.0.1:8080. And has functions to start using
/// the HTTP server with it's defined function
///
/// ## Example Code
///
/// ```rust
/// fn main() -> Result<(), StdStupidError> {
///     // Start a HTTP server listening on 127.0.0.1 on port 9182, with the ServeFile Function,
///     and 8 threads, and keepalive enabled
///     let IpAddressToUse: String = "127.0.0.1".to_string();
///     let portTouse: u16 = 9182;
///
///     let mut HttpServer = HttpServer::new(
///         ServerFunction::ServeFile,
///         Some(IpAddressToUse),
///         Some(portTouse),
///         8,
///         true
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
    server_function: ServerFunction,
    port: u16,
    tcp_listener: Option<TcpListener>,
    keepalive: bool,
    thread_pool: ThreadPool,
}

pub mod http_compose;
pub mod http_parser;
pub mod http_struct;

impl HttpServer {
    /// Creates the HTTP server struct making sure the IP is valid and not inside of the
    /// multicast/documentation range and if not provided goes with default Port and IP, and if so returns the created struct
    pub fn new(
        server_function_type: ServerFunction,
        ip_address_given: Option<&str>,
        port_given: Option<u16>,
        thread_count: usize,
        keepalive: bool,
    ) -> Result<Self, StdStupidError> {
        let port_to_use: u16 = match port_given {
            Some(p) => p,
            _ => DEFAULT_LISTEN_TO_PORT,
        };

        let ip_address_to_use: Ipv4Addr = match ip_address_given {
            Some(i) => i.parse::<Ipv4Addr>()?,
            _ => DEFAULT_LISTEN_TO_IP,
        };

        // Checks if the address is multicast/Documentation range, if yes rejects.
        if ip_address_to_use.is_multicast() {
            error!(
                "IP Address {} is inside of the  range, invalid.",
                ip_address_to_use
            );
            return Err(HttpServerError::new("IP Address Given is designated as Multicast").into());
        }
        if ip_address_to_use.is_documentation() {
            error!(
                "IP Address {} is inside of the documentation range, invalid.",
                ip_address_to_use
            );
            return Err(HttpServerError::new(
                "IP Address Given is designated as Documentation IP range.",
            )
            .into());
        }

        let thread_pool = ThreadPool::new(thread_count);

        Ok(Self {
            listening_address: ip_address_to_use,
            server_function: server_function_type,
            tcp_listener: None,
            port: port_to_use,
            keepalive,
            thread_pool,
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
                info!(
                    "HTTP server is now listening on {:?}:{:?} in server mode {:?}",
                    self.listening_address, self.port, self.server_function
                );
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
        let server_function = self.server_function;
        let http_keep_alive = self.keepalive;
        for stream in self
            .tcp_listener
            .as_ref()
            .expect("You should have a TCPlistener... how??")
            .incoming()
        {
            match stream {
                Ok(mut o) => {
                    self.thread_pool.execute(move || {
                        process_connection(server_function, http_keep_alive, &mut o)
                    });
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => panic!("Something went very wrong... {:?}", e),
            }
        }
        Ok(())
    }
}

fn process_connection(
    server_function_type: ServerFunction,
    http_keep_alive: bool,
    stream: &mut TcpStream,
) {
    let handle = process_http_connection(server_function_type, http_keep_alive, stream);

    // If the return is some it means we got an option of Some which means we need to call the
    // websocket handler for the rest of the connection
    if handle.is_some() {
        debug!("Got a HTTP handle value of some, switching over to websocket handler");
        let _ = process_websocket_connection(http_keep_alive, stream);
    }
}

fn process_http_connection(
    server_function_type: ServerFunction,
    http_keep_alive: bool,
    stream: &mut TcpStream,
) -> Option<()> {
    let mut execute_time: Instant = Instant::now();
    let mut receive_buffer: [u8; MAX_RECIEVE_BUFFER] = [0; MAX_RECIEVE_BUFFER];
    loop {
        let now = Instant::now();

        let amount = stream.read(&mut receive_buffer).unwrap_or(0);

        trace!("Recieved a message of {} bytes", amount);

        if server_function_type == ServerFunction::DumpRequest {
            write("./request.binary", &receive_buffer).unwrap()
        }

        if amount == 0 {
            let _ = stream.write_all(&compose_server_error());
            trace!("Responded to message with error");
        } else {
            match parse_http_connection(&receive_buffer) {
                Ok(w)
                    if w.headers.get("Connection") == Some(&"Upgrade".to_string())
                        && w.headers.get("Sec-WebSocket-Version") == Some(&"13".to_string()) =>
                {
                    // Need to return "Upgrade = websocket"
                    // Need to return "Connection = Upgrade"
                    // Need to return "Sec-WebSocket-Accept" with the value of Sec-WebSocket-Key
                    // concatted with "258EAFA5-E914-47DA-95CA-C5AB0DC85B11" as string, removing
                    // trailing and starting whitespaces, then hashed with SHA1 and base64 encoded
                    let value = w.headers.get("Sec-WebSocket-Key");

                    match value {
                        Some(value) => {
                            debug!("Got a websocket connection, processing key.");
                            let mut response_struct: HttpResponseStruct = HttpResponseStruct::new();

                            response_struct.set_status(101);

                            let hash: Vec<u8> = hash_text_sha1(format!(
                                "{}{}",
                                value, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
                            ))
                            .unwrap();

                            let base64 = BASE64_STANDARD.encode(hash);

                            debug!("Hashed and turned into base64 key to return: {}", base64);

                            response_struct.add_default_headers();
                            response_struct.add_header(format!("Sec-WebSocket-Accept: {}", base64));
                            response_struct.add_header("Connection: Upgrade");
                            response_struct.add_header("Upgrade: websocket");

                            let _ = &stream.write_all(&response_struct.get_response());
                            return Some(());
                        }
                        None => {
                            let _ = &stream.write_all(compose_server_error().as_slice());
                        }
                    }
                }
                Ok(d) => {
                    let _ = &stream.write_all(
                        compose_http_response(server_function_type, http_keep_alive, d).as_slice(),
                    );
                    trace!("Responded to message with sucess");
                    execute_time = Instant::now();
                }
                Err(_) => {
                    let _ = &stream.write_all(compose_server_error().as_slice());
                    trace!("Responded to message with error");
                }
            };
        }
        if now.duration_since(execute_time) > Duration::from_secs(7) || !http_keep_alive {
            debug!("Connection expired or read no more data, closing");
            return None;
        }
    }
}

fn process_websocket_connection(
    http_keep_alive: bool,
    stream: &mut TcpStream,
) -> Result<(), StdStupidError> {
    let mut execute_time: Instant = Instant::now();
    let mut buff_reader = BufReader::new(stream.try_clone()?);
    let mut buff_writer = BufWriter::new(stream.try_clone()?);

    let mut test_frame = WebSocketFrame::default();

    test_frame.set_message("GOOD MORNING VIETNAAAM");

    buff_writer.write_all(&test_frame.create_message_frame()?);
    buff_writer.flush();
    loop {
        let now = Instant::now();

        let result = buff_reader.fill_buf()?;

        let result_length = result.len();

        if result_length > 0 {
            print!("Recieved Data Bitch");
            trace!(
                "Recieved a message of {} bytes inside of the websocket",
                result.len()
            );

            let frame = WebSocketFrame::parseFrame(result.to_vec())?;

            if frame.data.as_slice() == b"ping" {
                todo!("Respond with pong bitch")
            }

            execute_time = Instant::now();

            buff_reader.consume(result_length);
        }

        if now.duration_since(execute_time) > Duration::from_secs(7) || !http_keep_alive {
            debug!("Connection expired or read no more data, closing the websocket connection");
            return Ok(());
        }
    }
}

#[cfg(test)]
mod http_stupid_tests {
    use crate::*;

    #[test]
    fn setup_server_normally() {
        let ip_address_to_use = "127.0.0.1";
        let port_to_use: u16 = 9182;

        let _ = HttpServer::new(
            ServerFunction::Debug,
            Some(ip_address_to_use),
            Some(port_to_use),
            8,
            false,
        )
        .unwrap();
    }

    #[test]
    #[should_panic]
    fn double_server_on_same_port() {
        let ip_address_to_use = "127.0.0.1";
        let port_to_use: u16 = 9182;

        let mut server_a = HttpServer::new(
            ServerFunction::Debug,
            Some(ip_address_to_use),
            Some(port_to_use),
            8,
            false,
        )
        .unwrap();

        server_a.setup_listener().unwrap();

        let mut server_b = HttpServer::new(
            ServerFunction::Debug,
            Some(ip_address_to_use),
            Some(port_to_use),
            8,
            false,
        )
        .unwrap();

        server_b.setup_listener().unwrap();
    }
}
