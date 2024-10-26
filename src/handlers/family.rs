use std::{collections::HashMap, sync::Arc};

use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;

use crate::models::NewFamily;
use crate::{models::Family, store::Store};

// pub use family::{create_handler, delete_handler, update_handler};
// family [post]
pub async fn create_handler(
    State(store): State<Arc<Store>>,
    Json(payload): Json<NewFamily>,
) -> StatusCode {
    if let Ok(_) = store.family.create(payload).await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn update_handler(
    State(store): State<Arc<Store>>,
    Json(payload): Json<Family>,
) -> StatusCode {
    if let Ok(_) = store.family.update(payload).await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn delete_handler(State(store): State<Arc<Store>>, Path(id): Path<i64>) -> StatusCode {
    if let Ok(_) = store.family.delete(id).await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn search_handler(
    State(store): State<Arc<Store>>,
    Query(query): Query<HashMap<String, String>>,
) -> String {
    let default_string = String::from("");
    let search_string = query.get("query").unwrap_or(&default_string);
    let family = store.family.search(&search_string).await.unwrap();

    format!("{:?}", family)
}
