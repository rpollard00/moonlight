use std::sync::Arc;

use axum::routing::{delete, patch, post};
use axum::{routing::get, Router};

use crate::handlers;
use crate::store::Store;
use crate::templates::{family_body, hello, root};

pub fn router(store: Arc<Store>) -> Router {
    Router::new()
        // HTML PAGE HANDLERS
        .route("/", get(root))
        .route("/add_family", get(family_body))
        .route("/hello/:name", get(hello))
        // JSON HANDLERS
        //   users
        .route("/api/users", post(handlers::user_create_handler))
        .route("/api/users", patch(handlers::user_update_handler))
        .route("/api/users/:id", delete(handlers::user_delete_handler))
        //   family
        .route("/api/family", post(handlers::family_create_handler))
        .route("/api/family", patch(handlers::family_update_handler))
        .route("/api/family/:id", delete(handlers::family_delete_handler))
        .with_state(store)
}
