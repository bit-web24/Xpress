use std::sync::Arc;
use tokio::io::Result;
use tokio::net::{TcpListener, ToSocketAddrs};

mod handler;
pub mod middleware;
pub mod path;
pub mod router;

use handler::RequestHandler;
use router::method::Method;
use router::request::Request;
use router::response::Response;
use router::{Route, Router};
use tokio::sync::Mutex;

use middleware::Middleware;

pub struct Xpress {
    middlewares: Vec<Arc<dyn Middleware>>,
    routes: Arc<Mutex<Router>>,
}

impl Xpress {
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
            routes: Arc::new(Mutex::new(Router::new())),
        }
    }

    pub async fn _use_(&mut self, middleware: impl Middleware + 'static) {
        self.middlewares.push(Arc::new(middleware));
    }

    pub async fn get<F>(&mut self, path: &str, callback: F) -> Result<()>
    where
        F: Fn(
                Request,
                Response,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let route = Route::new(path, Method::Get, callback);

        let mut routes = self.routes.lock().await;

        routes.add(route);
        Ok(())
    }

    pub async fn post<F>(&mut self, path: &str, callback: F) -> Result<()>
    where
        F: Fn(
                Request,
                Response,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let route = Route::new(path, Method::Post, callback);

        let mut routes = self.routes.lock().await;

        routes.add(route);
        Ok(())
    }

    pub async fn put<F>(&mut self, path: &str, callback: F) -> Result<()>
    where
        F: Fn(
                Request,
                Response,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let route = Route::new(path, Method::Put, callback);

        let mut routes = self.routes.lock().await;

        routes.add(route);
        Ok(())
    }

    pub async fn patch<F>(&mut self, path: &str, callback: F) -> Result<()>
    where
        F: Fn(
                Request,
                Response,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let route = Route::new(path, Method::Patch, callback);

        let mut routes = self.routes.lock().await;

        routes.add(route);
        Ok(())
    }

    pub async fn delete<F>(&mut self, path: &str, callback: F) -> Result<()>
    where
        F: Fn(
                Request,
                Response,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>>
            + Send
            + Sync
            + 'static,
    {
        let route = Route::new(path, Method::Delete, callback);

        let mut routes = self.routes.lock().await;

        routes.add(route);
        Ok(())
    }

    pub async fn listen<A, B>(&mut self, addr: A, callback: B) -> Result<()>
    where
        A: ToSocketAddrs,
        B: Fn() -> Result<()>,
    {
        let listener = TcpListener::bind(addr).await?;
        callback()?;

        loop {
            let (socket, addr) = listener.accept().await?;
            println!("CONNECTED: {addr}");

            let mut handler =
                RequestHandler::from(Arc::clone(&self.routes), self.middlewares.clone());

            tokio::spawn(async move {
                handler.handle(socket).await.expect("Error: handler error");
            });
        }
    }
}
