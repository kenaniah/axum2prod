use http::Request;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;
use uuid::Uuid;

pub type RequestId = Uuid;

#[derive(Debug, Clone)]
pub struct RequestIdService<S> {
    inner: S,
}

/// Middleware that adds a `RequestId` to each request.
impl<S> RequestIdService<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<B, S> Service<Request<B>> for RequestIdService<S>
where
    S: Service<Request<B>>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    #[inline]
    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<B>) -> Self::Future {
        let request_id = RequestId::new_v4();
        req.extensions_mut().insert(request_id);
        self.inner.call(req)
    }
}

#[derive(Debug, Clone)]
pub struct RequestIdLayer;

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService::new(inner)
    }
}
