use serde::{Deserialize, Serialize};
use tide::prelude::*; // Pulls in the json! macro.
use tide::*;

#[derive(Deserialize, Serialize)]
struct Cat {
    name: String,
}

async fn server() -> tide::Result<()> {
    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/submit").post(|mut req: Request<()>| async move {
        let cat: Cat = req.body_json().await?;
        println!("cat name: {}", cat.name);

        let cat = Cat {
            name: "chashu".into(),
        };

        Body::from_json(&cat)
    });

    app.at("/animals").get(|_| async {
        Ok(json!({
            "meta": { "count": 2 },
            "animals": [
                { "type": "cat", "name": "chashu" },
                { "type": "cat", "name": "nori" }
            ]
        }))
    });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn main() -> tide::Result<()> {
    block_on(server())
}