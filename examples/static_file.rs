use kanagawa::*;

async fn server() -> Result<()> {
    let mut app = kanagawa::new();
    app.with(kanagawa::log::LogMiddleware::new());
    app.at("/").get(|_| async { Ok("visit /src/*") });
    app.at("/src/*").serve_dir("src/")?;

    // Make sure examples/static_file.html is available relative to the current-dir this example is run from or replace this with an absolute path.
    app.at("/example").serve_file("examples/static_file.html")?;

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn main() -> Result<()> {
    block_on(server())
}