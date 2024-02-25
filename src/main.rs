use xpress::path::Path;
use xpress::Xpress;

mod body_parser;
use body_parser::BodyParser;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = Xpress::new();

    app._use_(BodyParser::json()).await;

    app.get("/", |_req, mut res| {
        Box::pin(async move {
            let path = Path::new("public");
            res.send_file(path.join("index.html").to_str()).await?;
            Ok(())
        })
    })
    .await?;

    app.post("/", |req, mut res| {
        Box::pin(async move {
            if let Some(body) = req.body {
                println!("{}", body.raw.unwrap());
                println!("deser: {}", body.json.unwrap()["key"]);
            } else {
                res.send("No Data!").await?;
            }

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
