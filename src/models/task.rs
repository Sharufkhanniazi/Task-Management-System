use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateTask {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    
    pub status: Option<TaskStatus>,

    pub priority: Option<TaskPriority>,
    
    pub due_date: Option<DateTime<Utc>>,
}


#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateTask {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    
    #[validate(length(max = 1000))]
    pub description: Option<String>,
    
    pub status: Option<TaskStatus>,

    pub priority: Option<TaskPriority>,
    
    pub due_date: Option<DateTime<Utc>>,
}



#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Clone, Copy)]
#[sqlx(type_name = "varchar")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Archived,
}

impl Default for TaskStatus {
    fn default() -> Self {
        TaskStatus::Pending
    }
}

impl Validate for TaskStatus {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq, Clone, Copy)]
#[sqlx(type_name = "varchar")]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Urgent,
}

impl Default for TaskPriority {
    fn default() -> Self {
        TaskPriority::Medium
    }
}

impl Validate for TaskPriority {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        Ok(())
    }
}

#[derive(Debug, Serialize)]
pub struct TaskResponse {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub due_date: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Task> for TaskResponse {
    fn from(task: Task) -> Self {
        TaskResponse {
            id: task.id,
            title: task.title,
            description: task.description,
            status: task.status,
            priority: task.priority,
            due_date: task.due_date,
            user_id: task.user_id,
            created_at: task.created_at,
            updated_at: task.updated_at,
        }
    }
}