use std::{env};
use std::fs::{File, metadata};
use std::io::Read;
use crate::constant::{HTTP_VERSIONS, HTTPError, REQUEST_METHODS, RESPONSE_STATUS_CODE_REASON_PHRASES};
use crate::{Config, CONSTANTS};
use crate::cors::Cors;
use crate::header::Header;
use crate::mime_type::MimeType;
use crate::range::{ContentRange, Range};

use crate::request::Request;
use crate::response::Response;


pub struct App {}

impl App {
    pub(crate) const NOT_FOUND_PAGE_FILEPATH: &'static str = "404.html";
    pub(crate) const INDEX_FILEPATH: &'static str = "index.html";

    pub(crate) fn handle_request(mut request: Request) -> (Response, Request) {

        // by default we assume route or static asset is not found
        let mut file_content = Vec::new();
        let mut file = File::open(&App::NOT_FOUND_PAGE_FILEPATH).expect("Unable to open file");
        file.read_to_end(&mut file_content).expect("Unable to read");

        let contents = file_content;
        let content_type = MimeType::detect_mime_type(App::NOT_FOUND_PAGE_FILEPATH);

        let length = contents.len() as u64;
        let content_range = ContentRange {
            unit: CONSTANTS.bytes.to_string(),
            range: Range { start: 0, end: length },
            size: length.to_string(),
            body: contents,
            content_type
        };


        let mut response = Response {
            http_version: HTTP_VERSIONS.http_version_1_1.to_string(),
            status_code: RESPONSE_STATUS_CODE_REASON_PHRASES.n404_not_found.status_code.to_string(),
            reason_phrase: RESPONSE_STATUS_CODE_REASON_PHRASES.n404_not_found.reason_phrase.to_string(),
            headers: vec![],
            content_range_list: vec![content_range]
        };

        if request.request_uri == CONSTANTS.slash {
            let mut file_content = Vec::new();
            let mut file = File::open(&App::INDEX_FILEPATH).expect("Unable to open file");
            file.read_to_end(&mut file_content).expect("Unable to read");

            let contents = file_content;
            let content_type = MimeType::detect_mime_type(App::INDEX_FILEPATH);


            let length = contents.len() as u64;
            let content_range = ContentRange {
                unit: CONSTANTS.bytes.to_string(),
                range: Range { start: 0, end: length },
                size: length.to_string(),
                body: contents,
                content_type
            };

            let content_range_list = vec![content_range];

            response = Response {
                http_version: HTTP_VERSIONS.http_version_1_1.to_string(),
                status_code: RESPONSE_STATUS_CODE_REASON_PHRASES.n200_ok.status_code.to_string(),
                reason_phrase: RESPONSE_STATUS_CODE_REASON_PHRASES.n200_ok.reason_phrase.to_string(),
                headers: vec![],
                content_range_list,
            };
        }

        let is_get = request.method == REQUEST_METHODS.get;
        let is_head = request.method == REQUEST_METHODS.head;
        let is_options = request.method == REQUEST_METHODS.options;
        if is_get || is_head || is_options && request.request_uri != CONSTANTS.slash {
            let boxed_content_range_list = App::process_static_resources(&request);
            if boxed_content_range_list.is_ok() {
                let content_range_list = boxed_content_range_list.unwrap();

                if content_range_list.len() != 0 {

                    let mut status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.n200_ok.status_code;
                    let mut reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.n200_ok.reason_phrase;

                    let does_request_include_range_header = request.get_header(Header::RANGE.to_string()).is_some();
                    if does_request_include_range_header {
                        status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.n206_partial_content.status_code;
                        reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.n206_partial_content.reason_phrase;
                    }

                    let is_options_request = request.method == REQUEST_METHODS.options;
                    if is_options_request {
                        status_code = RESPONSE_STATUS_CODE_REASON_PHRASES.n204_no_content.status_code;
                        reason_phrase = RESPONSE_STATUS_CODE_REASON_PHRASES.n204_no_content.reason_phrase;
                    }

                    response = Response {
                        http_version: HTTP_VERSIONS.http_version_1_1.to_string(),
                        status_code: status_code.to_string(),
                        reason_phrase: reason_phrase.to_string(),
                        headers: vec![],
                        content_range_list,
                    };

                    let is_cors_set_to_allow_all_requests : bool = env::var(Config::RWS_CONFIG_CORS_ALLOW_ALL).unwrap().parse().unwrap();
                    if is_cors_set_to_allow_all_requests {
                        (request, response) = Cors::allow_all(request, response).unwrap();
                    } else {
                        (request, response) = Cors::process_using_default_config(request, response).unwrap();
                    }
                }
            } else {
                let error : HTTPError = boxed_content_range_list.err().unwrap();
                let body = error.message;
                let body_length = body.len() as u64;

                let content_range_list = vec![
                    ContentRange {
                        unit: CONSTANTS.bytes.to_string(),
                        range: Range { start: 0, end: body_length },
                        size: body_length.to_string(),
                        body: body.as_bytes().to_vec(),
                        content_type: MimeType::TEXT_PLAIN.to_string(),
                    }
                ];

                response = Response {
                    http_version: HTTP_VERSIONS.http_version_1_1.to_string(),
                    status_code: error.status_code_reason_phrase.status_code.to_string(),
                    reason_phrase: error.status_code_reason_phrase.reason_phrase.to_string(),
                    headers: vec![],
                    content_range_list,
                };
            }

        }

        if request.request_uri != CONSTANTS.slash && request.method == REQUEST_METHODS.post {
            let content_range_list = vec![];

            response = Response {
                http_version: HTTP_VERSIONS.http_version_1_1.to_string(),
                status_code: RESPONSE_STATUS_CODE_REASON_PHRASES.n200_ok.status_code.to_string(),
                reason_phrase: RESPONSE_STATUS_CODE_REASON_PHRASES.n200_ok.reason_phrase.to_string(),
                headers: vec![],
                content_range_list,
            };

            let is_cors_set_to_allow_all_requests : bool = env::var(Config::RWS_CONFIG_CORS_ALLOW_ALL).unwrap().parse().unwrap();
            if is_cors_set_to_allow_all_requests {
                (request, response) = Cors::allow_all(request, response).unwrap();
            } else {
                (request, response) = Cors::process_using_default_config(request, response).unwrap();
            }
        }

        (response, request)
    }

    pub(crate) fn process_static_resources(request: &Request) -> Result<Vec<ContentRange>, HTTPError> {
        let dir = env::current_dir().unwrap();
        let working_directory = dir.as_path().to_str().unwrap();
        let static_filepath = [working_directory, request.request_uri.as_str()].join(CONSTANTS.empty_string);

        let mut content_range_list = Vec::new();

        let boxed_file = File::open(&static_filepath);
        if boxed_file.is_ok()  {
            let md = metadata(&static_filepath).unwrap();
            if md.is_file() {
                let mut range_header = &Header {
                    name: Header::RANGE.to_string(),
                    value: "bytes=0-".to_string()
                };

                let boxed_header = request.get_header(Header::RANGE.to_string());
                if boxed_header.is_some() {
                    range_header = boxed_header.unwrap();
                }

                let boxed_content_range_list = Range::get_content_range_list(&request.request_uri, range_header);
                if boxed_content_range_list.is_ok() {
                    content_range_list = boxed_content_range_list.unwrap();
                } else {
                    let error = boxed_content_range_list.err().unwrap();
                    return Err(error)
                }
            }
        }

        Ok(content_range_list)
    }
}