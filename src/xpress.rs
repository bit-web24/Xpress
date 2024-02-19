use tokio::io::Result;
use tokio::net::{TcpListener, ToSocketAddrs};

mod handler;
mod router;

use handler::RequestHandler;
use router::request::Request;
use router::response::Response;
use router::{Route, Router};
pub struct App<'a, F: Fn(Request, Response) -> Result<()> + Send + 'static + Clone> {
    name: &'a str,
    routes: Router<F>,
}

impl<'a, F> App<'a, F>
where
    F: Fn(Request, Response) -> Result<()> + Send + 'static + Clone,
{
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            routes: Router::<F>::new(),
        }
    }

    pub fn get(&mut self, route: &str, callback: F) -> Result<()> {
        let route = Route {
            path: String::from(route),
            method: router::Method::Get,
            callback,
        };

        self.routes.add(route);
        Ok(())
    }

    // pub fn post(&self, route: &str, callback: F) -> Result<()> {}
    // pub fn put(&self, route: &str, callback: F) -> Result<()> {}
    // pub fn delete(&self, route: &str, callback: F) -> Result<()> {}

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

            let mut handler = RequestHandler::from(self.routes.clone());

            tokio::spawn(async move {
                handler.handle(socket).await.expect("Error: handler error");
            })
            .await?;
        }
    }
}
