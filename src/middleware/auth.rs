use axum::extract::{FromRequestParts, FromRef};
use http::request::Parts;
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use uuid::Uuid;

use crate::{
    state::AppState,
    utils::{error::AppError, jwt::decode_jwt},
};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub id: Uuid,
    pub email: String,
    pub username: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::Unauthorized)?;

        let claims = decode_jwt(bearer.token(), &app_state.jwt_secret)
            .map_err(|_| AppError::Unauthorized)?;

        Ok(Self {
            id: claims.sub,
            email: claims.email,
            username: claims.username,
        })
    }
}
