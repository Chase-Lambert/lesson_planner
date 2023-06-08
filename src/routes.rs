// use crate::handlers::{authenticated, lessons, public};
use crate::handlers::{authenticated, lessons, lessons2, public};
use axum::{
    routing::{get, post},
    Router,
};

pub fn public_routes() -> Router {
    Router::new()
        .route("/landing", get(public::landing))
        .route("/demo", get(public::demo))
        .route("/signup", get(public::signup))
        .route("/login", get(public::login))
}

pub fn authenticated_routes() -> Router {
    Router::new()
        .route("/account", get(authenticated::account))
        .route("/classes", get(authenticated::classes))
        .route("/lesson", get(lessons::lesson_builder))
        .route("/lesson_viewer", post(lessons::lesson_viewer))
        .route("/lesson_builder2", get(lessons2::lesson_builder2))
        .route("/profile", get(authenticated::profile))
}
