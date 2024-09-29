use std::{fs::File, str};

use crate::*;

const DISALLOWED_PATTERNS: [&str; 2] = ["..", "./"];

/// Takes HTTP's server function and the parsed data from
/// [`httpParser::parse_http_connection()`], depending on the server type spits out the
/// headers in the request when server_function is Debug, or gets the file requested if
/// server_function is ServeFile, if function is ServeFile also makes sure it is not attempting to
/// do a file path escape.
pub fn compose_http_response(
    http_server_function: ServerFunction,
    http_keep_alive: bool,
    parse_return_data: ParseReturnData,
) -> Vec<u8> {
    if ServerFunction::Debug == http_server_function
        || http_server_function == ServerFunction::DumpRequest
    {
        let mut http_response_struct = HttpResponseStruct::new();

        http_response_struct.set_status(200);

        let mut response_body: String = "<html>".to_string();

        for i in parse_return_data.headers {
            let header = format!(
                "Header Name: {} <br/>Header Content: {} <br/><br/>",
                i.0, i.1
            );

            response_body.push_str(&header);
        }

        response_body.push_str("<html/>");

        http_response_struct.set_body(response_body);
        http_response_struct.add_default_headers();
        if http_keep_alive {
            http_response_struct.add_header("Keep-Alive: 7s");
        } else {
            http_response_struct.add_header("Connection: close");
        }

        http_response_struct.get_response()
    } else if ServerFunction::ServeFile == http_server_function {
        if HttpRequestType::GET != parse_return_data.http_request_type {
            let mut response: HttpResponseStruct = HttpResponseStruct::new();

            response.add_default_headers();

            response.set_status(405);

            response.get_response()
        } else {
            let document_root = "./";

            let mut path = document_root.to_string();

            let path_given = &parse_return_data.request_path[1..];

            let mut contains_prohibited = false;

            for i in DISALLOWED_PATTERNS {
                if path_given.contains(i) {
                    contains_prohibited = true;
                }
            }

            path.push_str(path_given);

            if contains_prohibited {
                let mut response: HttpResponseStruct = HttpResponseStruct::new();

                response.add_default_headers();

                response.set_status(403);

                response.get_response()
            } else {
                let mut response: HttpResponseStruct = HttpResponseStruct::new();

                match File::open(path) {
                    Ok(f) => {
                        let mut buffer: String = String::new();

                        let mut buf_reader: BufReader<File> = BufReader::new(f);

                        let read_status = buf_reader.read_to_string(&mut buffer);

                        match read_status {
                            Ok(_) => {
                                response.set_body(buffer);
                                response.set_status(200);
                            }
                            Err(_) => {
                                debug!("File read failed");
                                response.set_status(500)
                            }
                        };
                    }
                    Err(_) => {
                        debug!("File was not found");
                        response.set_status(404)
                    }
                };

                response.add_default_headers();

                response.get_response()
            }
        }
    } else {
        todo!()
    }
}

/// Composer designed to just return the most barebones that is needed to return a server error.
pub fn compose_server_error() -> Vec<u8> {
    let mut http_response_struct = HttpResponseStruct::new();

    http_response_struct.set_status(500);

    http_response_struct.add_default_headers();

    http_response_struct.get_response()
}

/// Takes in the header as a Generic of a ref type of string, and converts it to bytes and appends
/// it to the vector given and returns the vector which has the header applied to the Vector of
/// bytes.
pub fn add_header<T: AsRef<str>>(header: T, mut vector: Vec<u8>) -> Vec<u8> {
    vector.extend_from_slice(header.as_ref().as_bytes());

    vector
}
