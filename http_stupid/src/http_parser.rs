use crate::*;
use errors_stupid::StdStupidError;
use std::{collections::HashMap, str};

use tracing::{debug, error, field::debug, info, span, warn, Level};

/// Takes an argument of `&[u8]` with the data contained being that from a buffered reader on a
/// TCPListerner and returns the data contained within including the httpVersion used, the type of request that was recieved, the path that was requested,
/// and lastly a hash map of all the headers in a <String, String> format where the key is the
/// header name and the content is the headers content inside of the Struct of [`httpStruct::ParseReturnData`]
pub fn parse_http_connection(
    connection_data_raw: &[u8],
) -> Result<ParseReturnData, StdStupidError> {
    let mut header_hash_map: HashMap<String, String> = HashMap::new();

    let mut http_version_given: Option<f32> = None;
    let mut http_request_type_given: Option<HttpRequestType> = None;
    let mut http_path_given: Option<String> = None;

    let mut iterator = connection_data_raw.lines();

    for (e, i) in iterator.by_ref().enumerate() {
        let unwrapped = i?;

        if e == 0 {
            let mut rest: &[u8];
            let http: &[u8];
            let path: &[u8];
            let version: &[u8];
            // Find the slash and get the HTTP request type
            (http, rest) = unwrapped
                .as_bytes()
                .split_at_checked(
                    unwrapped
                        .find('/')
                        .ok_or_else(|| HttpServerError::new("Failed to find /"))?,
                )
                .ok_or_else(|| {
                    HttpServerError::new("Failed to split string... should be impossible")
                })?;
            http_request_type_given = Some(parse_http_request_type(str::from_utf8(http)?));
            (path, rest) = rest
                .split_at_checked(
                    rest.windows(1)
                        .position(|c| matches!(c, b"H"))
                        .ok_or_else(|| HttpServerError::new("Failed to find H"))?,
                )
                .ok_or_else(|| {
                    HttpServerError::new("Failed to split string... should be impossible")
                })?;

            http_path_given = Some(str::from_utf8(path)?.trim().to_string());

            (_, version) = rest
                .split_at_checked(
                    rest.windows(1)
                        .position(|c| matches!(c, b"/"))
                        .ok_or_else(|| HttpServerError::new("Failed to find /"))?
                        + 1,
                )
                .ok_or_else(|| {
                    HttpServerError::new("Failed to split string... should be impossible")
                })?;

            http_version_given = Some(str::from_utf8(version)?.parse::<f32>()?);
        } else if unwrapped.is_empty() {
            break;
        } else {
            let header: Vec<&str> = unwrapped.split(":").collect();

            let http_header_name = header[0].trim().to_string();
            let http_header_content = header[1..].concat().trim().to_string();

            header_hash_map.insert(http_header_name, http_header_content);
        }
    }

    let mut body: String = String::new();
    for i in iterator {
        if http_request_type_given == Some(HttpRequestType::POST)
            && header_hash_map.contains_key("Content-Type")
        {
            let content = header_hash_map.get("Content-Type");
            if content == Some(&"application/x-www-form-urlencoded".to_string()) {
                debug!("Got a x-www-form-urlencoded post request, handling");
                body = i?;
            } else {
                debug!("Unsupported post Content-Type");
            }
        }
    }

    #[cfg(debug_assertions)]
    {
        debug!(
            request_path = http_path_given, request_type = ?http_request_type_given, version = ?http_version_given
        );
        for (header, content) in &header_hash_map {
            println!("Header: {header} : {content}");
        }
        if !body.is_empty() {
            println!("\n-----Body Contents----- ");
            for i in body.splitn(64, "&") {
                println!("{}", i);
            }
            println!("\n-----Body ended-----");
        }
    }

    Ok(ParseReturnData {
        httpVersion: http_version_given
            .ok_or_else(|| HttpServerError::new("HTTP Version of the connection was invalid"))?,
        HttpRequestType: http_request_type_given.ok_or_else(|| {
            HttpServerError::new("Request Type Version of the connection was invalid")
        })?,
        requestPath: http_path_given.ok_or_else(|| {
            HttpServerError::new("Request Path Version of the connection was invalid")
        })?,
        headers: header_hash_map,
    })
}

fn parse_http_request_type<T: AsRef<str>>(to_parse: T) -> HttpRequestType {
    match to_parse.as_ref().trim() {
        "GET" => HttpRequestType::GET,
        "POST" => HttpRequestType::POST,
        "HEAD" => HttpRequestType::HEAD,
        "PUT" => HttpRequestType::PUT,
        "CONNECT" => HttpRequestType::CONNECT,
        "DELETE" => HttpRequestType::DELETE,
        "TRACE" => HttpRequestType::TRACE,
        "OPTIONS" => HttpRequestType::OPTIONS,
        _ => HttpRequestType::INVALID,
    }
}
