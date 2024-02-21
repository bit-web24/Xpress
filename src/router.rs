use self::{request::Request, response::Response};
use std::{collections::HashMap, sync::Arc};
use tokio::io::Result;
pub mod request;
pub mod response;

#[derive(Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Clone)]
pub struct Route {
    pub path: String,
    pub method: Method,
    pub callback: Arc<
        dyn Fn(
                Request,
                Response,
            )
                -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<()>> + Send>>
            + Send
            + Sync
            + 'static,
    >,
}

impl Route {
    pub fn new<F>(path: &str, method: Method, callback: F) -> Self
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
        Route {
            path: String::from(path),
            method,
            callback: Arc::new(callback),
        }
    }
}

#[derive(Clone)]

pub struct Router {
    get: HashMap<String, Route>,
    post: HashMap<String, Route>,
    put: HashMap<String, Route>,
    delete: HashMap<String, Route>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            get: HashMap::new(),
            post: HashMap::new(),
            put: HashMap::new(),
            delete: HashMap::new(),
        }
    }

    pub fn add(&mut self, route: Route) {
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

    pub fn get(&mut self, path: &str) -> Option<&Route> {
        if let Some(route) = self.get.get(path) {
            return Some(route);
        }

        None
    }
}
