use crate::router::request::Request;
use tokio::io::Result;

pub trait Middleware: Send + Sync {
    fn handle(&self, req: &mut Request) -> Result<()>;
}
