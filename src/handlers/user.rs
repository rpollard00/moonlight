use std::sync::Arc;

use axum::extract::State;

use crate::store::Store;

pub async fn get_all(State(store): State<Arc<Store>>) -> String {
    let users = store.users.get_all().await.unwrap();

    format!("{:?}", users)
}
