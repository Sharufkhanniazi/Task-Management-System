use axum::{
    extract::{State, Path, Query},
    response::IntoResponse, 
    Json,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use crate::{middleware::auth::AuthenticatedUser, models::task::{CreateTask, Task, TaskPriority, TaskStatus, UpdateTask, TaskResponse}, state::AppState, utils::error::AppError};
use sqlx::QueryBuilder;
use sqlx::postgres::Postgres;

pub async fn create_task(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Json(payload): Json<CreateTask>,
) -> Result<impl IntoResponse, AppError>{
    
    payload.validate()?;

    let task = sqlx::query_as::<_,Task>(
        r#"
        INSERT INTO tasks (title, description, status, priority, due_date, user_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#
    ).bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.status.unwrap_or_default())
    .bind(&payload.priority.unwrap_or_default())
    .bind(&payload.due_date)
    .bind(user.id)
    .fetch_one(&state.db) 
    .await?;

    Ok(Json(task))
}

#[derive(Debug, Deserialize)]
pub struct Pagination{
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<TaskStatus>,
    pub priority: Option<TaskPriority>,
}

pub async fn get_tasks(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Query(pagination): Query<Pagination>
) -> Result<impl IntoResponse, AppError> {
    let page = pagination.page.unwrap_or(1);
    let per_page = pagination.per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    let mut query_str = "SELECT * FROM tasks WHERE user_id = $1".to_string();
    let mut query = sqlx::query_as::<_, Task>(&query_str)
        .bind(user.id);

    if let Some(status) = pagination.status {
        query_str.push_str(" AND status = $2");
        query = sqlx::query_as::<_, Task>(&query_str)
            .bind(user.id)
            .bind(status); 
    }

    if let Some(priority) = pagination.priority {
        let param_index = if pagination.status.is_some() { 3 } else { 2 };
        query_str.push_str(&format!(" AND priority = ${}", param_index));
        query = sqlx::query_as::<_, Task>(&query_str)
            .bind(user.id);

        if let Some(status) = pagination.status {
            query = query.bind(status);
        }

        query = query.bind(priority); 
    }

    let param_index = if pagination.status.is_some() && pagination.priority.is_some() {
        4
    } else if pagination.status.is_some() || pagination.priority.is_some() {
        3
    } else {
        2
    };

    query_str.push_str(&format!(" ORDER BY created_at DESC LIMIT ${} OFFSET ${}", param_index, param_index + 1));

    query = sqlx::query_as::<_, Task>(&query_str)
        .bind(user.id);

    if let Some(status) = pagination.status {
        query = query.bind(status);
    }
    if let Some(priority) = pagination.priority {
        query = query.bind(priority);
    }

    query = query
        .bind(per_page as i64)
        .bind(offset as i64);

    let tasks: Vec<Task> = query
        .fetch_all(&state.db)
        .await?;

    let response: Vec<TaskResponse> = tasks.into_iter().map(TaskResponse::from).collect();

    Ok(Json(response))
}


pub async fn get_task(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(task_id): Path<Uuid>
) -> Result<impl IntoResponse, AppError> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT * FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(user.id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::TaskNotFound)?;

    Ok(Json(TaskResponse::from(task)))
}

pub async fn update_task(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(task_id): Path<Uuid>,
    Json(payload): Json<UpdateTask>,
) -> Result<impl IntoResponse, AppError> {
    payload.validate()?;

    let existing_task = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(user.id)
    .fetch_one(&state.db)
    .await?;

    if existing_task == 0 {
        return Err(AppError::TaskNotFound);
    }

    let mut builder: QueryBuilder<Postgres> = QueryBuilder::new("UPDATE tasks SET ");
    let mut has_update = false;

    if let Some(title) = &payload.title {
        if has_update { builder.push(", "); }
        builder.push("title = ").push_bind(title);
        has_update = true;
    }

    if let Some(description) = &payload.description {
        if has_update { builder.push(", "); }
        builder.push("description = ").push_bind(description);
        has_update = true;
    }

    if let Some(status) = &payload.status {
        if has_update { builder.push(", "); }
        builder.push("status = ").push_bind(status);
        has_update = true;
    }

    if let Some(priority) = &payload.priority {
        if has_update { builder.push(", "); }
        builder.push("priority = ").push_bind(priority);
        has_update = true;
    }

    if let Some(due_date) = &payload.due_date {
        if has_update { builder.push(", "); }
        builder.push("due_date = ").push_bind(due_date);
        has_update = true;
    }

    if !has_update {
        return Err(AppError::ValidationError(validator::ValidationErrors::new()));
    }

    builder.push(" WHERE id = ").push_bind(task_id);
    builder.push(" AND user_id = ").push_bind(user.id);
    builder.push(" RETURNING *");

    let query = builder.build_query_as::<Task>();
    let task: Task = query.fetch_one(&state.db).await?;

    Ok(Json(TaskResponse::from(task)))
}

pub async fn delete_task(
    State(state): State<AppState>,
    user: AuthenticatedUser,
    Path(task_id): Path<Uuid>,
) -> Result<impl IntoResponse, AppError> {

    let result = sqlx::query(
        "DELETE FROM tasks WHERE id = $1 AND user_id = $2"
    )
    .bind(task_id)
    .bind(user.id)
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::TaskNotFound);
    }

    Ok(Json(json!({
        "message": "Task deleted sucessfully"
    })))
}