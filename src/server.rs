use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("192.168.25.172:8080").await?;
    println!("Listening on port 8080");
    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("NEW CONNECTION: {}", addr);
        tokio::spawn(async move {
            let mut buf = [0; 1024];
            let n = socket
                .read(&mut buf)
                .await
                .expect("ERROR: Could not read the request line!");
            println!("RECEIVED: {}", String::from_utf8_lossy(&buf[..n]));

            let content = fs::read_to_string("./public/index.html")
                .await
                .expect("ERROR: File Not Found");
            let req_ln = "HTTP/1.1 200 OK";
            let response = format!(
                "{}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
                req_ln,
                content.len(),
                content
            );

            socket
                .write_all(response.as_bytes())
                .await
                .expect("ERROR: Could not write to socket!");
        })
        .await?;
    }
}
