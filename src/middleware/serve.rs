use crate::{middleware::Middleware, router::request::Request};

pub struct ServeStatic {
    root: String,
}

impl ServeStatic {
    pub fn dir(root: &str) -> Self {
        Self {
            root: root.to_string(),
        }
    }
}

impl Middleware for ServeStatic {
    fn handle(&self, req: &mut Request) -> std::io::Result<()> {
        Ok(())
    }
}
