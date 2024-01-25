mod test_utils;
use test_utils::ServerTestingExt;

#[nuclei::test]
async fn nested() -> kanagawa::Result<()> {
    let mut inner = kanagawa::new();
    inner.at("/foo").get(|_| async { Ok("foo") });
    inner.at("/bar").get(|_| async { Ok("bar") });

    let mut outer = kanagawa::new();
    // Nest the inner app on /foo
    outer.at("/foo").nest(inner);

    assert_eq!(outer.get("/foo/foo").recv_string().await?, "foo");
    assert_eq!(outer.get("/foo/bar").recv_string().await?, "bar");
    Ok(())
}

#[nuclei::test]
async fn nested_middleware() -> kanagawa::Result<()> {
    let echo_path = |req: kanagawa::Request<()>| async move { Ok(req.url().path().to_string()) };
    let mut app = kanagawa::new();
    let mut inner_app = kanagawa::new();
    inner_app.with(kanagawa::utils::After(|mut res: kanagawa::Response| async move {
        res.insert_header("x-kanagawa-test", "1");
        Ok(res)
    }));
    inner_app.at("/echo").get(echo_path);
    inner_app.at("/:foo/bar").strip_prefix().get(echo_path);
    app.at("/foo").nest(inner_app);
    app.at("/bar").get(echo_path);

    let mut res = app.get("/foo/echo").await?;
    assert_eq!(res["X-Kanagawa-Test"], "1");
    assert_eq!(res.status(), 200);
    assert_eq!(res.body_string().await?, "/echo");

    let mut res = app.get("/foo/x/bar").await?;
    assert_eq!(res["X-Kanagawa-Test"], "1");
    assert_eq!(res.status(), 200);
    assert_eq!(res.body_string().await?, "/");

    let mut res = app.get("/bar").await?;
    assert!(res.header("X-Kanagawa-Test").is_none());
    assert_eq!(res.status(), 200);
    assert_eq!(res.body_string().await?, "/bar");
    Ok(())
}

#[nuclei::test]
async fn nested_with_different_state() -> kanagawa::Result<()> {
    let mut outer = kanagawa::new();
    let mut inner = kanagawa::with_state(42);
    inner.at("/").get(|req: kanagawa::Request<i32>| async move {
        let num = req.state();
        Ok(format!("the number is {}", num))
    });
    outer.at("/").get(|_| async { Ok("Hello, world!") });
    outer.at("/foo").nest(inner);

    assert_eq!(outer.get("/foo").recv_string().await?, "the number is 42");
    assert_eq!(outer.get("/").recv_string().await?, "Hello, world!");
    Ok(())
}
