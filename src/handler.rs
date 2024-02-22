use std::io::Error;
use std::sync::Arc;

use tokio::io::Result;
use tokio::{io::AsyncReadExt, net::TcpStream};

use super::router::request::Request;
use super::router::response::Response;
use super::router::Router;
use crate::router::method::Method;

pub struct RequestHandler {
    routes: Arc<tokio::sync::Mutex<Router>>,
}

impl RequestHandler {
    pub fn from(routes: Arc<tokio::sync::Mutex<Router>>) -> Self {
        Self { routes }
    }

    pub async fn handle(&mut self, mut socket: TcpStream) -> Result<()> {
        // Handler
        let mut buf = [0; 1024];
        let n = socket
            .read(&mut buf)
            .await
            .expect("Error reading from the socket!");

        let request_str = String::from_utf8_lossy(&buf[..n]);
        let mut lines = request_str.lines();

        // Extract request line
        let req_ln = lines.next().unwrap();
        let mut request_line = req_ln.split_whitespace();

        let (method, path, version) = (
            Method::from(request_line.next().unwrap()),
            request_line.next().unwrap(),
            request_line.next().unwrap(),
        );

        // Extract headers
        let mut headers = Vec::new();
        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                break; // Empty line indicates end of headers
            }
            headers.push(line.to_string());
        }

        // Extract data (if any)
        let data = lines.collect::<Vec<&str>>().join("\n");

        let req = Request::new(
            (method.clone(), path.to_string(), version.to_string()),
            headers,
            data,
        );

        let res = Response::new(socket);

        match method {
            Method::Get => {
                let mut routes = self.routes.lock().await;
                match routes.get(path) {
                    Some(ref route) => (route.callback)(req, res).await,
                    None => Err(Error::new(
                        std::io::ErrorKind::NotFound,
                        "ERROR: 404 page not found!",
                    )),
                }
            }
            // "POST" => {}
            // "PUT" => {}
            // "DELETE" => {}
            _ => Err(Error::new(
                std::io::ErrorKind::Other,
                "ERROR: 404 page not found!",
            )),
        }
    }
}
