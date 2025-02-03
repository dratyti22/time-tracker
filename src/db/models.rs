use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Task {
    pub id: i64,
    pub name: String,
    pub start_time: String,
    pub end_time: Option<String>,
}
