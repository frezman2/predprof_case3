use sqlx::{PgPool, postgres::PgPoolOptions};

pub async fn create_pool() -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/admission")
        .await
        .expect("Не удалось подключиться к БД")
}
