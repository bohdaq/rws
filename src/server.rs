use std::io::prelude::*;
use std::net::TcpStream;
use std::{env, fs};
use std::borrow::Borrow;

use crate::request::Request;
use crate::response::Response;
use crate::app::App;
use crate::CONSTANTS;
use crate::constant::{HTTP_VERSIONS, REQUEST_METHODS, RESPONSE_STATUS_CODE_REASON_PHRASES};


pub struct Server {}


impl Server {
    pub(crate) fn handle_connection(s: TcpStream) {
        let mut buffer = [0; 1024];

        let mut stream = s;

        stream.read(&mut buffer).unwrap();


        let response = Server::process_request(&buffer[..]);

        stream.write(response.borrow()).unwrap();
        stream.flush().unwrap();

    }

    pub(crate) fn process_request(request: &[u8]) -> Vec<u8> {
        let request: Request = Request::parse_request(&request.to_vec());
        let response = App::handle_request(request);
        let raw_response = Response::generate_response(response);

        raw_response
    }
}


