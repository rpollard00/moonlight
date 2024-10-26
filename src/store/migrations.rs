use sqlx::{migrate::MigrateDatabase, Pool, Sqlite};
use std::error::Error;

pub async fn init_db(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if !Sqlite::database_exists(&url)
        .await
        .unwrap_or_else(|_| false)
    {
        println!("creating db: {}", url);
        Sqlite::create_database(url).await?;
    } else {
        println!("db: {} already exists", url);
    }

    Ok(())
}

pub async fn run_migrations(db: &Pool<Sqlite>) -> Result<(), Box<dyn Error>> {
    println!("Execute run migrations...");
    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("Crate dir: {}", crate_dir);
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");
    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await?
        .run(db)
        .await?;

    println!("Ran migration results: {:?}", migration_results);

    Ok(())
}
