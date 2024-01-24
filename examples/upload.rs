use std::io::{Error as IoError, Write};
use std::path::Path;
use std::sync::Arc;

use std::{fs::OpenOptions, io};
use std::fs::File;
use kv_log_macro::info;
use tempfile::TempDir;
use kanagawa::prelude::*;
use kanagawa::*;
use futures::AsyncWriteExt;

#[derive(Clone)]
struct TempDirState {
    tempdir: Arc<TempDir>,
}

impl TempDirState {
    fn try_new() -> Result<Self> {
        Ok(Self {
            tempdir: Arc::new(tempfile::tempdir()?),
        })
    }

    fn path(&self) -> &Path {
        self.tempdir.path()
    }
}

async fn server() -> Result<()> {
    let mut app = kanagawa::with_state(TempDirState::try_new()?);
    app.with(kanagawa::log::LogMiddleware::new());

    // To test this example:
    // $ cargo run --example upload
    // $ curl -T ./README.md localhost:8080 # this writes the file to a temp directory
    // $ curl localhost:8080/README.md # this reads the file from the same temp directory

    app.at(":file")
        .put(|mut req: Request<TempDirState>| async move {
            let path = req.param("file")?;
            let fs_path = req.state().path().join(path);

            let file = OpenOptions::new()
                .create(true)
                .write(true)
                .open(&fs_path).unwrap();

            let mut file = Handle::<File>::new(file).unwrap();

            // io::copy is not needed, since uring's copy is better than std impl.
            let body = req.body_bytes().await?;
            file.write_all(body.as_slice()).await?;
            let bytes_written = body.len();

            info!("file written", {
                bytes: bytes_written,
                path: fs_path.canonicalize()?.to_str()
            });

            Ok(json!({ "bytes": bytes_written }))
        })
        .get(|req: Request<TempDirState>| async move {
            let path = req.param("file")?;
            let fs_path = req.state().path().join(path);

            if let Ok(body) = Body::from_file(fs_path).await {
                Ok(body.into())
            } else {
                Ok(Response::new(StatusCode::NotFound))
            }
        });

    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

fn main() -> Result<()> {
    block_on(server())
}