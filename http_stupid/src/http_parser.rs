use crate::*;
use errors_stupid::StdStupidError;
use standard_stupid::findSubStringWithBytes;
use std::{collections::HashMap, str};

/// Takes an argument of `&[u8]` with the data contained being that from a buffered reader on a
/// TCPListerner and returns the data contained within including the httpVersion used, the type of request that was recieved, the path that was requested,
/// and lastly a hash map of all the headers in a <String, String> format where the key is the
/// header name and the content is the headers content inside of the Struct of [`httpStruct::ParseReturnData`]
pub fn parse_http_connection(
    mut connection_data_raw: &[u8],
) -> Result<ParseReturnData, StdStupidError> {
    let mut header_hash_map: HashMap<String, String> = HashMap::new();
    let mut http_version_given: Option<f32> = None;
    let mut http_request_type_given: Option<HttpRequestType> = None;
    let mut http_path_given: Option<String> = None;
    for (e, i) in connection_data_raw.lines().enumerate() {
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
            http_request_type_given = Some(parse_http_request_type(str::from_utf8(http).unwrap()));
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
    #[cfg(debug_assertions)]
    {
        println!(
            "HTTP Request path given by new functiopn was: {:?}",
            http_path_given
        );
        println!(
            "HTTP request type found by new function was: {:?}",
            http_request_type_given
        );
        println!(
            "HTTP Version given by new function was: {:?}",
            http_version_given
        );
        for (header, content) in &header_hash_map {
            println!("Header: {header} : {content}");
        }
    }

    if http_version_given.is_some()
        && http_request_type_given.is_some()
        && http_path_given.is_some()
    {
        Ok(ParseReturnData {
            httpVersion: http_version_given.unwrap(),
            HttpRequestType: http_request_type_given.unwrap(),
            requestPath: http_path_given.unwrap(),
            headers: header_hash_map,
        })
    } else {
        Err(StdStupidError::HttpServer(HttpServerError {
            source: "One of the connection points was invalid".into(),
        }))
    }
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
