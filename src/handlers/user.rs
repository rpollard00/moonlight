use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    models::{NewUser, User},
    store::Store,
};

#[axum_macros::debug_handler]
pub async fn create_handler(
    State(store): State<Arc<Store>>,
    Json(payload): Json<NewUser>,
) -> StatusCode {
    if let Ok(_) = store.users.create(payload).await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn update_handler(
    State(store): State<Arc<Store>>,
    Json(payload): Json<User>,
) -> StatusCode {
    if let Ok(_) = store.users.update(payload).await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn delete_handler(State(store): State<Arc<Store>>, Path(id): Path<i64>) -> StatusCode {
    if let Ok(_) = store.users.delete(id).await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn search_handler(State(store): State<Arc<Store>>) -> String {
    let users = store.users.search("").await.unwrap();

    format!("{:?}", users)
}
