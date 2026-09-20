mod application;
mod database;
mod infrastructure;
mod input;
mod interfaces;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  let database = tauri::async_runtime::block_on(database::setup_database());
  let repository = infrastructure::todo_repository::TodoRepository::new(database);
  let service = application::todo_service::TodoService::new(repository);

  tauri::Builder::default()
    .manage(service)
    .invoke_handler(tauri::generate_handler![
        interfaces::tauri::commands::get_todos,
        interfaces::tauri::commands::add_todo,
        interfaces::tauri::commands::complete_todo,
        interfaces::tauri::commands::edit_todo,
        interfaces::tauri::commands::delete_todo
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