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
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await?;
        let mut request_str = String::from_utf8_lossy(&buf[..n]);
        let req = Request::parse(&mut request_str);
        let mut res = Response::new(socket);

        match req.method {
            Method::Get | Method::Post | Method::Put | Method::Patch | Method::Delete => {
                let mut routes = self.routes.lock().await;
                match routes.get(&req.method, &req.path) {
                    Some(route) => (route.callback)(req, res).await,
                    None => {
                        res.status.status_code = 404;
                        res.send(format!("Page {} Not Found!", req.path).as_str())
                            .await?;
                        Ok(())
                    }
                }
            }
            _ => {
                res.status.status_code = 405;
                res.send(format!("Method {:?} Not Allowed!", req.method).as_str())
                    .await?;
                Ok(())
            }
        }
    }
}
