use std::{fs::File, str};

use crate::*;

const DISALLOWED_PATTERNS: [&str; 2] = ["..", "./"];

/// Takes in the HTTP's server function and the parsed data from
/// [`httpParser::parse_http_connection()`], and depending on the server type either spits out the
/// headers made in the request when server_function is Debug, or gets the file requested if
/// server_function is ServeFile, if function is ServeFile also makes sure it is not attempting to
/// do a file path escape.
pub fn compose_http_response(
    http_server_function: server_function,
    parse_return_data: ParseReturnData,
) -> Vec<u8> {
    if server_function::Debug == http_server_function {
        let mut http_response_struct = HttpResponseStruct::new();

        http_response_struct.setStatus(200);

        let mut response_body: String = "<html>".to_string();

        for i in parse_return_data.headers {
            let header = format!(
                "Header Name: {} <br/>Header Content: {} <br/><br/>",
                i.0, i.1
            );

            response_body.push_str(&header);
        }

        response_body.push_str("<html/>");

        http_response_struct.setBody(response_body);
        http_response_struct.addDefaultHeaders();

        http_response_struct.getResponse()
    } else if server_function::ServeFile == http_server_function {
        if HttpRequestType::GET != parse_return_data.HttpRequestType {
            let mut response: HttpResponseStruct = HttpResponseStruct::new();

            response.addDefaultHeaders();

            response.setStatus(405);

            response.getResponse()
        } else {
            let document_root = "./";

            let mut path = document_root.to_string();

            let path_given = &parse_return_data.requestPath[1..];

            let mut contains_prohibited = false;

            for i in DISALLOWED_PATTERNS {
                if path_given.contains(i) {
                    contains_prohibited = true;
                }
            }

            path.push_str(path_given);

            #[cfg(debug_assertions)]
            println!("Path: {}", path);

            if contains_prohibited {
                let mut response: HttpResponseStruct = HttpResponseStruct::new();

                response.addDefaultHeaders();

                response.setStatus(403);

                response.getResponse()
            } else {
                let mut response: HttpResponseStruct = HttpResponseStruct::new();

                let file = File::open(path).unwrap();

                let mut buf_reader: BufReader<File> = BufReader::new(file);

                let mut buffer: String = String::new();

                let read_status = buf_reader.read_to_string(&mut buffer);

                match read_status {
                    Ok(_) => {
                        response.setBody(buffer);
                        response.setStatus(200);
                    }
                    Err(_) => response.setStatus(404),
                };

                response.addDefaultHeaders();

                response.getResponse()
            }
        }
    } else {
        todo!()
    }
}

/// Composer designed to just return the most barebones that is needed to return a server error.
pub fn compose_server_error() -> Vec<u8> {
    let mut http_response_struct = HttpResponseStruct::new();

    http_response_struct.setStatus(500);

    http_response_struct.addDefaultHeaders();

    http_response_struct.getResponse()
}

/// Takes in the header as a Generic of a ref type of string, and converts it to bytes and appends
/// it to the vector given and returns the vector which has the header applied to the Vector of
/// bytes.
pub fn add_header<T: AsRef<str>>(header: T, mut vector: Vec<u8>) -> Vec<u8> {
    vector.extend_from_slice(header.as_ref().as_bytes());

    vector
}
