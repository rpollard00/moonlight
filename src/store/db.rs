use crate::models::{Family, NewFamily, NewUser, User};
use sqlx::{sqlite::SqliteQueryResult, SqlitePool};

#[derive(Clone)]
pub struct Store {
    // db: SqlitePool,
    pub users: UserStore,
    pub family: FamilyStore,
}

impl Store {
    pub fn new(db: SqlitePool) -> Self {
        let users = UserStore::new(db.clone());
        let family = FamilyStore::new(db.clone());

        Self {
            // db,
            users,
            family,
        }
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

    pub async fn create(&self, req: NewUser) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            "INSERT INTO users (username, first_name, last_name, email, family_id)
             VALUES (?, ?, ?, ?);",
        )
        .bind(req.username)
        .bind(req.first_name)
        .bind(req.last_name)
        .bind(req.email)
        .bind(req.family_id)
        .execute(&self.db)
        .await
    }

    pub async fn delete(&self, id: i64) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query("DELETE FROM users WHERE id = (?);")
            .bind(id)
            .execute(&self.db)
            .await
    }

    pub async fn get(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT id, username, first_name, last_name, email, family_id FROM users WHERE id = (?);",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await
    }

    pub async fn update(&self, req: User) -> Result<SqliteQueryResult, sqlx::Error> {
        sqlx::query(
            r#"UPDATE users 
                SET username = (?),
                SET first_name = (?),
                SET last_name = (?),
                SET email = (?),
                WHERE id = (?);"#,
        )
        .bind(req.username)
        .bind(req.first_name)
        .bind(req.last_name)
        .bind(req.email)
        .bind(req.id)
        .execute(&self.db)
        .await
    }

    pub async fn search(&self, query: &str) -> Result<Vec<User>, sqlx::Error> {
        if query.is_empty() {
            sqlx::query_as::<_, User>(
                "SELECT id, username, first_name, last_name, email, family_id FROM users LIMIT 100;",
            )
            .fetch_all(&self.db)
            .await
        } else {
            sqlx::query_as::<_, User>(
                r#"SELECT id, username, first_name, last_name, email, family_id  
                FROM users WHERE 
                LOWER(username) LIKE '%' || ? || '%' 
                OR LOWER(first_name) LIKE '%' || ? || '%' 
                OR LOWER(last_name) LIKE '%' || ? || '%' 
                OR LOWER(email) LIKE '%' || ? || '%' 
                LIMIT 100
                ORDER BY first_name, last_name;"#,
            )
            .bind(query.to_ascii_lowercase())
            .fetch_all(&self.db)
            .await
        }
    }
}

#[derive(Clone)]
pub struct FamilyStore {
    db: SqlitePool,
}

impl FamilyStore {
    pub fn new(db: SqlitePool) -> Self {
        Self { db }
    }

    pub async fn create(&self, req: NewFamily) -> Result<SqliteQueryResult, sqlx::Error> {
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
        sqlx::query("UPDATE family SET name = (?) WHERE id = (?);")
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
