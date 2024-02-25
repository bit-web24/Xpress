use serde_json::Value;

#[derive(Clone)]
pub struct Body {
    pub json: Option<Value>,
    pub url_encoded: Option<Value>,
    pub raw: Option<String>,
    pub buffer: Option<Vec<u8>>,
}

impl Body {
    pub fn new(raw: String) -> Self {
        Self {
            json: None,
            url_encoded: None,
            raw: Some(raw),
            buffer: None,
        }
    }
}
