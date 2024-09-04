use std::{collections::HashMap, str};

use crate::*;
use errors_stupid::StdStupidError;
use standard_stupid::findSubStringWithBytes;

/// Takes an argument of `&[u8]` with the data contained being that from a buffered reader on a
/// TCPListerner and returns the data contained within including the httpVersion used, the type of request that was recieved, the path that was requested,
/// and lastly a hash map of all the headers in a <String, String> format where the key is the
/// header name and the content is the headers content inside of the Struct of [`httpStruct::ParseReturnData`]
pub fn parse_http_connection(
    mut connection_data_raw: &[u8],
) -> Result<ParseReturnData, StdStupidError> {

    for i in connection_data_raw.lines(){
        println!("new line");
        println!("{:?}", i.unwrap());
    }

    let mut connection_data = connection_data_raw.to_vec();
    // Find the / to find the method being used
    let method_location = findSubStringWithBytes(connection_data.as_slice(), b"/")?;

    let request_type_raw = str::from_utf8(&connection_data[0..method_location as usize - 1])?;

    let http_request_type_given: HttpRequestType = parse_http_request_type(request_type_raw);

    connection_data.drain(0..method_location as usize);

    // Find the H to know the entire path
    let request_path_location = findSubStringWithBytes(connection_data.as_slice(), &[0x48])?;

    let request_path_given: String =
        str::from_utf8(&connection_data[0..request_path_location as usize - 1])?.to_string();

    connection_data.drain(0..request_path_location as usize);

    // Find the first CLRF
    let http_location = findSubStringWithBytes(connection_data.as_slice(), &[0x0a])?;

    let http_version_given: f32 =
        str::from_utf8(&connection_data[5..http_location as usize - 1])?.parse::<f32>()?;

    let mut header_hash_map: HashMap<String, String> = HashMap::new();

    connection_data.drain(0..http_location as usize + 1);

    while connection_data.len() > 2 {
        let header_clrf_location =
            findSubStringWithBytes(connection_data.as_slice(), &[0x0a]).unwrap_or(0);

        let header_clrf_location_get: u32;
        let header_clrf_location_drain: usize;

        if header_clrf_location == 0 {
            header_clrf_location_get = connection_data.len() as u32;
            header_clrf_location_drain = connection_data.len();
        } else {
            header_clrf_location_get = header_clrf_location - 1;
            header_clrf_location_drain = (header_clrf_location + 1) as usize;
        }
        let header_supplied = connection_data[0..header_clrf_location_get as usize].to_vec();

        let header_double_period_location =
            findSubStringWithBytes(header_supplied.as_slice(), b":").unwrap();

        let header_name =
            str::from_utf8(&header_supplied[0..header_double_period_location as usize])?
                .to_string();

        let header_content = str::from_utf8(
            &header_supplied
                [header_double_period_location as usize + 2..header_clrf_location as usize - 1],
        )?
        .to_string();
        header_hash_map.insert(header_name, header_content);

        connection_data.drain(0..header_clrf_location_drain);
    }

    drop(connection_data);
    #[cfg(debug_assertions)]
    println!("HTTP Version found is: {:?}", http_version_given);
    #[cfg(debug_assertions)]
    println!("Request path found is: {:?}", request_path_given);
    #[cfg(debug_assertions)]
    println!("Request type found is: {:?}", http_request_type_given);
    #[cfg(debug_assertions)]
    for (header, content) in &header_hash_map {
        println!("Header: {header} : {content}");
    }
    Ok(ParseReturnData {
        httpVersion: http_version_given,
        HttpRequestType: http_request_type_given,
        requestPath: request_path_given,
        headers: header_hash_map,
    })
}

fn parse_http_request_type<T: AsRef<str>>(to_parse: T) -> HttpRequestType {
    match to_parse.as_ref() {
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
