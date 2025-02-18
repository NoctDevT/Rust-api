use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, body::BoxBody,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use std::env;
use std::rc::Rc;
use std::task::{Context, Poll};
use crate::models::claims::Claims; 
pub struct AuthMiddleware; 

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static, 
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct AuthMiddlewareMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = Rc::clone(&self.service);

        Box::pin(async move {
            let secret_key = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

            if let Some(auth_header) = req.headers().get("Authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if let Some(token) = auth_str.strip_prefix("Bearer ") {
                        match decode::<Claims>(
                            token,
                            &DecodingKey::from_secret(secret_key.as_ref()),
                            &Validation::new(Algorithm::HS256),
                        ) {
                            Ok(decoded_token) => {
                                println!("Token valid for user: {}", decoded_token.claims.sub);
                                return srv.call(req).await;
                            }
                            Err(_) => {
                                let response = HttpResponse::Unauthorized().finish();
                                return Ok(req.into_response(response));
                            }
                        }
                    }
                }
            }

            let response = HttpResponse::Unauthorized().finish();
            Ok(req.into_response(response))
        })
    }
}
