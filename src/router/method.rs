#[derive(Clone, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

impl Method {
    pub fn from(method_str: &str) -> Option<Self> {
        let method = match method_str {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            _ => return None,
        };

        Some(method)
    }
}
