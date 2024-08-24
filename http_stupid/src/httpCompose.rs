use std::{
    fs::{self, File},
    str,
};

use crate::*;

const DISALLOWED_PATTERNS: [&str; 2] = ["..", "./"];

/// Takes in the HTTP's server function and the parsed data from
/// [`httpParser::parse_http_connection()`], and depending on the server type either spits out the
/// headers made in the request when ServerFunction is Debug, or gets the file requested if
/// ServerFunction is ServeFile, if function is ServeFile also makes sure it is not attempting to
/// do a file path escape.
pub fn composeHttpResponse(
    HttpServerFunction: ServerFunction,
    parseReturnData: ParseReturnData,
) -> Vec<u8> {
    if ServerFunction::Debug == HttpServerFunction {
        let mut HttpResponseStruct = HttpResponseStruct::new();

        HttpResponseStruct.setStatus(200);

        let mut responseBody: String = "<html>".to_string();

        for i in parseReturnData.headers {
            let header = format!(
                "Header Name: {} <br/>Header Content: {} <br/><br/>",
                i.0, i.1
            );

            responseBody.push_str(&header);
        }

        responseBody.push_str("<html/>");

        HttpResponseStruct.setBody(responseBody);
        HttpResponseStruct.addDefaultHeaders();

        HttpResponseStruct.getResponse()
    } else if ServerFunction::ServeFile == HttpServerFunction {
        if HttpRequestType::GET != parseReturnData.HttpRequestType {
            let mut response: HttpResponseStruct = HttpResponseStruct::new();

            response.addDefaultHeaders();

            response.setStatus(405);

            response.getResponse()
        } else {
            let document_root = "./";

            let mut path = document_root.to_string();

            let path_given = &parseReturnData.requestPath[1..];

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

                let mut file = File::open(path).unwrap();

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
    let mut HttpResponseStruct = HttpResponseStruct::new();

    HttpResponseStruct.setStatus(500);

    HttpResponseStruct.addDefaultHeaders();

    HttpResponseStruct.getResponse()
}

/// Takes in the header as a Generic of a ref type of string, and converts it to bytes and appends
/// it to the vector given and returns the vector which has the header applied to the Vector of
/// bytes.
pub fn addHeader<T: AsRef<str>>(header: T, mut vector: Vec<u8>) -> Vec<u8> {
    vector.extend_from_slice(header.as_ref().as_bytes());

    vector
}
