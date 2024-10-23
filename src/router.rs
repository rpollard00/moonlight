use axum::{routing::get, Router};

use crate::templates::{hello, root};

pub fn init_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/hello/:name", get(hello))
}
