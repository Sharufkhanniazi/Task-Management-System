use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState{
    pub db: PgPool,
    pub jwt_secret: String,
    pub hash_cost: u32,
}