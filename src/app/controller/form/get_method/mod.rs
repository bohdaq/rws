use std::collections::HashMap;
use crate::controller::Controller;
use crate::mime_type::MimeType;
use crate::range::{ContentRange, Range};
use crate::request::{METHOD, Request};
use crate::response::{Response, STATUS_CODE_REASON_PHRASE};
use crate::server::ConnectionInfo;
use crate::symbol::SYMBOL;

pub struct FormGetMethodController;

impl Controller for FormGetMethodController {
    fn is_matching(request: &Request, _connection: &ConnectionInfo) -> bool {
        let boxed_path = request.get_uri_path();
        if boxed_path.is_err() {
            let message = format!("unable to get path {}", boxed_path.err().unwrap());
            eprintln!("{}", message);
            return false
        }

        let path = boxed_path.unwrap();
        path == "/form-get-method" && request.method == METHOD.get
    }

    fn process(_request: &Request, mut response: Response, _connection: &ConnectionInfo) -> Response {
        response.status_code = *STATUS_CODE_REASON_PHRASE.n200_ok.status_code;
        response.reason_phrase = STATUS_CODE_REASON_PHRASE.n200_ok.reason_phrase.to_string();

        // here is the form data, as an example here it is printed in the response body
        let boxed_query_option = _request.get_uri_query();
        if boxed_query_option.is_err() {
            let error_message = boxed_query_option.clone().err().unwrap().to_string();
            eprintln!("unable to extract query from url: {}", error_message)
        }
        let query_option = boxed_query_option.unwrap();
        if query_option.is_some() {
            let form: HashMap<String, String> = query_option.unwrap();


            let mut formatted_list : Vec<String> = vec![];
            for (key, value) in form.into_iter() {
                let formatted_output = format!("{} is {}{}", key, value, SYMBOL.new_line_carriage_return);
                formatted_list.push(formatted_output);
            }

            let response_body = formatted_list.join(SYMBOL.empty_string);
            response.content_range_list = vec![
                ContentRange{
                    unit: Range::BYTES.to_string(),
                    range: Range { start: 0, end: response_body.len() as u64 },
                    size: response_body.len().to_string(),
                    body: Vec::from(response_body.as_bytes()),
                    content_type: MimeType::TEXT_PLAIN.to_string(),
                }
            ];
        }

        response
    }
}

impl FormGetMethodController {

    pub fn is_matching_request(request: &Request) -> bool {
        let boxed_path = request.get_uri_path();
        if boxed_path.is_err() {
            let message = format!("unable to get path {}", boxed_path.err().unwrap());
            eprintln!("{}", message);
            return false
        }

        let path = boxed_path.unwrap();
        path == "/form-get-method" && request.method == METHOD.get

    }

    pub fn process_request(_request: &Request, mut response: Response) -> Response {
        response.status_code = *STATUS_CODE_REASON_PHRASE.n200_ok.status_code;
        response.reason_phrase = STATUS_CODE_REASON_PHRASE.n200_ok.reason_phrase.to_string();

        // here is the form data, as an example here it is printed in the response body
        let boxed_query_option = _request.get_uri_query();
        if boxed_query_option.is_err() {
            let error_message = boxed_query_option.clone().err().unwrap().to_string();
            eprintln!("unable to extract query from url: {}", error_message)
        }
        let query_option = boxed_query_option.unwrap();
        if query_option.is_some() {
            let form: HashMap<String, String> = query_option.unwrap();


            let mut formatted_list : Vec<String> = vec![];
            for (key, value) in form.into_iter() {
                let formatted_output = format!("{} is {}{}", key, value, SYMBOL.new_line_carriage_return);
                formatted_list.push(formatted_output);
            }

            let response_body = formatted_list.join(SYMBOL.empty_string);
            response.content_range_list = vec![
                ContentRange{
                    unit: Range::BYTES.to_string(),
                    range: Range { start: 0, end: response_body.len() as u64 },
                    size: response_body.len().to_string(),
                    body: Vec::from(response_body.as_bytes()),
                    content_type: MimeType::TEXT_PLAIN.to_string(),
                }
            ];
        }

        response
    }
}