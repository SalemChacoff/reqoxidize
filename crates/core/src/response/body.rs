use bytes::Bytes;

#[derive(Debug, Clone)]
pub struct ResponseBody {
    bytes: Bytes,
    text: Option<String>,
}

impl ResponseBody {
    pub fn new(bytes: Bytes) -> Self {
        let text = String::from_utf8(bytes.to_vec()).ok();
        Self { bytes, text }
    }

    pub fn as_text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    pub fn as_bytes(&self) -> &Bytes {
        &self.bytes
    }

    pub fn json<T: serde::de::DeserializeOwned>(&self) -> crate::Result<T> {
        let value = serde_json::from_slice(&self.bytes)?;
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        age: u32,
    }

    #[test]
    fn test_new_with_valid_utf8() {
        let data = "Hello, World!";
        let bytes = Bytes::from(data);
        let body = ResponseBody::new(bytes.clone());

        assert_eq!(body.as_bytes(), &bytes);
        assert_eq!(body.as_text(), Some(data));
    }

    #[test]
    fn test_new_with_invalid_utf8() {
        let invalid_bytes = vec![0xFF, 0xFE, 0xFD];
        let bytes = Bytes::from(invalid_bytes);
        let body = ResponseBody::new(bytes.clone());

        assert_eq!(body.as_bytes(), &bytes);
        assert_eq!(body.as_text(), None);
    }

    #[test]
    fn test_as_text_returns_string() {
        let data = "Test data";
        let bytes = Bytes::from(data);
        let body = ResponseBody::new(bytes);

        assert_eq!(body.as_text(), Some(data));
    }

    #[test]
    fn test_as_bytes_returns_original_bytes() {
        let data = vec![1, 2, 3, 4, 5];
        let bytes = Bytes::from(data.clone());
        let body = ResponseBody::new(bytes.clone());

        assert_eq!(body.as_bytes(), &bytes);
    }

    #[test]
    fn test_json_deserialize_success() {
        let test_data = TestData {
            name: "John".to_string(),
            age: 30,
        };
        let json_str = serde_json::to_string(&test_data).unwrap();
        let bytes = Bytes::from(json_str);
        let body = ResponseBody::new(bytes);

        let result: TestData = body.json().unwrap();
        assert_eq!(result, test_data);
    }

    #[test]
    fn test_json_deserialize_failure() {
        let invalid_json = "{ invalid json }";
        let bytes = Bytes::from(invalid_json);
        let body = ResponseBody::new(bytes);

        let result: Result<TestData, _> = body.json();
        assert!(result.is_err());
    }

    #[test]
    fn test_clone() {
        let data = "Clone test";
        let bytes = Bytes::from(data);
        let body = ResponseBody::new(bytes);
        let cloned = body.clone();

        assert_eq!(body.as_text(), cloned.as_text());
        assert_eq!(body.as_bytes(), cloned.as_bytes());
    }

    #[test]
    fn test_empty_body() {
        let bytes = Bytes::new();
        let body = ResponseBody::new(bytes);

        assert_eq!(body.as_text(), Some(""));
        assert!(body.as_bytes().is_empty());
    }
}
