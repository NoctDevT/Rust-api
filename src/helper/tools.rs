use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::env;
use crate::models::claims::Claims;

pub fn validate_jwt(token: &str) -> Result<String, String> {
    let secret = env::var("SECRET_KEY").map_err(|_| "SECRET_KEY not set".to_string())?;

    let decoding_key = DecodingKey::from_secret(secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);

    match decode::<Claims>(token, &decoding_key, &validation) {
        Ok(token_data) => Ok(token_data.claims.sub),
        Err(err) => Err(format!("JWT validation failed: {:?}", err)),
    }
}