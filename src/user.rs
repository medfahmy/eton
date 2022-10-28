use axum::{
    routing::{delete, get, post, put},
    Router,
};
use tracing::info;

struct User {
    id: i32,
    username: String,
    email: String,
    password: String,
}

pub fn create_router() -> Router {
    Router::new().route("/", get(query_users).post(create_user))
}

async fn query_users() {
    info!("query users")
}

async fn create_user() {
    info!("create user")
}

async fn update_user() {
    info!("update user")
}

async fn delete_user() {
    info!("update user")
}
