# Xpress

Lightweight & Asynchronous Web Application Framework for Rust

# Usage
```rust
use xpress::Xpress;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Xpress::new();

    app.get("/", |_req, mut res| {
        Box::pin(async move {
            res.send("Welcome to the homepage!").await?;
            Ok(())
        })
    })
    .await?;

    app.get("/about", |_req, mut res| {
        Box::pin(async move {
            res.send("About Page!").await?;
            Ok(())
        })
    })
    .await?;

    app.listen("127.0.0.1:8080", || {
        println!("Server is running on http://localhost:8080");
        Ok(())
    })
    .await?;

    Ok(())
}

```
