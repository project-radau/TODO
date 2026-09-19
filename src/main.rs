#![allow(dead_code)]

mod application;
mod infrastructure;
mod cli;
mod database;
mod input;

use crate::infrastructure::todo_repository::TodoRepository;
use crate::application::todo_service::TodoService;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database = database::setup_database().await;
    let repository = TodoRepository::new(database);
    let service = TodoService::new(repository);

    cli::run(service).await;
}