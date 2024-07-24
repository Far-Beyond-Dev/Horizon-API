use actix_service::Service;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpMessage};
use jsonwebtoken::{decode, DecodingKey, Validation, errors::ErrorKind};
use std::env;
use futures::future::Either;
use serde::{Deserialize, Serialize};

// Define the Claims structure according to your JWT payload
#[derive(Debug, Deserialize, Serialize)]
struct Claims {
    sub: String, // Subject (typically user ID)
    exp: usize,  // Expiration time (as a Unix timestamp)
    // Add more fields if needed
}

pub async fn auth_middleware(
    req: ServiceRequest,
    srv: &actix_service::Service
) -> Result<ServiceResponse, Error> {
    // Extract Authorization header
    if let Some(header) = req.headers().get("Authorization") {
        if let Ok(header_str) = header.to_str() {
            if let Some(token) = header_str.strip_prefix("Bearer ") {
                let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

                // Decode the JWT token
                let token_data = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_ref()),
                    &Validation::default(),
                );

                match token_data {
                    Ok(_) => return Ok(srv.call(req).await?),
                    Err(e) => {
                        // Handle decoding errors
                        match e.kind() {
                            ErrorKind::InvalidToken | ErrorKind::InvalidIssuer | ErrorKind::ExpiredSignature => {
                                return Err(actix_web::error::ErrorUnauthorized("Invalid token"))
                            },
                            _ => {
                                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
                            },
                        }
                    }
                }
            }
        }
    }

    // No Authorization header or invalid token
    Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
}
