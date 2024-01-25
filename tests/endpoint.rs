use kanagawa::http::{Method, Request, Url};
use kanagawa::Response;

#[nuclei::test]
async fn should_accept_boxed_endpoints() {
    fn endpoint() -> Box<dyn kanagawa::Endpoint<()>> {
        Box::new(|_| async { Ok("hello world") })
    }

    let mut app = kanagawa::Server::new();
    app.at("/").get(endpoint());

    let mut response: Response = app
        .respond(Request::new(
            Method::Get,
            Url::parse("http://example.com/").unwrap(),
        ))
        .await
        .unwrap();

    assert_eq!(
        response.take_body().into_string().await.unwrap(),
        "hello world"
    );
}
