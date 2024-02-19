mod xpress;
use xpress::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new("Application");

    app.get("/", |req, mut res| {
        res.send("Welcome to the homepage!");
        // res.sendFile("index.html")?;
        Ok(())
    })?;

    // app.get("/about", |req, res| {
    //     // res.send("About Us");
    //     Ok(())
    // });

    app.listen("127.0.0.1:8080", || {
        println!("Server is running )on http://localhost:8080");
        Ok(())
    })
    .await?;

    Ok(())
}
