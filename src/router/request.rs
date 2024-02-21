use super::Method;

pub struct Request {
    method: Method,
    path: String,
    version: String,
    headers: Vec<String>,
    body: String,
}

impl Request {
    pub fn new(req_ln: (Method, String, String), headers: Vec<String>, body: String) -> Self {
        let (method, path, version) = req_ln;
        Self {
            method,
            path,
            version,
            headers,
            body,
        }
    }
}
