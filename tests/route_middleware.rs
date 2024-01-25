mod test_utils;
use http_types::headers::HeaderName;
use test_utils::ServerTestingExt;
use kanagawa::Middleware;

#[derive(Debug)]
struct TestMiddleware(HeaderName, &'static str);

impl TestMiddleware {
    fn with_header_name(name: &'static str, value: &'static str) -> Self {
        Self(name.into(), value)
    }
}

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for TestMiddleware {
    async fn handle(
        &self,
        req: kanagawa::Request<State>,
        next: kanagawa::Next<'_, State>,
    ) -> kanagawa::Result<kanagawa::Response> {
        let mut res = next.run(req).await;
        res.insert_header(self.0.clone(), self.1);
        Ok(res)
    }
}

async fn echo_path<State>(req: kanagawa::Request<State>) -> kanagawa::Result<String> {
    Ok(req.url().path().to_string())
}

#[nuclei::test]
async fn route_middleware() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    let mut foo_route = app.at("/foo");
    foo_route // /foo
        .with(TestMiddleware::with_header_name("X-Foo", "foo"))
        .get(echo_path);
    foo_route
        .at("/bar") // nested, /foo/bar
        .with(TestMiddleware::with_header_name("X-Bar", "bar"))
        .get(echo_path);
    foo_route // /foo
        .post(echo_path)
        .reset_middleware()
        .put(echo_path);

    assert_eq!(app.get("/foo").await?["X-Foo"], "foo");
    assert_eq!(app.post("/foo").await?["X-Foo"], "foo");
    assert!(app.put("/foo").await?.header("X-Foo").is_none());

    let res = app.get("/foo/bar").await?;
    assert_eq!(res["X-Foo"], "foo");
    assert_eq!(res["x-bar"], "bar");
    Ok(())
}

#[nuclei::test]
async fn app_and_route_middleware() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.with(TestMiddleware::with_header_name("X-Root", "root"));
    app.at("/foo")
        .with(TestMiddleware::with_header_name("X-Foo", "foo"))
        .get(echo_path);
    app.at("/bar")
        .with(TestMiddleware::with_header_name("X-Bar", "bar"))
        .get(echo_path);

    let res = app.get("/foo").await?;
    assert_eq!(res["X-Root"], "root");
    assert_eq!(res["x-foo"], "foo");
    assert!(res.header("x-bar").is_none());

    let res = app.get("/bar").await?;
    assert_eq!(res["X-Root"], "root");
    assert!(res.header("x-foo").is_none());
    assert_eq!(res["X-Bar"], "bar");
    Ok(())
}

#[nuclei::test]
async fn nested_app_with_route_middleware() -> kanagawa::Result<()> {
    let mut inner = kanagawa::new();
    inner.with(TestMiddleware::with_header_name("X-Inner", "inner"));
    inner
        .at("/baz")
        .with(TestMiddleware::with_header_name("X-Baz", "baz"))
        .get(echo_path);

    let mut app = kanagawa::new();
    app.with(TestMiddleware::with_header_name("X-Root", "root"));
    app.at("/foo")
        .with(TestMiddleware::with_header_name("X-Foo", "foo"))
        .get(echo_path);
    app.at("/bar")
        .with(TestMiddleware::with_header_name("X-Bar", "bar"))
        .nest(inner);

    let res = app.get("/foo").await?;
    assert_eq!(res["X-Root"], "root");
    assert!(res.header("X-Inner").is_none());
    assert_eq!(res["X-Foo"], "foo");
    assert!(res.header("X-Bar").is_none());
    assert!(res.header("X-Baz").is_none());

    let res = app.get("/bar/baz").await?;
    assert_eq!(res["X-Root"], "root");
    assert_eq!(res["X-Inner"], "inner");
    assert!(res.header("X-Foo").is_none());
    assert_eq!(res["X-Bar"], "bar");
    assert_eq!(res["X-Baz"], "baz");
    Ok(())
}

#[nuclei::test]
async fn subroute_not_nested() -> kanagawa::Result<()> {
    let mut app = kanagawa::new();
    app.at("/parent") // /parent
        .with(TestMiddleware::with_header_name("X-Parent", "Parent"))
        .get(echo_path);
    app.at("/parent/child") // /parent/child, not nested
        .with(TestMiddleware::with_header_name("X-Child", "child"))
        .get(echo_path);

    let res = app.get("/parent/child").await?;
    assert!(res.header("X-Parent").is_none());
    assert_eq!(res["x-child"], "child");
    Ok(())
}
