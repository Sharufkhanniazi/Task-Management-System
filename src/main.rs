mod state;
mod models;
mod utils;
mod handlers;
mod middleware;

use tracing;
use anyhow;
use std::env;
use tracing_subscriber;
use axum::{
    Router,
    routing::{get, post, put, delete},
};
use dotenvy;
use sqlx::PgPool;
use crate::{handlers::auth::{register, login}, state::AppState};
use crate::handlers::tasks::{create_task, get_tasks, get_task, update_task, delete_task};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let db_url = env::var("DATABASE_URL").expect("Can't find database url");
    let db = PgPool::connect(&db_url).await?;

    let jwt_secret = env::var("JWT_SECRET").expect("Can't find jwt secret key");

    let hash_cost: u32 = env::var("HASH_COST")
        .expect("Can't find hash cost")
        .parse()
        .expect("HASH_COST must be a valid u32");

    let app_state = AppState{ db, jwt_secret, hash_cost };

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/tasks", post(create_task).get(get_tasks))
        .route("/tasks/{task_id}", get(get_task).put(update_task).delete(delete_task))
        .with_state(app_state);

    let addr = "0.0.0.0:3000";

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Listening on Server {}", addr);
    axum::serve(listener, app).await?;

    Ok(())
}
