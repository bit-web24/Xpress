use tokio::io::Result;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};

mod router;

use router::request::Request;
use router::response::Response;
use router::{Route, Router};

pub struct App<'a, F: Fn(Request, Response) -> Result<()> + Send + 'static> {
    name: &'a str,
    routes: Router<F>,
}

impl<'a, F> App<'a, F>
where
    F: Fn(Request, Response) -> Result<()> + Send + 'static,
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
            let (mut socket, addr) = listener.accept().await?;
            println!("CONNECTED: {addr}");

            tokio::spawn(async move {
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
                    "GET" => match self.routes.get(path).as_ref() {
                        Some(&route) => (route.callback)(req, res),
                        None => {
                            panic!("404 Page Not Found!");
                        }
                    },
                    // "POST" => {}
                    // "PUT" => {}
                    // "DELETE" => {}
                    _ => panic!("undefined HTTP method"),
                }
            })
            .await?;
        }
    }
}
