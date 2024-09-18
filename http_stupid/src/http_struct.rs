use std::collections::HashMap;

#[derive(Debug)]
pub enum connectionReturn {
    TcpStream,
    SocketAddr,
}
#[allow(clippy::upper_case_acronyms)]
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

#[allow(clippy::upper_case_acronyms, dead_code)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum server_function {
    ServeFile,
    Debug,
    DumpRequest,
    Proxy,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseReturnData {
    pub httpVersion: f32,
    pub HttpRequestType: HttpRequestType,
    pub requestPath: String,
    pub headers: HashMap<String, String>,
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
    pub fn setStatus(&mut self, statusCode: u16) {
        self.status = Vec::from(
            format!(
                "HTTP/1.1 {} {:?}\r\n",
                statusCode,
                HttpStatusCode::from(statusCode)
            )
            .as_bytes(),
        )
    }
    pub fn addHeader<T: AsRef<str>>(&mut self, header: T) {
        self.headers
            .extend_from_slice(format!("{}\r\n", header.as_ref()).as_bytes())
    }

    pub fn setBody(&mut self, body: String) {
        self.body.extend_from_slice(body.as_bytes())
    }

    pub fn addDefaultHeaders(&mut self) {
        self.addHeader("Server: std-stupid-http");
        self.addHeader("Content-Type: text/html");
        self.addHeader("Accept-Ranges: bytes");
        self.addHeader("Keep-Alive: 7s");
        //self.addHeader("Connection: close");
        self.addHeader("Cache-Control: no-cache");
    }

    pub fn getResponse(&mut self) -> Vec<u8> {
        let mut response_vec: Vec<u8> = Vec::new();

        response_vec.append(&mut self.status);

        self.addHeader(format!("Content-Length: {}\r\n", self.body.len() + 2));

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
