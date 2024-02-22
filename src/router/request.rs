use super::header::Header;
use super::Method;
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: Header,
    pub body: String,
}

impl Request {
    pub fn new(req_ln: (Method, String, String), headers: Vec<String>, body: String) -> Self {
        let (method, path, version) = req_ln;
        Self {
            method,
            path,
            version,
            headers: Header::from(headers),
            body,
        }
    }
}
