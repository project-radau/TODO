use tauri::{AppHandle, Manager};
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

pub async fn setup_database(app: &AppHandle) -> SqlitePool {

    let app_data_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");

    std::fs::create_dir_all(&app_data_dir)
        .expect("Failed to create app data directory");

    let database_path = app_data_dir.join("todos.db");

    let database = SqlitePoolOptions::new()
        .connect_with(
            SqliteConnectOptions::new()
                .filename(database_path)
                .create_if_missing(true),
        )
        .await
        .expect("Failed to connect to database");

    sqlx::migrate!("./migrations")
        .run(&database)
        .await
        .expect("Failed to run database migrations");

    database
}