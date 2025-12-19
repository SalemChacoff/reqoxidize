pub mod rest;

use crate::{Request, Response, Result};
use async_trait::async_trait;

#[async_trait]
pub trait Protocol: Send + Sync {
    fn name(&self) -> &'static str;

    async fn execute(&self, request: Request) -> Result<Response>;

    fn validate(&self, request: &Request) -> Result<()>;
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ProtocolType {
    #[default]
    Rest,
    Soap,
    Grpc,
}
