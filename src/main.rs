mod xpress;
use xpress::router::{request::Request, response::Response};
use xpress::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new("Application");

    app.get("/", |req: Request, mut res: Response| {
        Box::pin(async move {
            res.send("Welcome to the homepage!").await?;
            Ok(())
        })
    })
    .await?;

    app.get("/about", |req: Request, mut res: Response| {
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
