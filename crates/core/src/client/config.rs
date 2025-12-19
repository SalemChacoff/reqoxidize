use crate::protocol::ProtocolType;

#[derive(Debug, Clone)]
pub struct ClientConfig {
    pub protocol: ProtocolType,
    pub default_timeout_ms: Option<u64>,
    pub base_url: Option<String>,
    pub user_agent: Option<String>,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            protocol: ProtocolType::Rest,
            default_timeout_ms: Some(30_000),
            base_url: None,
            user_agent: Some("reqoxidize/0.1.0".into()),
        }
    }
}
