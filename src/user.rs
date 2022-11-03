use crate::errors::CustomError;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};
use tracing::info;

pub fn create_router() -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

#[derive(Deserialize, Serialize, FromRow)]
struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize, FromRow)]
struct SignUpData {
    username: String,
    email: String,
    password: String,
    confirm_password: String,
}

#[derive(Deserialize, Serialize, FromRow)]
struct LoginInput {
    username: String,
    password: String,
}

async fn register() {
    info!("register")
}

async fn login() {
    info!("login")
}

async fn logout() {
    info!("logout")
}

async fn refresh() {
    info!("refresh")
}
