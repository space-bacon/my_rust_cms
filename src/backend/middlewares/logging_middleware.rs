use std::task::{Context, Poll};
use std::time::Instant;
use axum::{
    middleware::Next,
    http::{Request, StatusCode},
    response::Response,
};
use tower_layer::Layer;
use tower_service::Service;
use log::info;

/// Middleware struct to log request and response data
pub struct LoggingMiddleware<S> {
    inner: S,
}

impl<S> LoggingMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

/// Layer for logging middleware, to be added to the stack
pub struct LoggingLayer;

impl<S> Layer<S> for LoggingLayer {
    type Service = LoggingMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggingMiddleware::new(inner)
    }
}

/// Middleware implementation to log requests
impl<S, B> Service<Request<B>> for LoggingMiddleware<S>
where
    S: Service<Request<B>, Response = Response> + Clone,
{
    type Response = Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<B>) -> Self::Future {
        let method = req.method().clone();
        let path = req.uri().path().to_string();
        let start = Instant::now();

        let fut = self.inner.call(req);

        // Log the request details and execution time
        Box::pin(async move {
            let response = fut.await?;
            let status = response.status();
            let duration = start.elapsed();
            info!("{} {} -> {} (in {:?})", method, path, status, duration);
            Ok(response)
        })
    }
}

/// Apply the logging middleware in your application
pub fn logging_middleware() -> LoggingLayer {
    LoggingLayer
}
