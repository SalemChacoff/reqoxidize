mod body;

pub use body::ResponseBody;

use crate::types::Headers;
use bytes::Bytes;

#[derive(Debug)]
pub struct Response {
    pub status: u16,
    pub headers: Headers,
    pub body: ResponseBody,
    pub elapsed_ms: u64,
}

impl Response {
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    pub fn text(&self) -> Option<&str> {
        self.body.as_text()
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> crate::Result<T> {
        self.body.json()
    }

    pub fn bytes(&self) -> &Bytes {
        self.body.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    fn create_response(status: u16, body_data: &str) -> Response {
        Response {
            status,
            headers: HashMap::new(),
            body: ResponseBody::new(Bytes::from(body_data.to_string())),
            elapsed_ms: 100,
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestPayload {
        id: u32,
        message: String,
    }

    #[test]
    fn test_is_success_with_200() {
        let response = create_response(200, "OK");
        assert!(response.is_success());
    }

    #[test]
    fn test_is_success_with_201() {
        let response = create_response(201, "Created");
        assert!(response.is_success());
    }

    #[test]
    fn test_is_success_with_299() {
        let response = create_response(299, "");
        assert!(response.is_success());
    }

    #[test]
    fn test_is_success_returns_false_for_300() {
        let response = create_response(300, "");
        assert!(!response.is_success());
    }

    #[test]
    fn test_is_success_returns_false_for_404() {
        let response = create_response(404, "Not Found");
        assert!(!response.is_success());
    }

    #[test]
    fn test_is_success_returns_false_for_500() {
        let response = create_response(500, "Internal Server Error");
        assert!(!response.is_success());
    }

    #[test]
    fn test_is_success_returns_false_for_199() {
        let response = create_response(199, "");
        assert!(!response.is_success());
    }

    #[test]
    fn test_text_returns_body_content() {
        let response = create_response(200, "Hello, World!");
        assert_eq!(response.text(), Some("Hello, World!"));
    }

    #[test]
    fn test_text_returns_empty_string_for_empty_body() {
        let response = create_response(204, "");
        assert_eq!(response.text(), Some(""));
    }

    #[test]
    fn test_bytes_returns_body_bytes() {
        let data = "Test bytes";
        let response = create_response(200, data);
        assert_eq!(response.bytes(), &Bytes::from(data));
    }

    #[test]
    fn test_json_deserialize_success() {
        let payload = TestPayload {
            id: 1,
            message: "Success".to_string(),
        };
        let json_str = serde_json::to_string(&payload).unwrap();
        let response = create_response(200, &json_str);

        let result: TestPayload = response.json().unwrap();
        assert_eq!(result, payload);
    }

    #[test]
    fn test_json_deserialize_failure() {
        let response = create_response(200, "invalid json");
        let result: Result<TestPayload, _> = response.json();
        assert!(result.is_err());
    }

    #[test]
    fn test_response_with_headers() {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        let response = Response {
            status: 200,
            headers,
            body: ResponseBody::new(Bytes::from("{}")),
            elapsed_ms: 50,
        };

        assert_eq!(response.headers.get("Content-Type"), Some(&"application/json".to_string()));
    }

    #[test]
    fn test_elapsed_ms_is_stored() {
        let response = Response {
            status: 200,
            headers: HashMap::new(),
            body: ResponseBody::new(Bytes::new()),
            elapsed_ms: 250,
        };

        assert_eq!(response.elapsed_ms, 250);
    }
}
