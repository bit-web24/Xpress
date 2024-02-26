use crate::router::request::Request;
use tokio::io::Result;
pub mod body_parser;
pub mod serve;

pub trait Middleware: Send + Sync {
    fn handle(&self, req: &mut Request) -> Result<()>;
}
