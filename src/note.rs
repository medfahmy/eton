use crate::errors::CustomError;
use axum::{
    extract::Path,
    http::StatusCode,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(query_all))
        .route("/", post(create))
        .route("/:id", get(query_by_id))
        .route("/:id", put(update))
        .route("/:id", delete(remove))
}

#[derive(Deserialize, Serialize, FromRow)]
struct Note {
    id: i32,
    title: String,
    content: String,
}

#[derive(Deserialize, Serialize, FromRow)]
struct NewNote {
    title: String,
    content: String,
}

async fn query_all(Extension(pool): Extension<PgPool>) -> (StatusCode, Json<Vec<Note>>) {
    let notes = query_as!(Note, "select * from notes")
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(notes))
}

async fn query_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Note>, CustomError> {
    let note = query_as!(Note, "select * from notes where id=$1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    Ok(Json(note))
}

async fn create(
    Json(note): Json<NewNote>,
    Extension(pool): Extension<PgPool>,
) -> Result<(StatusCode, Json<NewNote>), CustomError> {
    if note.title.is_empty() {
        return Err(CustomError::BadRequest);
    }

    query!(
        "insert into notes (title, content) values ($1, $2)",
        &note.title,
        &note.content
    )
    .execute(&pool)
    .await
    .map_err(|_| CustomError::InternalServerError)?;

    Ok((StatusCode::CREATED, Json(note)))
}

async fn update(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
    Json(note): Json<NewNote>,
) -> Result<(StatusCode, Json<NewNote>), CustomError> {
    let _note = query_as!(Note, "select * from notes where id=$1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    query!(
        "update notes set title=$1, content=$2 where id=$3",
        note.title,
        note.content,
        id
    )
    .execute(&pool)
    .await
    .map_err(|_| CustomError::NotFound)?;

    Ok((StatusCode::OK, Json(note)))
}

async fn remove(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<StatusCode, CustomError> {
    let _note = query_as!(Note, "select * from notes where id=$1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    query!("delete from notes where id=$1", id)
        .execute(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    Ok(StatusCode::OK)
}
