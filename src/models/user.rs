use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
