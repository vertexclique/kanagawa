use kanagawa::http::Cookie;
use kanagawa::*;

/// Kanagawa will use the the `Cookies`'s `Extract` implementation to build this parameter.
///
async fn retrieve_cookie(req: Request<()>) -> kanagawa::Result<String> {
    Ok(format!("hello cookies: {:?}", req.cookie("hello").unwrap()))
}

async fn insert_cookie(_req: Request<()>) -> kanagawa::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.insert_cookie(Cookie::new("hello", "world"));
    Ok(res)
}

async fn remove_cookie(_req: Request<()>) -> kanagawa::Result {
    let mut res = Response::new(StatusCode::Ok);
    res.remove_cookie(Cookie::named("hello"));
    Ok(res)
}

async fn server() -> Result<()> {
    let mut app = kanagawa::new();
    app.with(kanagawa::log::LogMiddleware::new());

    app.at("/").get(retrieve_cookie);
    app.at("/set").get(insert_cookie);
    app.at("/remove").get(remove_cookie);
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

fn main() -> kanagawa::Result<()> {
    block_on(server())
}