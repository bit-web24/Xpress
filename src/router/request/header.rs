use std::collections::HashMap;

#[derive(Debug)]
pub struct Header {
    pub fields: HashMap<String, String>,
}

impl Header {
    pub fn new(headers: Vec<String>) -> Self {
        let mut fields = HashMap::<String, String>::new();

        for header in headers {
            let x: Vec<&str> = header.split(": ").collect();
            fields.insert(x[0].to_string(), x[1].to_string());
        }

        Self { fields }
    }

    pub fn get(&self, key: &str) -> &String {
        self.fields.get(key).unwrap()
    }
}
