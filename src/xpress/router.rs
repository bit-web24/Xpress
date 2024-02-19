use std::collections::HashMap;

pub mod request;
pub mod response;

use request::Request;
use response::Response;
use tokio::io::Result;

#[derive(Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Clone)]

pub struct Route<F>
where
    F: Fn(Request, Response) -> Result<()> + Clone,
{
    pub path: String,
    pub method: Method,
    pub callback: F,
}

#[derive(Clone)]

pub struct Router<F>
where
    F: Fn(Request, Response) -> Result<()> + Clone,
{
    get: HashMap<String, Route<F>>,
    post: HashMap<String, Route<F>>,
    put: HashMap<String, Route<F>>,
    delete: HashMap<String, Route<F>>,
}

impl<F> Router<F>
where
    F: Fn(Request, Response) -> Result<()> + Clone,
{
    pub fn new() -> Self {
        Self {
            get: HashMap::new(),
            post: HashMap::new(),
            put: HashMap::new(),
            delete: HashMap::new(),
        }
    }

    pub fn add(&mut self, route: Route<F>) {
        match route.method {
            Method::Get => {
                self.get.insert(route.path.clone(), route);
            }
            Method::Post => {
                self.post.insert(route.path.clone(), route);
            }
            Method::Put => {
                self.put.insert(route.path.clone(), route);
            }
            Method::Delete => {
                self.delete.insert(route.path.clone(), route);
            }
        }
    }

    pub fn get(&mut self, path: &str) -> Option<&Route<F>> {
        if let Some(route) = self.get.get(path) {
            return Some(route);
        }

        None
    }
}
