use crate::models::User;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct Store {
    db: SqlitePool,

    pub users: UserStore,
}

impl Store {
    pub fn new(db: SqlitePool) -> Self {
        let users = UserStore::new(db.clone());

        Self { db, users }
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
