use std::future::Future;
use std::pin::Pin;
use kanagawa::http::{self, url::Url, Method};

mod test_utils;

fn auth_middleware<'a>(
    request: kanagawa::Request<()>,
    next: kanagawa::Next<'a, ()>,
) -> Pin<Box<dyn Future<Output = kanagawa::Result> + 'a + Send>> {
    let authenticated = match request.header("X-Auth") {
        Some(header) => header == "secret_key",
        None => false,
    };

    Box::pin(async move {
        if authenticated {
            Ok(next.run(request).await)
        } else {
            Ok(kanagawa::Response::new(kanagawa::StatusCode::Unauthorized))
        }
    })
}

async fn echo_path<State>(req: kanagawa::Request<State>) -> kanagawa::Result<String> {
    Ok(req.url().path().to_string())
}

#[async_std::test]
async fn route_middleware() {
    let mut app = kanagawa::new();
    app.at("/protected").with(auth_middleware).get(echo_path);
    app.at("/unprotected").get(echo_path);

    // Protected
    let req = http::Request::new(
        Method::Get,
        Url::parse("http://localhost/protected").unwrap(),
    );
    let res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), kanagawa::StatusCode::Unauthorized);

    let mut req = http::Request::new(
        Method::Get,
        Url::parse("http://localhost/protected").unwrap(),
    );
    req.insert_header("X-Auth", "secret_key");
    let res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), kanagawa::StatusCode::Ok);

    // Unprotected
    let req = http::Request::new(
        Method::Get,
        Url::parse("http://localhost/unprotected").unwrap(),
    );
    let res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), kanagawa::StatusCode::Ok);

    let mut req = http::Request::new(
        Method::Get,
        Url::parse("http://localhost/unprotected").unwrap(),
    );
    req.insert_header("X-Auth", "secret_key");
    let res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), kanagawa::StatusCode::Ok);
}

#[async_std::test]
async fn app_middleware() {
    let mut app = kanagawa::new();
    app.with(auth_middleware);
    app.at("/foo").get(echo_path);

    // Foo
    let req = http::Request::new(Method::Get, Url::parse("http://localhost/foo").unwrap());
    let res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), kanagawa::StatusCode::Unauthorized);

    let mut req = http::Request::new(Method::Get, Url::parse("http://localhost/foo").unwrap());
    req.insert_header("X-Auth", "secret_key");
    let res: http::Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), kanagawa::StatusCode::Ok);
}
