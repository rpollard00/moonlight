use std::ascii::AsciiExt;

use crate::models::{Family, User};
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

#[derive(Clone)]
pub struct Store {
    db: SqlitePool,

    pub users: UserStore,
    pub family: FamilyStore,
}

impl Store {
    pub fn new(db: SqlitePool) -> Self {
        let users = UserStore::new(db.clone());
        let family = FamilyStore::new(db.clone());

        Self { db, users, family }
    }
}

#[derive(Clone)]
pub struct UserStore {
    db: SqlitePool,
}

impl UserStore {
    pub(crate) fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn create(&self, req: User) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (username, first_name, last_name, email)
             VALUES (?, ?, ?, ?);",
        )
        .bind(req.username)
        .bind(req.first_name)
        .bind(req.last_name)
        .bind(req.email)
        .execute(&self.db)
        .await
        .unwrap();

        Ok(())
    }

    pub async fn get_all(&self) -> Result<Vec<User>, sqlx::Error> {
        let users: Vec<User> =
            sqlx::query_as("SELECT id, username, first_name, last_name, email FROM users")
                .fetch_all(&self.db)
                .await
                .unwrap();

        Ok(users)
    }
    // id INTEGER PRIMARY KEY NOT NULL,
    // username TEXT NOT NULL,
    // first_name TEXT NOT NULL,
    // last_name TEXT NOT NULL,
    // email TEXT NOT NULL,
    // dob INTEGER NOT NULL,
}

#[derive(Clone)]
struct FamilyStore {
    db: SqlitePool,
}

impl FamilyStore {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn create(&self, req: Family) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query("INSERT INTO family (name) VALUES (?);")
            .bind(req.name)
            .execute(&self.db)
            .await
    }

    pub async fn delete(&self, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM family WHERE id = (?);")
            .bind(id)
            .execute(&self.db)
            .await
    }

    pub async fn get(&self, id: i64) -> Result<Option<Family>, sqlx::Error> {
        sqlx::query_as::<_, Family>("SELECT id, name FROM family WHERE id = (?);")
            .bind(id)
            .fetch_optional(&self.db)
            .await
    }

    pub async fn update(&self, req: Family) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query("UPDATE family SET name = (?) WHERE (?);")
            .bind(req.name)
            .bind(req.id)
            .execute(&self.db)
            .await
    }

    pub async fn search(&self, query: &str) -> Result<Vec<Family>, sqlx::Error> {
        if query.is_empty() {
            sqlx::query_as::<_, Family>("SELECT id, name FROM family LIMIT 100;")
                .fetch_all(&self.db)
                .await
        } else {
            sqlx::query_as::<_, Family>(
                "SELECT id, name FROM family WHERE LOWER(name) LIKE '%' || ? || '%' LIMIT 100;",
            )
            .bind(query.to_ascii_lowercase())
            .fetch_all(&self.db)
            .await
        }
    }
}
