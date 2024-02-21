mod xpress;
use xpress::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new("Application");

    app.get("/", |req, mut res| async move {
        res.send("Welcome to the homepage!").await?;
        Ok(())
    })
    .await?;

    app.get("/", |req, mut res| async move {
        res.send("About Us").await?;
        Ok(())
    })
    .await?;

    app.listen("127.0.0.1:8080", || {
        println!("Server is running on http://localhost:8080");
        Ok(())
    })
    .await?;

    Ok(())
}
