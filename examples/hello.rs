use kanagawa::*;

async fn server() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.with(kanagawa::log::LogMiddleware::new());

    app.at("/").get(|_| async { Ok("Hello, world!") });
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn main() -> Result<()> {
    block_on(server())
}