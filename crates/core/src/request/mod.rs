mod body;
mod builder;

pub use body::Body;
pub use builder::RequestBuilder;

use crate::types::{Headers, Method, QueryParams};

#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub url: String,
    pub headers: Headers,
    pub query_params: QueryParams,
    pub body: Option<Body>,
    pub timeout_ms: Option<u64>,
}

impl Request {
    pub fn builder() -> RequestBuilder {
        RequestBuilder::new()
    }

    pub fn get(url: impl Into<String>) -> RequestBuilder {
        RequestBuilder::new().method(Method::Get).url(url)
    }

    pub fn post(url: impl Into<String>) -> RequestBuilder {
        RequestBuilder::new().method(Method::Post).url(url)
    }

    pub fn put(url: impl Into<String>) -> RequestBuilder {
        RequestBuilder::new().method(Method::Put).url(url)
    }

    pub fn delete(url: impl Into<String>) -> RequestBuilder {
        RequestBuilder::new().method(Method::Delete).url(url)
    }
}
