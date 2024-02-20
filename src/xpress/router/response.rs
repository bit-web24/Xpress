use tokio::fs;
use tokio::io::Result;

use tokio::{io::AsyncWriteExt, net::TcpStream};
pub struct Response {
    pub socket: TcpStream,
}

impl Response {
    pub fn new(socket: TcpStream) -> Self {
        Self { socket }
    }

    pub async fn send(&mut self, msg: &str) -> Result<()> {
        let status_line = "HTTP/1.1 200 OK";
        let headers = format!(
            "Content-Type: text/plain\r\nContent-Length: {}\r\n",
            msg.len()
        );
        let response = format!("{}\r\n{}\r\n{}", status_line, headers, msg);
        self.socket.write_all(response.as_bytes()).await?;
        Ok(())
    }

    pub async fn send_file(&mut self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path).await?;
        let status_line = "HTTP/1.1 200 OK";
        let headers = format!(
            "Content-Type: text/html\r\nContent-Length: {}\r\n",
            content.len()
        );
        let response = format!("{}\r\n{}\r\n{}", status_line, headers, content);
        self.socket.write_all(response.as_bytes()).await?;
        Ok(())
    }
}
