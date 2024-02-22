#[derive(Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Undefined,
}

impl Method {
    pub fn from(method_str: &str) -> Self {
        match method_str {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            _ => Method::Undefined,
        }
    }
}
