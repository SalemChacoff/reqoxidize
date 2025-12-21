use bytes::Bytes;
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum Body {
    Empty,
    Text(String),
    Json(serde_json::Value),
    Binary(Bytes),
    Form(Vec<(String, String)>),
}

impl Body {
    pub fn json<T: Serialize>(value: &T) -> crate::Result<Self> {
        let json = serde_json::to_value(value)?;
        Ok(Body::Json(json))
    }

    pub fn text(text: impl Into<String>) -> Self {
        Body::Text(text.into())
    }

    pub fn binary(data: impl Into<Bytes>) -> Self {
        Body::Binary(data.into())
    }

    pub fn form(fields: Vec<(String, String)>) -> Self {
        Body::Form(fields)
    }
}
