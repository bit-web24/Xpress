use crate::router::{request::Request, Router};
use tokio::io::Result;
pub mod body_parser;
pub mod serve;

pub trait Middleware: Send + Sync {
    fn handle(&self, routes: &mut Router, req: &mut Request) -> Result<()>;
}
