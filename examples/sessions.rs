use kanagawa::*;

async fn server() -> Result<()> {
    let mut app = kanagawa::new();
    app.with(kanagawa::log::LogMiddleware::new());

    app.with(kanagawa::sessions::SessionMiddleware::new(
        kanagawa::sessions::MemoryStore::new(),
        std::env::var("KANAGAWA_SECRET")
            .expect(
                "Please provide a KANAGAWA_SECRET value of at \
                      least 32 bytes in order to run this example",
            )
            .as_bytes(),
    ));

    app.with(kanagawa::utils::Before(
        |mut request: kanagawa::Request<()>| async move {
            let session = request.session_mut();
            let visits: usize = session.get("visits").unwrap_or_default();
            session.insert("visits", visits + 1).unwrap();
            request
        },
    ));

    app.at("/").get(|req: kanagawa::Request<()>| async move {
        let visits: usize = req.session().get("visits").unwrap();
        Ok(format!("you have visited this website {} times", visits))
    });

    app.at("/reset")
        .get(|mut req: kanagawa::Request<()>| async move {
            req.session_mut().destroy();
            Ok(kanagawa::Redirect::new("/"))
        });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

fn main() -> kanagawa::Result<()> {
    block_on(server())
}
