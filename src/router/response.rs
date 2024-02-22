use super::header::Header;
use super::status::Status;
use tokio::fs;
use tokio::io::Result;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub struct Response {
    socket: TcpStream,
    pub headers: Header,
    pub status: Status,
}

impl Response {
    pub fn new(socket: TcpStream) -> Self {
        Self {
            socket,
            headers: Header::new(),
            status: Status::new(),
        }
    }

    pub async fn send(&mut self, msg: &str) -> Result<()> {
        self.status.status_code = 200;
        self.headers.set("Content-Type", "text/plain");
        self.headers
            .set("Content-Length", msg.len().to_string().as_str());

        let response = format!(
            "{}\r\n{}\r\n{}",
            self.status.to_string(),
            self.headers.to_string(),
            msg
        );
        self.socket.write_all(response.as_bytes()).await?;
        Ok(())
    }

    pub async fn send_file(&mut self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path).await?;
        self.status.status_code = 200;
        self.headers.set("Content-Type", "text/html");
        self.headers
            .set("Content-Length", content.len().to_string().as_str());

        let response = format!(
            "{}\r\n{}\r\n{}",
            self.status.to_string(),
            self.headers.to_string(),
            content
        );
        self.socket.write_all(response.as_bytes()).await?;
        Ok(())
    }
}
