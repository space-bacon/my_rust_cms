use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, HttpMessage, HttpResponse};
use actix_service::Service;
use futures::future::{ok, Ready};
use crate::services::auth_service::AuthService;

pub struct AuthMiddleware;

impl<S, B> actix_service::Transform<S> for AuthMiddleware
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service for AuthMiddlewareService<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if let Some(auth_header) = req.headers().get("Authorization") {
            let token = auth_header.to_str().unwrap_or("");
            if AuthService::validate_token(token).is_ok() {
                return self.service.call(req);
            }
        }

        let (req, _) = req.into_parts();
        let response = HttpResponse::Unauthorized().finish().map_into_right_body();
        Box::pin(async { Ok(ServiceResponse::new(req, response)) })
    }
}
