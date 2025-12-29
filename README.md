# Task Management API (Rust + Axum)

A RESTful Task Management API built with **Rust**, **Axum**, **SQLx**, and **PostgreSQL**.  
It supports **JWT-based authentication**, **user-scoped tasks**, **pagination**, and **role-safe updates**.

---

## 🚀 Features

- User Registration & Login
- JWT Authentication (Bearer Token)
- CRUD Operations for Tasks
- Task Pagination & Filtering
- Secure Password Hashing (bcrypt)
- Input Validation
- SQLx Compile-time Safe Queries
- Clean Project Structure

---

## 🧱 Tech Stack

- **Rust**
- **Axum 0.8**
- **SQLx (PostgreSQL)**
- **JWT (jsonwebtoken)**
- **bcrypt**
- **validator**
- **dotenvy**
- **uuid**
- **chrono**

---

## 📂 Project Structure

```text
src/
├── handlers/
│   ├── auth.rs
│   └── tasks.rs
├── middleware/
│   └── auth.rs
├── models/
│   ├── task.rs
│   └── user.rs
├── utils/
│   ├── error.rs
│   └── jwt.rs
├── state.rs
└── main.rs

migrations/
└── initial.sql

--------------------------------------------
Authentication Flow
Register
POST /register
Content-Type: application/json

{
  "email": "user@example.com",
  "username": "user123",
  "password": "password123"
}

Response:

{
  "token": "jwt_token_here",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "username": "user123",
    "created_at": "2025-12-15T10:00:00Z"
  }
}
--------------------------------------------
Login
POST /login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "password123"
}

Response:

{
  "token": "jwt_token_here",
  "user": { ... }
}

Use token for authenticated routes:

Authorization: Bearer <token>

--------------------------------------------
Create Task
POST /tasks
Authorization: Bearer <token>
Content-Type: application/json

{
  "title": "Finish project",
  "description": "Complete Rust API",
  "priority": "High",
  "status": "Pending",
  "due_date": "2025-12-20T12:00:00Z"
}

--------------------------------------------
Get Tasks (with Pagination & Filters)
GET /tasks?page=1&per_page=10&status=Pending&priority=High
Authorization: Bearer <token>

Get Single Task
GET /tasks/{task_id}
Authorization: Bearer <token>

--------------------------------------------
Update Task
PUT /tasks/{task_id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "status": "Completed",
  "priority": "Medium"
}

--------------------------------------------
Delete Task
DELETE /tasks/{task_id}
Authorization: Bearer <token>

Response:

{
  "message": "Task deleted successfully"
}

❌ Error Handling

All endpoints return standardized JSON error responses:

{
  "error": "Validation failed",
  "message": "Invalid input data"
}


HTTP Status Codes:

400 Validation Error

401 Unauthorized

403 Forbidden

404 Not Found

409 Conflict

500 Internal Server Error

🧪 Development
Run Server
cargo run


Server runs on:

http://localhost:3000

--------------------------------------------
SQLx Offline Support (Recommended)

cargo install sqlx-cli
cargo sqlx prepare

