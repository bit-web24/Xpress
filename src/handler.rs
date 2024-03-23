use std::future::Future;
use std::sync::Arc;

use tokio::io::Result;
use tokio::{io::AsyncReadExt, net::TcpStream};

use super::router::request::Request;
use super::router::response::Response;
use super::router::Router;
use crate::middleware::Middleware;

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

        for mw in &self.middlewares {
            let routes = &mut self.routes.lock().await;
            mw.handle(routes, &mut req)?;
        }

        let method = req.method.as_ref();

        if method.is_none() {
            res.status.status_code = 405;
            res.send(format!("Method {:?} Not Allowed!", method.unwrap()).as_str())
                .await?;
            return Ok(());
        }

        let mut routes = self.routes.lock().await;
        let route = routes.get(&method.unwrap(), &req.path);
        let callback: Arc<
            dyn Fn(
                    Request,
                    Response,
                ) -> std::pin::Pin<
                    Box<dyn Future<Output = std::prelude::v1::Result<(), std::io::Error>> + Send>,
                > + Send
                + Sync,
        >;

        if route.is_none() {
            res.status.status_code = 404;
            res.send(format!("Page {} Not Found!", req.path).as_str())
                .await?;
            return Ok(());
        }

        callback = route.unwrap().callback.clone();

        tokio::spawn(async move {
            callback(req, res).await.expect("Error in request handler!");
        });

        Ok(())
    }
}
