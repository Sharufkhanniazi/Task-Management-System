use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::error::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims{
    pub sub: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub email: String,
    pub username: String,
}

pub fn create_jwt(user_id: Uuid, email: &str, username: &str, jwt_secret: &str)
-> Result<String, AppError> {
    let now = Utc::now();
    let exp = now + Duration::hours(24);

    let claims = Claims{
        sub: user_id,
        exp: exp.timestamp() as usize,
        iat: now.timestamp() as usize,
        email: email.to_string(),
        username: username.to_string(),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )?;

    Ok(token)
}

pub fn decode_jwt(token: &str, jwt_secret: &str)-> Result<Claims, AppError> {
    let token_data = decode::<Claims>(
        token, 
        &DecodingKey::from_secret(jwt_secret.as_bytes()), 
        &Validation::default()
    )?;

    Ok(token_data.claims)
}