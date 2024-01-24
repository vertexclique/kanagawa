use kanagawa::{*, sse};

async fn server() -> Result<()> {
    let mut app = kanagawa::new();
    app.with(kanagawa::log::LogMiddleware::new());
    app.at("/sse").get(sse::endpoint(|_req, sender| async move {
        sender.send("fruit", "banana", None).await?;
        sender.send("fruit", "apple", None).await?;
        Ok(())
    }));
    app.listen("localhost:8080").await?;
    Ok(())
}

fn main() -> Result<()> {
    block_on(server())
}
