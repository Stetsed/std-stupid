use std::collections::HashMap;

use errors_stupid::{HttpServerError, StdStupidError};

#[derive(Debug)]
pub enum ConnectionReturn {
    TcpStream,
    SocketAddr,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum HttpRequestType {
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ServerFunction {
    ServeFile,
    Debug,
    DumpRequest,
    Proxy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseReturnData {
    pub http_version: f32,
    pub http_request_type: HttpRequestType,
    pub request_path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

#[derive(Debug, PartialEq)]
pub struct HttpResponseStruct {
    status: Vec<u8>,
    headers: Vec<u8>,
    body: Vec<u8>,
}

impl Default for HttpResponseStruct {
    fn default() -> Self {
        Self::new()
    }
}

impl HttpResponseStruct {
    pub fn new() -> Self {
        HttpResponseStruct {
            status: Vec::new(),
            headers: Vec::new(),
            body: Vec::new(),
        }
    }
    pub fn set_status(&mut self, status_code: u16) {
        self.status = Vec::from(
            format!(
                "HTTP/1.1 {} {:?}\r\n",
                status_code,
                HttpStatusCode::from(status_code)
            )
            .as_bytes(),
        )
    }
    pub fn add_header<T: AsRef<str>>(&mut self, header: T) {
        self.headers
            .extend_from_slice(format!("{}\r\n", header.as_ref()).as_bytes())
    }

    pub fn set_body<T: AsRef<str>>(&mut self, body: T) {
        self.body.extend_from_slice(body.as_ref().as_bytes())
    }

    pub fn add_default_headers(&mut self) {
        self.add_header("Server: std-stupid-http");
        self.add_header("Content-Type: text/html");
        self.add_header("Accept-Ranges: bytes");
        self.add_header("Cache-Control: no-cache");
    }

    pub fn get_response(&mut self) -> Vec<u8> {
        let mut response_vec: Vec<u8> = Vec::new();

        response_vec.append(&mut self.status);

        if !self.body.is_empty() {
            self.add_header(format!("Content-Length: {}\r\n", self.body.len()));
        }

        response_vec.append(&mut self.headers);

        response_vec.extend_from_slice(b"\r\n");

        response_vec.append(&mut self.body);

        response_vec
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub enum HttpStatusCode {
    /// 100 Continue (RFC 7231)
    Continue,

    /// 101 Switching Protocols (RFC 7231)
    SwitchingProtocols,

    /// 102 Processing (RFC 2518)
    Processing,

    /// 103 Early Hints (RFC 8297)
    EarlyHints,

    /// 200 OK (RFC 7231)
    Ok,

    /// 201 Created (RFC 7231)
    Created,

    /// 202 Accepted (RFC 7231)
    Accepted,

    /// 203 Non-Authoritative Information (RFC 7231)
    NonAuthoritativeInformation,

    /// 204 No Content (RFC 7231)
    NoContent,

    /// 205 Reset Content (RFC 7231)
    ResetContent,

    /// 206 Partial Content (RFC 7233)
    PartialContent,

    /// 207 Multi-Status (RFC 4918)
    MultiStatus,

    /// 208 Already Reported (RFC 5842)
    AlreadyReported,

    /// 226 IM Used (RFC 3229)
    IMUsed,

    /// 300 Multiple Choices (RFC 7231)
    MultipleChoices,

    /// 301 Moved Permanently (RFC 7231)
    MovedPermanently,

    /// 302 Found (RFC 7231)
    Found,

    /// 303 See Other (RFC 7231)
    SeeOther,

    /// 304 Not Modified (RFC 7232)
    NotModified,

    /// 305 Use Proxy (RFC 7231)
    UseProxy,

    /// 306 Switch Proxy (RFC 7231)
    SwitchProxy,

    /// 307 Temporary Redirect (RFC 7231)
    TemporaryRedirect,

    /// 308 Permanent Redirect (RFC 7538)
    PermanentRedirect,

    /// 400 Bad Request (RFC 7231)
    BadRequest,

    /// 401 Unauthorized (RFC 7235)
    Unauthorized,

    /// 402 Payment Required (RFC 7231)
    PaymentRequired,

    /// 403 Forbidden (RFC 7231)
    Forbidden,

    /// 404 Not Found (RFC 7231)
    NotFound,

    /// 405 Method Not Allowed (RFC 7231)
    MethodNotAllowed,

    /// 406 Not Acceptable (RFC 7231)
    NotAcceptable,

    /// 407 Proxy Authentication Required (RFC 7235)
    ProxyAuthenticationRequired,

    /// 408 Request Timeout (RFC 7231)
    RequestTimeout,

    /// 409 Conflict (RFC 7231)
    Conflict,

    /// 410 Gone (RFC 7231)
    Gone,

    /// 411 Length Required (RFC 7231)
    LengthRequired,

    /// 412 Precondition Failed (RFC 7232)
    PreconditionFailed,

    /// 413 Payload Too Large (RFC 7231)
    PayloadTooLarge,

    /// 414 URI Too Long (RFC 7231)
    UriTooLong,

    /// 415 Unsupported Media Type (RFC 7231)
    UnsupportedMediaType,

    /// 416 Range Not Satisfiable (RFC 7233)
    RangeNotSatisfiable,

    /// 417 Expectation Failed (RFC 7231)
    ExpectationFailed,

    /// 418 I'm a teapot (RFC 2324)
    ImATeapot,

    /// 421 Misdirected Request (RFC 7540)
    MisdirectedRequest,

    /// 422 Unprocessable Entity (RFC 4918)
    UnprocessableEntity,

    /// 423 Locked (RFC 4918)
    Locked,

    /// 424 Failed Dependency (RFC 4918)
    FailedDependency,

    /// 426 Upgrade Required (RFC 7231)
    UpgradeRequired,

    /// 428 Precondition Required (RFC 6585)
    PreconditionRequired,

    /// 429 Too Many Requests (RFC 6585)
    TooManyRequests,

    /// 431 Request Header Fields Too Large (RFC 6585)
    RequestHeaderFieldsTooLarge,

    /// 451 Unavailable For Legal Reasons (RFC 7725)
    UnavailableForLegalReasons,

    /// 500 Internal Server Error (RFC 7231)
    InternalServerError,

    /// 501 Not Implemented (RFC 7231)
    NotImplemented,

    /// 502 Bad Gateway (RFC 7231)
    BadGateway,

    /// 503 Service Unavailable (RFC 7231)
    ServiceUnavailable,

    /// 504 Gateway Timeout (RFC 7231)
    GatewayTimeout,

    /// 505 HTTP Version Not Supported (RFC 7231)
    HttpVersionNotSupported,

    /// 506 Variant Also Negotiates (RFC 2295)
    VariantAlsoNegotiates,

    /// 507 Insufficient Storage (RFC 4918)
    InsufficientStorage,

    /// 508 Loop Detected (RFC 5842)
    LoopDetected,

    /// 510 Not Extended (RFC 2774)
    NotExtended,

    /// 511 Network Authentication Required (RFC 6585)
    NetworkAuthenticationRequired,

    /// Unknown status code
    Unknown(u16),
}

impl From<u16> for HttpStatusCode {
    fn from(code: u16) -> Self {
        match code {
            100 => HttpStatusCode::Continue,
            101 => HttpStatusCode::SwitchingProtocols,
            102 => HttpStatusCode::Processing,
            103 => HttpStatusCode::EarlyHints,
            200 => HttpStatusCode::Ok,
            201 => HttpStatusCode::Created,
            202 => HttpStatusCode::Accepted,
            203 => HttpStatusCode::NonAuthoritativeInformation,
            204 => HttpStatusCode::NoContent,
            205 => HttpStatusCode::ResetContent,
            206 => HttpStatusCode::PartialContent,
            207 => HttpStatusCode::MultiStatus,
            208 => HttpStatusCode::AlreadyReported,
            226 => HttpStatusCode::IMUsed,
            300 => HttpStatusCode::MultipleChoices,
            301 => HttpStatusCode::MovedPermanently,
            302 => HttpStatusCode::Found,
            303 => HttpStatusCode::SeeOther,
            304 => HttpStatusCode::NotModified,
            305 => HttpStatusCode::UseProxy,
            306 => HttpStatusCode::SwitchProxy,
            307 => HttpStatusCode::TemporaryRedirect,
            308 => HttpStatusCode::PermanentRedirect,
            400 => HttpStatusCode::BadRequest,
            401 => HttpStatusCode::Unauthorized,
            402 => HttpStatusCode::PaymentRequired,
            403 => HttpStatusCode::Forbidden,
            404 => HttpStatusCode::NotFound,
            405 => HttpStatusCode::MethodNotAllowed,
            406 => HttpStatusCode::NotAcceptable,
            407 => HttpStatusCode::ProxyAuthenticationRequired,
            408 => HttpStatusCode::RequestTimeout,
            409 => HttpStatusCode::Conflict,
            410 => HttpStatusCode::Gone,
            411 => HttpStatusCode::LengthRequired,
            412 => HttpStatusCode::PreconditionFailed,
            413 => HttpStatusCode::PayloadTooLarge,
            414 => HttpStatusCode::UriTooLong,
            415 => HttpStatusCode::UnsupportedMediaType,
            416 => HttpStatusCode::RangeNotSatisfiable,
            417 => HttpStatusCode::ExpectationFailed,
            418 => HttpStatusCode::ImATeapot,
            421 => HttpStatusCode::MisdirectedRequest,
            422 => HttpStatusCode::UnprocessableEntity,
            423 => HttpStatusCode::Locked,
            424 => HttpStatusCode::FailedDependency,
            426 => HttpStatusCode::UpgradeRequired,
            428 => HttpStatusCode::PreconditionRequired,
            429 => HttpStatusCode::TooManyRequests,
            431 => HttpStatusCode::RequestHeaderFieldsTooLarge,
            451 => HttpStatusCode::UnavailableForLegalReasons,
            500 => HttpStatusCode::InternalServerError,
            501 => HttpStatusCode::NotImplemented,
            502 => HttpStatusCode::BadGateway,
            503 => HttpStatusCode::ServiceUnavailable,
            504 => HttpStatusCode::GatewayTimeout,
            505 => HttpStatusCode::HttpVersionNotSupported,
            506 => HttpStatusCode::VariantAlsoNegotiates,
            507 => HttpStatusCode::InsufficientStorage,
            508 => HttpStatusCode::LoopDetected,
            510 => HttpStatusCode::NotExtended,
            511 => HttpStatusCode::NetworkAuthenticationRequired,
            _ => HttpStatusCode::Unknown(code),
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub struct WebSocketFrame {
    fin: bool,
    rsv1: bool,
    rsv2: bool,
    rsv3: bool,
    pub op_code: WebSocketOpCode,
    mask: bool,
    payload_length: u64,
    mask_key: Vec<u8>,
    pub data: Vec<u8>,
}

impl Default for WebSocketFrame {
    fn default() -> Self {
        let fin = false;
        let rsv1 = false;
        let rsv2 = false;
        let rsv3 = false;

        let op_code = WebSocketOpCode::default();

        let mask = false;

        let payload_length: u64 = 0;

        let mask_key: Vec<u8> = Vec::new();

        let data: Vec<u8> = Vec::new();
        WebSocketFrame {
            fin,
            rsv1,
            rsv2,
            rsv3,
            op_code,
            mask,
            payload_length,
            mask_key,
            data,
        }
    }
}

impl WebSocketFrame {
    pub fn set_message<T: AsRef<str>>(&mut self, message: T) {
        self.data = message.as_ref().as_bytes().to_vec();

        self.payload_length = self.data.len() as u64;

        if self.data.len() as u64 != u64::MAX {
            self.fin = true;
        }
    }
    pub fn create_server_message_frame(&mut self) -> Result<Vec<u8>, StdStupidError> {
        let mut frame: Vec<u8> = Vec::new();

        let mut byte_working: u8 = 0;

        if self.fin {
            byte_working += 128;
        }
        if self.rsv1 {
            byte_working += 64;
        }
        if self.rsv2 {
            byte_working += 32;
        }
        if self.rsv3 {
            byte_working += 16
        }

        match self.op_code {
            WebSocketOpCode::Continuation => byte_working += 0,
            WebSocketOpCode::Text => byte_working += 1,
            WebSocketOpCode::Binary => byte_working += 2,
            // Not used in this current implementation
            WebSocketOpCode::NonControl => byte_working += 0,
            WebSocketOpCode::ConnectionClose => byte_working += 8,
            WebSocketOpCode::Ping => byte_working += 9,
            WebSocketOpCode::Pong => byte_working += 10,
            // Not used in this current implementation
            WebSocketOpCode::FutureControl => byte_working += 1,
            WebSocketOpCode::Invalid => todo!("Time to do some sketchy shit doododa"),
        };

        // Insert the first byte which includes the FIN, RSV and OPCODE
        frame.push(byte_working);

        byte_working = 0;

        // Setup payload length calculation.. kill me already :D

        let data_length = self.data.len();

        if data_length < 126 {
            byte_working += data_length as u8;
            frame.push(byte_working);
        } else if data_length < u16::MAX as usize {
            byte_working += 126;
            frame.push(byte_working);

            frame.extend_from_slice(&data_length.to_le_bytes());
        } else if data_length < u64::MAX as usize {
            byte_working += 127;
            frame.push(byte_working);

            frame.extend_from_slice(&data_length.to_le_bytes());
        }

        if self.mask {
            return Err(
                HttpServerError::new("Mask should not be set for a server responding").into(),
            );
        }

        frame.extend(&self.data);

        Ok(frame)
    }
    pub fn parse_frame(frame: Vec<u8>) -> Result<Self, errors_stupid::StdStupidError> {
        let mut frame_iteratored = frame.clone().into_iter();

        let byte1 = frame_iteratored.next().ok_or(StdStupidError::From())?;

        let fin: bool = (byte1 & 128) > 0;
        let rsv1: bool = (byte1 & 64) > 0;
        let rsv2: bool = (byte1 & 32) > 0;
        let rsv3: bool = (byte1 & 16) > 0;

        let op_code = match byte1 & 15 {
            0 => WebSocketOpCode::Continuation,
            1 => WebSocketOpCode::Text,
            2 => WebSocketOpCode::Binary,
            3..=7 => WebSocketOpCode::NonControl,
            8 => WebSocketOpCode::ConnectionClose,
            9 => WebSocketOpCode::Ping,
            10 => WebSocketOpCode::Pong,
            11..=15 => WebSocketOpCode::FutureControl,
            _ => WebSocketOpCode::Invalid,
        };

        // Get the second byte from the frame, which holds the mask in bit 1, and the
        // payload_length base in 2-7.
        let byte2 = frame_iteratored.next().ok_or(StdStupidError::From())?;

        let mask = (byte2 & 128) > 0;

        if !mask {
            todo!("No mask, Drop connection here")
        }
        // Depending on payload length, if the payload length is not 126, or 127 then we use the 7
        // bits as our payload length. However if it is 126 then we continue reading and take the
        // next 2 bytes, 8 bytes if the value is 127, and then we take this as a number as little
        // endian bytes.
        let mut payload_length: u64 = (byte2 & 127) as u64;

        if payload_length == 126 {
            let mut combine: [u8; 2] = [0, 0];
            for (e, i) in frame_iteratored.by_ref().take(2).enumerate() {
                combine[e] = i;
            }
            payload_length = u64::from_le_bytes(combine[..].try_into().unwrap());
        } else if payload_length == 127 {
            let mut combine: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
            for (e, i) in frame_iteratored.by_ref().take(8).enumerate() {
                combine[e] = i;
            }
            payload_length = u64::from_le_bytes(combine[..].try_into().unwrap());
        }

        // Get the mask
        let mask_key: Vec<u8> = frame_iteratored.by_ref().take(4).collect();

        if payload_length != frame_iteratored.len() as u64 {
            todo!("Break request, somehow more or less payload than len?")
        };

        let data = frame_iteratored
            .enumerate()
            .map(|(i, pre_data)| pre_data ^ mask_key[i % 4])
            .collect::<Vec<u8>>();

        Ok(Self {
            fin,
            rsv1,
            rsv2,
            rsv3,
            op_code,
            mask,
            payload_length,
            mask_key,
            data,
        })
    }
}
#[derive(Debug, Default, PartialEq, Eq)]
pub enum WebSocketOpCode {
    Continuation,
    #[default]
    Text,
    Binary,
    NonControl,
    ConnectionClose,
    Ping,
    Pong,
    FutureControl,
    Invalid,
}
