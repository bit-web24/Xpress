use std::future::Future;
use std::io::Error;
use std::sync::Arc;

use crate::xpress::router::{request::Request, response::Response};
use tokio::io::Result;
use tokio::{io::AsyncReadExt, net::TcpStream};

use super::router::Router;
pub struct RequestHandler<F, Fut>
where
    F: Fn(Request, Response) -> Fut + Send + 'static + Clone,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    routes: Arc<tokio::sync::Mutex<Router<F, Fut>>>,
}

impl<F, Fut> RequestHandler<F, Fut>
where
    F: Fn(Request, Response) -> Fut + Send + 'static + Clone,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    pub fn from(routes: Arc<tokio::sync::Mutex<Router<F, Fut>>>) -> Self {
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
            request_line.next().unwrap(),
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
            (method.to_string(), path.to_string(), version.to_string()),
            headers,
            data,
        );
        let res = Response::new(socket);

        match method {
            "GET" => {
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
