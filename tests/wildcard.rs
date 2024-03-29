mod test_utils;
use test_utils::ServerTestingExt;
use kanagawa::{Error, Request, StatusCode};

async fn add_one(req: Request<()>) -> Result<String, kanagawa::Error> {
    let num: i64 = req
        .param("num")?
        .parse()
        .map_err(|err| Error::new(StatusCode::BadRequest, err))?;
    Ok((num + 1).to_string())
}

async fn add_two(req: Request<()>) -> Result<String, kanagawa::Error> {
    let one: i64 = req
        .param("one")?
        .parse()
        .map_err(|err| Error::new(StatusCode::BadRequest, err))?;
    let two: i64 = req
        .param("two")?
        .parse()
        .map_err(|err| Error::new(StatusCode::BadRequest, err))?;
    Ok((one + two).to_string())
}

async fn echo_param(req: Request<()>) -> kanagawa::Result<kanagawa::Response> {
    match req.param("param") {
        Ok(path) => Ok(path.into()),
        Err(_) => Ok(StatusCode::NotFound.into()),
    }
}

async fn echo_wildcard(req: Request<()>) -> kanagawa::Result<kanagawa::Response> {
    match req.wildcard() {
        Some(path) => Ok(path.into()),
        None => Ok(StatusCode::NotFound.into()),
    }
}

#[nuclei::test]
async fn param() -> kanagawa::Result<()> {
    let mut app = kanagawa::Server::new();
    app.at("/add_one/:num").get(add_one);
    assert_eq!(app.get("/add_one/3").recv_string().await?, "4");
    assert_eq!(app.get("/add_one/-7").recv_string().await?, "-6");
    Ok(())
}

#[nuclei::test]
async fn invalid_segment_error() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.at("/add_one/:num").get(add_one);
    assert_eq!(
        app.get("/add_one/a").await?.status(),
        StatusCode::BadRequest
    );
    Ok(())
}

#[nuclei::test]
async fn not_found_error() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.at("/add_one/:num").get(add_one);
    assert_eq!(app.get("/add_one/").await?.status(), StatusCode::NotFound);
    Ok(())
}

#[nuclei::test]
async fn wildcard() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.at("/echo/*").get(echo_wildcard);
    assert_eq!(app.get("/echo/some_path").recv_string().await?, "some_path");
    assert_eq!(
        app.get("/echo/multi/segment/path").recv_string().await?,
        "multi/segment/path"
    );
    assert_eq!(app.get("/echo/").await?.status(), StatusCode::Ok);
    assert_eq!(app.get("/echo").await?.status(), StatusCode::Ok);
    Ok(())
}

#[nuclei::test]
async fn multi_param() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.at("/add_two/:one/:two/").get(add_two);
    assert_eq!(app.get("/add_two/1/2/").recv_string().await?, "3");
    assert_eq!(app.get("/add_two/-1/2/").recv_string().await?, "1");
    assert_eq!(app.get("/add_two/1").await?.status(), StatusCode::NotFound);
    Ok(())
}

#[nuclei::test]
async fn wildcard_last_segment() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.at("/echo/:param/*").get(echo_param);
    assert_eq!(app.get("/echo/one/two").recv_string().await?, "one");
    assert_eq!(
        app.get("/echo/one/two/three/four").recv_string().await?,
        "one"
    );
    Ok(())
}

#[nuclei::test]
async fn ambiguous_router_wildcard_vs_star() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.at("/:one/:two").get(|_| async { Ok("one/two") });
    app.at("/posts/*").get(|_| async { Ok("posts/*") });
    assert_eq!(app.get("/posts/10").recv_string().await?, "posts/*");
    Ok(())
}
