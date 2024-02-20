mod xpress;
use xpress::App;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new("Application");

    app.get(String::from("/"), |req, mut res| async move {
        // res.send("Welcome to the homepage!").await?;
        let current_dir = std::env::current_dir().expect("Failed to get current directory");
        let file_path = current_dir.join("public").join("index.html");
        let file_path_str = file_path.to_string_lossy();
        // println!("FILE PATH: {}", file_path_str);

        res.send_file(&file_path_str).await?;
        Ok(())
    })
    .await?;

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
