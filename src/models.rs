use sqlx::FromRow;

#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub family_id: i64,
}

#[derive(Clone, FromRow, Debug)]
pub struct Family {
    pub id: i64,
    pub name: String,
}
