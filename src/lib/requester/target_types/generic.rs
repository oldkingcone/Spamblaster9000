use std::path::PathBuf;
use serde_json;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct GenericData {
    target_url: String,
    target_body: Option<serde_json::Value>,
    target_method: serde_json::Value,
    target_headers: Option<serde_json::Value>,
}


pub struct Generic {
    pub target_url: serde_json::Value,
    pub target_body: Option<serde_json::Value>,
    pub target_method: GenericMethod,
    pub target_headers: Option<serde_json::Value>,
}

impl Generic {
    pub fn new() -> Self {
        Self {
            target_url: serde_json::Value::Null,
            target_body: None,
            target_method: GenericMethod::GET,
            target_headers: None,
        }
    }

    pub fn load_json_file(target_file: PathBuf) -> Result<Self, String> {
        if !std::path::Path::new(&target_file).exists() {
            return Err(format!("Target file {} does not exist", target_file.display()));
        }
        let d = std::fs::read_to_string(target_file).map_err(|e| e.to_string())?;
        let data: GenericData = serde_json::from_str(&d).map_err(|e| e.to_string())?;
        Ok(Self {
            target_url: data.target_url.into(),
            target_body: data.target_body,
            target_method: parse_method_from_json(&data.target_method)?,
            target_headers: data.target_headers,
        })
    }
}

pub enum GenericMethod {
    POST,
    GET,
    PUT,
    PATCH,
    DELETE,
    HEAD,
    OPTIONS,
    TRACE,
    CONNECT,
}

impl FromStr for GenericMethod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_uppercase().as_str() {
            "POST" => Ok(GenericMethod::POST),
            "GET" => Ok(GenericMethod::GET),
            "PUT" => Ok(GenericMethod::PUT),
            "PATCH" => Ok(GenericMethod::PATCH),
            "DELETE" => Ok(GenericMethod::DELETE),
            "HEAD" => Ok(GenericMethod::HEAD),
            "OPTIONS" => Ok(GenericMethod::OPTIONS),
            "TRACE" => Ok(GenericMethod::TRACE),
            "CONNECT" => Ok(GenericMethod::CONNECT),
            other => Err(format!("Unknown HTTP method: {}", other)),
        }
    }
}

impl From<GenericMethod> for reqwest::Method {
    fn from(method: GenericMethod) -> Self {
        method.to_reqwest_method()
    }
}

impl GenericMethod {
    fn to_reqwest_method(&self) -> reqwest::Method {
        match self {
            GenericMethod::GET => reqwest::Method::GET,
            GenericMethod::PUT => reqwest::Method::PUT,
            GenericMethod::PATCH => reqwest::Method::PATCH,
            GenericMethod::DELETE => reqwest::Method::DELETE,
            GenericMethod::HEAD => reqwest::Method::HEAD,
            GenericMethod::OPTIONS => reqwest::Method::OPTIONS,
            GenericMethod::TRACE => reqwest::Method::TRACE,
            GenericMethod::CONNECT => reqwest::Method::CONNECT,
            _ => reqwest::Method::POST,
        }
    }
}

fn parse_method_from_json(value: &serde_json::Value) -> Result<GenericMethod, String> {
    let s = value.as_str().ok_or("Method must be a string")?;
    let m = GenericMethod::from_str(s).map_err(|e| e.to_string())?;
    Ok(m)
}

pub fn load_json_file(target_file: PathBuf) -> Result<Generic, String> {
    Ok(Generic::load_json_file(target_file)?)
}