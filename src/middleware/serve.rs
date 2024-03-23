use super::Middleware;
use crate::router::{method, request::Request, Route, Router};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tokio::io::Result;

pub struct ServeStatic {
    root: String,
}

impl ServeStatic {
    pub fn dir(root: &str) -> Self {
        Self {
            root: Self::sanitize_path(root).to_str().unwrap().to_string(),
        }
    }

    fn sanitize_path(path: &str) -> PathBuf {
        let mut sanitized_path = PathBuf::new();
        for component in path.split('/') {
            if component == ".." || component == "." {
                // Ignore '..' and '.' components
                continue;
            }
            sanitized_path.push(component);
        }
        sanitized_path
    }
}

impl Middleware for ServeStatic {
    fn handle(&self, routes: &mut Router, req: &mut Request) -> Result<()> {
        if let Some(method::Method::Get) = req.method {
            if routes
                .get(&req.method.as_ref().unwrap(), req.path.as_str())
                .is_none()
            {
                let sanitized_path = Self::sanitize_path(req.path.as_str());
                let file_path = Path::new(&self.root).join(&sanitized_path);

                if let Err(_) = fs::metadata(&file_path) {
                    return Ok(());
                }

                req.path = file_path.to_str().unwrap().to_string();

                let route = Route::new(
                    file_path.to_str().unwrap(),
                    req.method.clone().unwrap(),
                    |reqst, mut res| {
                        {
                            Box::pin(async move {
                                res.send_file(&reqst.path).await?;
                                Ok(())
                            })
                        }
                    },
                );

                routes.add(route);
            }
        }

        Ok(())
    }
}
