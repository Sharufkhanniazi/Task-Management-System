use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use bcrypt::{hash, verify};
use validator::Validate;


use crate::utils::jwt::create_jwt;
use crate::state::{AppState};
use crate::models::user::{RegisterUser, UserResponse, AuthResponse, LoginUser};
use crate::utils::error::AppError;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUser>
)-> Result<impl IntoResponse, AppError> {

    payload.validate()?;

    let existing_user = sqlx::query_scalar::<_,i64>(
        "SELECT COUNT(*) FROM users WHERE email = $1 OR username = $2"
    ).bind(&payload.email)
    .bind(&payload.username)
    .fetch_one(&state.db)
    .await?;

    if existing_user > 0 {
        return Err(AppError::UserAlreadyExists);
    }

    let password_hash = hash(&payload.password, state.hash_cost).unwrap();

    let user = sqlx::query_as::<_,UserResponse>(
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1,$2,$3)
        RETURNING id, email, username, created_at
        "#
    ).bind(&payload.email)
    .bind(&payload.username)
    .bind(&password_hash)
    .fetch_one(&state.db)
    .await?;

    let token = create_jwt(user.id, &user.email, &user.username, &state.jwt_secret)?;

    Ok(Json(AuthResponse {
        token,
        user,
    }))
}


pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginUser>
) -> Result<impl IntoResponse, AppError> {

    payload.validate()?;

    let user = sqlx::query_as::<_,(uuid::Uuid, String, String, String)>(
        "SELECT id, email, username, password_hash FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    let (id, email, username, password_hash) = user;
    if !verify(&payload.password, &password_hash).unwrap() {
        return Err(AppError::InvalidCredentials);
    }

    let token = create_jwt(id, &email, &username, &state.jwt_secret)?;

    let user_response = UserResponse {
        id,
        email,
        username,
        created_at: chrono::Utc::now(),
    };

    Ok(Json(AuthResponse {
        token,
        user: user_response,
    }))
}