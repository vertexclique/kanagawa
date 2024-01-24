use std::io::ErrorKind;

use kanagawa::utils::After;
use kanagawa::*;

async fn server() -> Result<()> {
    let mut app = kanagawa::new();
    app.with(kanagawa::log::LogMiddleware::new());

    app.with(After(|mut res: Response| async {
        if let Some(err) = res.downcast_error::<std::io::Error>() {
            if let ErrorKind::NotFound = err.kind() {
                let msg = format!("Error: {:?}", err);
                res.set_status(StatusCode::NotFound);

                // NOTE: You may want to avoid sending error messages in a production server.
                res.set_body(msg);
            }
        }
        Ok(res)
    }));

    app.at("/")
        .get(|_req: Request<_>| async { Ok(Body::from_file("./does-not-exist").await?) });

    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

fn main() -> kanagawa::Result<()> {
    block_on(server())
}