use async_trait::async_trait;
use std::time::{Duration, Instant};

use crate::{
    error::Result,
    protocol::Protocol,
    request::{Body, Request},
    response::{Response, ResponseBody},
    types::Headers,
};

pub struct RestProtocol {
    client: reqwest::Client,
}

impl RestProtocol {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }
}

impl Default for RestProtocol {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Protocol for RestProtocol {
    fn name(&self) -> &'static str {
        "REST"
    }

    async fn execute(&self, request: Request) -> Result<Response> {
        self.validate(&request)?;

        let start = Instant::now();

        let mut builder = self.client.request(request.method.into(), &request.url);

        // Headers
        for (key, value) in &request.headers {
            builder = builder.header(key, value);
        }

        // Query params
        if !request.query_params.is_empty() {
            builder = builder.query(&request.query_params);
        }

        // Timeout
        if let Some(timeout) = request.timeout_ms {
            builder = builder.timeout(Duration::from_millis(timeout));
        }

        // Body
        if let Some(body) = request.body {
            builder = match body {
                Body::Empty => builder,
                Body::Text(text) => builder.body(text),
                Body::Json(json) => builder.json(&json),
                Body::Binary(bytes) => builder.body(bytes),
                Body::Form(fields) => builder.form(&fields),
            };
        }

        let response = builder.send().await?;
        let elapsed = start.elapsed().as_millis() as u64;

        let status = response.status().as_u16();
        let headers: Headers = response
            .headers()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
            .collect();

        let bytes = response.bytes().await?;

        Ok(Response {
            status,
            headers,
            body: ResponseBody::new(bytes),
            elapsed_ms: elapsed,
        })
    }

    fn validate(&self, request: &Request) -> Result<()> {
        url::Url::parse(&request.url)?;
        Ok(())
    }
}
