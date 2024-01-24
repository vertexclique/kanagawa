use tide::*;

async fn server() -> tide::Result<()> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn main() -> Result<()> {
    block_on(server())
}