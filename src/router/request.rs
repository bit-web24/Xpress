use super::header::Header;
use super::Method;
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: String,
    pub headers: Header,
    pub body: Option<String>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            method: Method::Undefined,
            path: "".to_string(),
            version: "".to_string(),
            headers: Header::new(),
            body: None,
        }
    }

    pub fn parse(request_str: &mut std::borrow::Cow<'_, str>) -> Self {
        let mut req = Self::new();

        let mut lines = request_str.lines();

        // Extract request line
        let req_ln = lines.next().unwrap();
        let mut request_line = req_ln.split_whitespace();

        req.method = Method::from(request_line.next().unwrap());
        req.path = request_line.next().unwrap().to_string();
        req.version = request_line.next().unwrap().to_string();

        // Extract headers
        let mut headers = Vec::new();
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                break;
            }
            headers.push(line.to_string());
        }

        req.headers = Header::from(headers);

        // Extract data (if any)
        let data = lines.collect::<Vec<&str>>().join("\n");
        req.body = if data.is_empty() { None } else { Some(data) };

        req
    }
}
