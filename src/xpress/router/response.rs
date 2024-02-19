use tokio::io::Result;

use tokio::{io::AsyncWriteExt, net::TcpStream};
pub struct Response {
    pub socket: TcpStream,
}

impl Response {
    pub fn new(socket: TcpStream) -> Self {
        Self { socket }
    }

    pub fn send(&mut self, msg: &str) -> Result<()> {
        self.socket.write_all(msg.as_bytes());
        Ok(())
    }
}
