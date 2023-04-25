use crate::template::{authenticated, public};

use axum::{routing::get, Router};

pub fn public_routes() -> Router {
    Router::new()
        .route("/landing", get(public::landing))
        .route("/demo", get(public::demo))
        .route("/signup", get(public::signup))
        .route("/login", get(public::login))
}

pub fn authenticated_routes() -> Router {
    Router::new()
        .route("/classes", get(authenticated::classes))
        .route("/lessons", get(authenticated::lessons))
        .route("/profile", get(authenticated::profile))
}