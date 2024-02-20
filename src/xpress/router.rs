use std::{collections::HashMap, future::Future};

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

pub struct Route<F, Fut>
where
    F: Fn(Request, Response) -> Fut + Send + 'static + Clone,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    pub path: String,
    pub method: Method,
    pub callback: F,
}

#[derive(Clone)]

pub struct Router<F, Fut>
where
    F: Fn(Request, Response) -> Fut + Send + 'static + Clone,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    get: HashMap<String, Route<F, Fut>>,
    post: HashMap<String, Route<F, Fut>>,
    put: HashMap<String, Route<F, Fut>>,
    delete: HashMap<String, Route<F, Fut>>,
}

impl<F, Fut> Router<F, Fut>
where
    F: Fn(Request, Response) -> Fut + Send + 'static + Clone,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    pub fn new() -> Self {
        Self {
            get: HashMap::new(),
            post: HashMap::new(),
            put: HashMap::new(),
            delete: HashMap::new(),
        }
    }

    pub fn add(&mut self, route: Route<F, Fut>) {
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

    pub fn get(&mut self, path: &str) -> Option<&Route<F, Fut>> {
        if let Some(route) = self.get.get(path) {
            return Some(route);
        }

        None
    }
}
