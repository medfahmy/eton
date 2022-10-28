use crate::errors::CustomError;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow, PgPool};

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

pub fn create_router() -> Router {
    Router::new()
        .route("/", get(query_notes).post(create_note))
        .route("/status", get(async || StatusCode::OK))
        .route(
            "/:id",
            get(query_note_by_id)
                .put(update_note_by_id)
                .delete(delete_note_by_id),
        )
}

async fn query_notes(Extension(pool): Extension<PgPool>) -> (StatusCode, Json<Vec<Note>>) {
    let notes = query_as!(Note, "select * from notes")
        .fetch_all(&pool)
        .await
        .unwrap();

    (StatusCode::OK, Json(notes))
}

async fn query_note_by_id(
    Extension(pool): Extension<PgPool>,
    Path(id): Path<i32>,
) -> Result<Json<Note>, CustomError> {
    let note = query_as!(Note, "select * from notes where id=$1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| CustomError::NotFound)?;

    Ok(Json(note))
}

async fn create_note(
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

async fn update_note_by_id(
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

async fn delete_note_by_id(
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
