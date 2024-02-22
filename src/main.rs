use xpress::path::Path;
use xpress::Xpress;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Xpress::new();

    app.get("/", |_req, mut res| {
        Box::pin(async move {
            let path = Path::new("public");
            res.send_file(path.join("index.html").to_str()).await?;
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
