// backend/src/auth/middleware.rs
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    error::ErrorUnauthorized,
    http::header,
    web::Data,
    Error, HttpMessage,
};
use futures::future::{ready, LocalBoxFuture, Ready};
use std::{
    future::Future,
    pin::Pin,
    rc::Rc,
    sync::Arc,
    task::{Context, Poll},
};
use crate::{
    auth::verify_jwt,
    config::Config,
};

// Auth middleware
pub struct JwtAuth;

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtAuthMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        
        // Get the config from the app data
        let config_opt = req.app_data::<Data<Arc<Config>>>();
        
        if config_opt.is_none() {
            return Box::pin(async move {
                Err(ErrorUnauthorized("Server configuration error"))
            });
        }
        
        let config = config_opt.unwrap();
        
        // Extract the token from the Authorization header
        let auth_header = req
            .headers()
            .get(header::AUTHORIZATION)
            .map(|h| h.to_str().unwrap_or_default().to_string());
        
        // Check if token exists and has the correct format
        let token = match auth_header {
            Some(header) if header.starts_with("Bearer ") => header[7..].to_string(),
            _ => {
                return Box::pin(async move {
                    Err(ErrorUnauthorized("Invalid authorization header"))
                });
            }
        };
        
        // Verify the token
        match verify_jwt(&token, &config.auth.jwt_secret) {
            Ok(claims) => {
                // Add the claims to the request extensions
                req.extensions_mut().insert(claims);
                
                let fut = service.call(req);
                Box::pin(async move {
                    let res = fut.await?;
                    Ok(res)
                })
            }
            Err(_) => Box::pin(async move {
                Err(ErrorUnauthorized("Invalid or expired token"))
            }),
        }
    }
}

// Function to extract user_id from request's extensions
pub fn extract_user_id(req: &ServiceRequest) -> Option<String> {
    req.extensions()
        .get::<crate::auth::Claims>()
        .map(|claims| claims.sub.clone())
}
