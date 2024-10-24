use std::sync::Arc;

use axum::extract::State;
use axum::{routing::get, Router};

use crate::store::Store;
use crate::templates::{hello, root};

pub fn init_router(store: Arc<Store>) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello", get(hello))
        .route("/hello/:name", get(hello))
        .route("/users", get(user_handler))
        .with_state(store)
}

pub async fn user_handler(State(store): State<Arc<Store>>) -> String {
    let users = store.users.get_all().await.unwrap();

    format!("{:?}", users)
}
