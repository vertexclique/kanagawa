use tracing::{error, info, warn};

use crate::{Middleware, Next, Request};

/// Log all incoming requests and responses.
///
/// In the case of nested applications, this middleware will only run once for each request.
///
/// # Examples
///
/// ```
/// let mut app = kanagawa::Server::new();
/// app.with(kanagawa::log::LogMiddleware::new());
/// ```
#[derive(Debug, Default, Clone)]
pub struct LogMiddleware {
    _priv: (),
}

struct LogMiddlewareHasBeenRun;

impl LogMiddleware {
    /// Create a new instance of `LogMiddleware`.
    #[must_use]
    pub fn new() -> Self {
        Self { _priv: () }
    }

    /// Log a request and a response.
    async fn log<'a, State: Clone + Send + Sync + 'static>(
        &'a self,
        mut req: Request<State>,
        next: Next<'a, State>,
    ) -> crate::Result {
        if req.ext::<LogMiddlewareHasBeenRun>().is_some() {
            return Ok(next.run(req).await);
        }
        req.set_ext(LogMiddlewareHasBeenRun);

        let path = req.url().path().to_owned();
        let method = req.method().to_string();
        info!(method = ?method, path = ?path, "<-- Request received");
        let start = std::time::Instant::now();
        let response = next.run(req).await;
        let status = response.status();
        if status.is_server_error() {
            if let Some(error) = response.error() {
                error!(message = format!("{:?}", error),
                    error_type = error.type_name(),
                    method = method,
                    path = path,
                    status = format!("{} - {}", status as u16, status.canonical_reason()),
                    duration = format!("{:?}", start.elapsed()),
                    "Internal error --> Response sent",
                );
            } else {
                error!(method = method,
                    path = path,
                    status = format!("{} - {}", status as u16, status.canonical_reason()),
                    duration = format!("{:?}", start.elapsed()),
                    "Internal error --> Response sent"
                );
            }
        } else if status.is_client_error() {
            if let Some(error) = response.error() {
                warn!(message = format!("{:?}", error),
                    error_type = error.type_name(),
                    method = method,
                    path = path,
                    status = format!("{} - {}", status as u16, status.canonical_reason()),
                    duration = format!("{:?}", start.elapsed()),
                    "Client error --> Response sent"
                );
            } else {
                warn!(method = method,
                    path = path,
                    status = format!("{} - {}", status as u16, status.canonical_reason()),
                    duration = format!("{:?}", start.elapsed()),
                    "Client error --> Response sent"
                );
            }
        } else {
            info!(method = method,
                path = path,
                status = format!("{} - {}", status as u16, status.canonical_reason()),
                duration = format!("{:?}", start.elapsed()),
                "--> Response sent"
            );
        }
        Ok(response)
    }
}

#[async_trait::async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for LogMiddleware {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> crate::Result {
        self.log(req, next).await
    }
}
