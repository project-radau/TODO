use crate::application::todo::Todo;
use crate::application::todo_service::TodoService;
use crate::infrastructure::todo_repository::TodoRepository;

#[tauri::command]
pub async fn get_todos(service: tauri::State<'_, TodoService<TodoRepository>>,) -> Result<Vec<Todo>, String> {
    service
        .get_all()
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn add_todo(title: String, service: tauri::State<'_, TodoService<TodoRepository>>) -> Result<i64, String> {
    service
        .add_todo(&title)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn complete_todo(id: i64, service: tauri::State<'_, TodoService<TodoRepository>>) -> Result<(), String> {
    service
        .complete_todo(id)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn edit_todo(id: i64, title: String, service: tauri::State<'_, TodoService<TodoRepository>>) -> Result<(), String> {
    service
        .edit_todo(id, &title)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn delete_todo(id: i64, service: tauri::State<'_, TodoService<TodoRepository>>) -> Result<(), String> {
    service
        .remove_todo(id)
        .await
        .map(|_| ())
        .map_err(|error| error.to_string())
}