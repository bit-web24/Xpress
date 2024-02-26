use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Header {
    pub fields: HashMap<String, String>,
}

impl Header {
    pub fn new() -> Self {
        Self {
            fields: HashMap::<String, String>::new(),
        }
    }

    pub fn from(headers: Vec<String>) -> Self {
        let mut fields = HashMap::<String, String>::new();

        for header in headers {
            let x: Vec<&str> = header.split(": ").collect();
            fields.insert(x[0].to_string(), x[1].to_string());
        }

        Self { fields }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.fields.get(key)
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.fields.insert(key.to_string(), value.to_string());
    }

    pub fn to_string(&self) -> String {
        let mut headers = String::new();
        for (key, value) in &self.fields {
            headers.push_str(&format!("{key}: {value}\r\n"));
        }

        headers
    }
}
