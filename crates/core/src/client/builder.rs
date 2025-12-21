use super::{Client, ClientConfig};
use crate::protocol::{Protocol, ProtocolType, rest::RestProtocol};
use std::sync::Arc;

pub struct ClientBuilder {
    config: ClientConfig,
}

impl ClientBuilder {
    pub fn new() -> Self {
        Self {
            config: ClientConfig::default(),
        }
    }

    pub fn protocol(mut self, protocol: ProtocolType) -> Self {
        self.config.protocol = protocol;
        self
    }

    pub fn timeout(mut self, ms: u64) -> Self {
        self.config.default_timeout_ms = Some(ms);
        self
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.config.base_url = Some(url.into());
        self
    }

    pub fn build(self) -> Client {
        let protocol: Arc<dyn Protocol> = match self.config.protocol {
            ProtocolType::Rest => Arc::new(RestProtocol::new()),
            ProtocolType::Soap => todo!("SOAP not implemented yet"),
            ProtocolType::Grpc => todo!("gRPC not implemented yet"),
        };

        Client {
            protocol,
            config: self.config,
        }
    }
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}
