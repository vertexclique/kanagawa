use http_types::{self, Url};
use kanagawa::{self, Method, Request, Response, Result};

#[nuclei::test]
async fn test_missing_param() -> kanagawa::Result<()> {
    async fn greet(req: Request<()>) -> Result<Response> {
        assert_eq!(req.param("name")?, "Param \"name\" not found");
        Ok(Response::new(200))
    }

    let mut server = kanagawa::new();
    server.at("/").get(greet);

    let req = http_types::Request::new(Method::Get, Url::parse("http://example.com/")?);
    let res: http_types::Response = server.respond(req).await?;
    assert_eq!(res.status(), 500);
    Ok(())
}

#[nuclei::test]
async fn hello_world_parametrized() -> Result<()> {
    async fn greet(req: kanagawa::Request<()>) -> Result<impl Into<Response>> {
        let body = format!("{} says hello", req.param("name").unwrap_or("nori"));
        Ok(Response::builder(200).body(body))
    }

    let mut server = kanagawa::new();
    server.at("/").get(greet);
    server.at("/:name").get(greet);

    let req = http_types::Request::new(Method::Get, Url::parse("http://example.com/")?);
    let mut res: http_types::Response = server.respond(req).await?;
    assert_eq!(res.body_string().await?, "nori says hello");

    let req = http_types::Request::new(Method::Get, Url::parse("http://example.com/iron")?);
    let mut res: http_types::Response = server.respond(req).await?;
    assert_eq!(res.body_string().await?, "iron says hello");
    Ok(())
}
