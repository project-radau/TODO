mod application;
mod database;
mod infrastructure;
mod input;

use crate::application::todo_service::TodoService;
use crate::infrastructure::todo_repository::TodoRepository;

#[tauri::command]
async fn get_todos(service: tauri::State<'_, TodoService<TodoRepository>>,) -> Result<Vec<application::todo::Todo>, String> {
    service
        .get_all()
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn add_todo(title: String, service: tauri::State<'_, TodoService<TodoRepository>>) -> Result<i64, String> {
    service
        .add_todo(&title)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn complete_todo(id: i64, service: tauri::State<'_, TodoService<TodoRepository>>) -> Result<(), String> {
    service
        .complete_todo(id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn edit_todo(id: i64, title: String, service: tauri::State<'_, TodoService<TodoRepository>>) -> Result<(), String> {
    service
        .edit_todo(id, &title)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
async fn delete_todo(id: i64, service: tauri::State<'_, TodoService<TodoRepository>>) -> Result<(), String> {
    service
        .remove_todo(id)
        .await
        .map(|_| ())
        .map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let database = tauri::async_runtime::block_on(database::setup_database());

  let repository = infrastructure::todo_repository::TodoRepository::new(database);

  let service = TodoService::new(repository);

  tauri::Builder::default()
    .manage(service)
    .invoke_handler(tauri::generate_handler![
        get_todos,
        add_todo,
        complete_todo,
        edit_todo,
        delete_todo
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}