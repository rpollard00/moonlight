use sqlx::FromRow;

#[derive(Clone, FromRow, Debug, serde::Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub family_id: i64,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub family_id: i64,
}

#[derive(Clone, FromRow, Debug, serde::Deserialize)]
pub struct Family {
    pub id: i64,
    pub name: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct NewFamily {
    pub name: String,
}
