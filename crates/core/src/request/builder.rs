use super::{Body, Request};
use crate::types::{Headers, Method, QueryParams};

#[derive(Default)]
pub struct RequestBuilder {
    method: Option<Method>,
    url: Option<String>,
    headers: Headers,
    query_params: QueryParams,
    body: Option<Body>,
    timeout_ms: Option<u64>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn method(mut self, method: Method) -> Self {
        self.method = Some(method);
        self
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn headers(mut self, headers: Headers) -> Self {
        self.headers.extend(headers);
        self
    }

    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.insert(key.into(), value.into());
        self
    }

    pub fn body(mut self, body: Body) -> Self {
        self.body = Some(body);
        self
    }

    pub fn json<T: serde::Serialize>(self, value: &T) -> crate::Result<Self> {
        Ok(self.body(Body::json(value)?))
    }

    pub fn timeout(mut self, ms: u64) -> Self {
        self.timeout_ms = Some(ms);
        self
    }

    pub fn build(self) -> crate::Result<Request> {
        let url = self
            .url
            .ok_or_else(|| crate::Error::InvalidRequest("URL is required".into()))?;

        Ok(Request {
            method: self.method.unwrap_or(Method::Get),
            url,
            headers: self.headers,
            query_params: self.query_params,
            body: self.body,
            timeout_ms: self.timeout_ms,
        })
    }
}
