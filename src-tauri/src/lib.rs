use tauri::Manager;

mod application;
mod database;
mod infrastructure;
mod input;
mod interfaces;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let database = tauri::async_runtime::block_on(
                database::setup_database(app.handle())
            );

            let repository =
                infrastructure::todo_repository::TodoRepository::new(database);

            let service =
                application::todo_service::TodoService::new(repository);

            app.manage(service);

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            interfaces::tauri::commands::get_todos,
            interfaces::tauri::commands::add_todo,
            interfaces::tauri::commands::complete_todo,
            interfaces::tauri::commands::edit_todo,
            interfaces::tauri::commands::delete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}