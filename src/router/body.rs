use serde_json::Value;

#[derive(Clone)]
pub struct Body {
    pub json: Option<Value>,
    pub url_encoded: Option<Vec<(String, String)>>,
    pub raw: Option<String>,
}

impl Body {
    pub fn new(raw: String) -> Self {
        Self {
            json: None,
            url_encoded: None,
            raw: Some(raw),
        }
    }
}
