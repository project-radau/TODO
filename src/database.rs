use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

pub async fn setup_database() -> SqlitePool {
    let database = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename("todos.db")
                .create_if_missing(true)
        )
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Failed to run database migrations");

    database
}