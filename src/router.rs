use self::{request::Request, response::Response};
use std::{collections::HashMap, sync::Arc};
use tokio::io::Result;

pub mod header;
pub mod method;
pub mod request;
pub mod response;
pub mod status;

use method::Method;

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
    pub get: HashMap<String, Route>,
    pub post: HashMap<String, Route>,
    pub put: HashMap<String, Route>,
    pub patch: HashMap<String, Route>,
    pub delete: HashMap<String, Route>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            get: HashMap::new(),
            post: HashMap::new(),
            put: HashMap::new(),
            patch: HashMap::new(),
            delete: HashMap::new(),
        }
    }

    pub fn add(&mut self, route: Route) {
        match route.method {
            Method::Get => {
                self.get.entry(route.path.clone()).or_insert(route);
            }
            Method::Post => {
                self.post.entry(route.path.clone()).or_insert(route);
            }
            Method::Put => {
                self.put.entry(route.path.clone()).or_insert(route);
            }
            Method::Patch => {
                self.patch.entry(route.path.clone()).or_insert(route);
            }
            Method::Delete => {
                self.delete.entry(route.path.clone()).or_insert(route);
            }
            _ => (),
        }
    }

    pub fn get(&mut self, method: &Method, path: &str) -> Option<&Route> {
        match method {
            Method::Get => self.get.get(path),
            Method::Post => self.post.get(path),
            Method::Put => self.put.get(path),
            Method::Patch => self.patch.get(path),
            Method::Delete => self.delete.get(path),
            _ => None,
        }
    }
}
