use std::str::FromStr;
use std::string::ToString;
use serde_json::Value;

pub const BAD_RESPONSE_CODES: [i32; 7] = [400, 404, 403, 500, 503, 501, 502];
pub const BAD_RESPONSE_MESSAGES: [&'static str; 2] = [
    "you are being rate limited",
    "you are temporarily being blocked"
];

pub const DEFAULT_DATA_DIRECTORY: &str = ".local/share/spam_blaster/";

pub enum TargetType {
    Discord = 0,
    Telegram = 1,
    Grabify = 2,
    Generic = 3,
}

pub struct MainConfig {
    worker_pool_size: usize,
    total_workers: usize,
    cpu_limit: usize,
    exit_on_fail: bool,
    exit_on_blocked: bool,
    jitter: Option<bool>,
    proxy: Option<String>,
}

#[derive(Debug, Clone)]
pub struct RequestConfig {
    pub url: String,
    pub method: RequestMethod,
    pub body: Option<Value>,
    pub jitter: Option<Vec<f64>>,
    pub known_bad_response_codes: Vec<i32>,
    pub known_bad_response_strings: Vec<&'static str>,
    pub proxy: Option<String>,
}

#[derive(Clone, Debug)]
pub enum RequestMethod {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

impl FromStr for RequestMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "GET" => Ok(RequestMethod::GET),
            "PUT" => Ok(RequestMethod::PUT),
            "PATCH" => Ok(RequestMethod::PATCH),
            "DELETE" => Ok(RequestMethod::DELETE),
            "HEAD" => Ok(RequestMethod::HEAD),
            "OPTIONS" => Ok(RequestMethod::OPTIONS),
            "TRACE" => Ok(RequestMethod::TRACE),
            "CONNECT" => Ok(RequestMethod::CONNECT),
            other => Err(format!("Unknown HTTP method: {}", other)),
        }
    }
}

impl From<RequestMethod> for reqwest::Method {
    fn from(value: RequestMethod) -> Self {
        match value {
            RequestMethod::GET => reqwest::Method::GET,
            RequestMethod::POST => reqwest::Method::POST,
            RequestMethod::PUT => reqwest::Method::PUT,
            RequestMethod::PATCH => reqwest::Method::PATCH,
            RequestMethod::DELETE => reqwest::Method::DELETE,
            RequestMethod::HEAD => reqwest::Method::HEAD,
            RequestMethod::OPTIONS => reqwest::Method::OPTIONS,
            RequestMethod::TRACE => reqwest::Method::TRACE,
            RequestMethod::CONNECT => reqwest::Method::CONNECT,
        }
    }
}

impl RequestConfig {

    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_method(&mut self, method: String) {
        self.method = RequestMethod::from_str(&method).unwrap();
    }

    pub fn set_body(&mut self, body: Value) {
        #[cfg(debug_assertions)]{
            println!("{:?}", body);
        }
        self.body = Some(body);
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_body(&self) -> Option<Value> {
        self.body.clone()
    }

    pub fn get_body_as_string(&self) -> String {
        self.body.clone().unwrap().to_string()
    }

    pub fn get_body_as_json(&self) -> Value {
        self.body.clone().unwrap()
    }

    pub fn get_body_as_json_string(&self) -> String {
        self.body.clone().unwrap().to_string()
    }
}