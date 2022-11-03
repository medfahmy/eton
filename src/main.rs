use axum::{Extension, Router, Server};
use std::net::SocketAddr;
use tower_http::trace;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod database;
mod errors;
mod note;
mod user;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let pool = database::create_pool("postgresql://postgres:postgres@localhost:5432/eton")
        .await
        .expect("database failed");

    let user_router = user::create_router();
    let note_router = note::create_router();

    let router = Router::new()
        .nest("/users", user_router)
        .nest("/notes", note_router)
        .layer(Extension(pool))
        .layer(
            trace::TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    let addr: SocketAddr = "127.0.0.1:8080"
        .parse()
        .expect("parsing server address failed");

    info!("listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");
}
