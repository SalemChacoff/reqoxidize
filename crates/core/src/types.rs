use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Headers = HashMap<String, String>;
pub type QueryParams = HashMap<String, String>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Head,
    Options,
}

impl From<Method> for reqwest::Method {
    fn from(method: Method) -> Self {
        match method {
            Method::Get => reqwest::Method::GET,
            Method::Post => reqwest::Method::POST,
            Method::Put => reqwest::Method::PUT,
            Method::Patch => reqwest::Method::PATCH,
            Method::Delete => reqwest::Method::DELETE,
            Method::Head => reqwest::Method::HEAD,
            Method::Options => reqwest::Method::OPTIONS,
        }
    }
}
