use sqlx::SqlitePool;
use std::error::Error;

mod handlers;
mod models;
mod router;
mod store;
mod templates;

use router::init_router;
use std::sync::Arc;
use store::{init_db, run_migrations, Store};

const SERVER_ADDR: &str = "0.0.0.0:42069";
const DB_URL: &str = "sqlite://sqlite.db";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = init_db(&DB_URL).await;

    let db = SqlitePool::connect(&DB_URL).await.unwrap();
    let _ = run_migrations(&db).await?;

    let store = Arc::new(Store::new(db));

    let app = init_router(store);
    let listener = tokio::net::TcpListener::bind(SERVER_ADDR).await.unwrap();
    println!("Listening on: {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
