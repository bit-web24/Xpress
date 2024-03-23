use super::router::request::Request;
use super::router::response::Response;
use super::router::Router;
use crate::middleware::Middleware;
use std::sync::Arc;
use tokio::io::Result;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct RequestHandler {
    routes: Arc<tokio::sync::Mutex<Router>>,
    middlewares: Vec<Arc<dyn Middleware>>,
}

impl RequestHandler {
    pub fn from(
        routes: Arc<tokio::sync::Mutex<Router>>,
        middlewares: Vec<Arc<dyn Middleware>>,
    ) -> Self {
        Self {
            routes,
            middlewares,
        }
    }

    pub async fn handle(&mut self, mut socket: TcpStream) -> Result<()> {
        let mut buf = [0; 1024];
        let n = socket.read(&mut buf).await?;
        let mut request_str = String::from_utf8_lossy(&buf[..n]);
        let mut req = Request::parse(&mut request_str);
        let mut res = Response::new(socket);

        if req.method.is_none() {
            res.status.status_code = 405;
            res.send(format!("Method {:?} Not Allowed!", req.method.unwrap()).as_str())
                .await?;
            return Ok(());
        }

        let routes = &mut self.routes.lock().await;

        for mw in &self.middlewares {
            mw.handle(routes, &mut req)?;
        }

        let route = routes.get(&req.method.as_ref().unwrap(), &req.path);

        if route.is_none() {
            res.status.status_code = 404;
            res.send(format!("Page {} Not Found!", req.path).as_str())
                .await?;
            return Ok(());
        }

        let callback = route.unwrap().callback.clone();

        tokio::spawn(async move {
            callback(req, res).await.expect("Error from callback!");
        });

        Ok(())
    }
}
