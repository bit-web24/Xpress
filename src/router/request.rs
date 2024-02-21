pub struct Request {
    method: String,
    path: String,
    version: String,
    headers: Vec<String>,
    data: String,
}

impl Request {
    pub fn new(req_ln: (String, String, String), headers: Vec<String>, data: String) -> Self {
        let (method, path, version) = req_ln;
        Self {
            method,
            path,
            version,
            headers,
            data,
        }
    }
}
