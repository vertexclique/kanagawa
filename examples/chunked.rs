use tide::*;

async fn server() -> Result<()> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());
    app.at("/").get(|_| async {
        // File sends are chunked by default.
        Ok(Body::from_file(file!()).await?)
    });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}


fn main() -> Result<()> {
    block_on(server())
}