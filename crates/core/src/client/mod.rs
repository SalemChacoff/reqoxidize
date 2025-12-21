mod builder;
mod config;

pub use builder::ClientBuilder;
pub use config::ClientConfig;

use crate::{
    Request, Response, Result,
    protocol::{Protocol, ProtocolType, rest::RestProtocol},
};
use std::sync::Arc;

pub struct Client {
    protocol: Arc<dyn Protocol>,
    config: ClientConfig,
}

impl Client {
    pub fn new() -> Self {
        Self::builder().build()
    }

    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    pub async fn execute(&self, request: Request) -> Result<Response> {
        self.protocol.execute(request).await
    }

    pub fn protocol_name(&self) -> &'static str {
        self.protocol.name()
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
