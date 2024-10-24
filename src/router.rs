use std::sync::Arc;

use axum::{routing::get, Router};

use crate::handlers::get_all;
use crate::store::Store;
use crate::templates::{hello, root};

pub fn router(store: Arc<Store>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/hello/:name", get(hello))
        .route("/users", get(get_all))
        .with_state(store)
}
