use std::fmt::{Display};

#[derive(Debug, serde::Serialize)]
pub struct Todo {
    id: i64,
    title: String,
    completed: bool
}

impl Display for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ID: {}\nTitle: {}\nComplete: {}",
            self.id,
            self.title,
            self.completed
        )
    }
}

impl Todo {
    pub fn new(id: i64, title: &str) -> Self {
        Self {
            id,
            title: String::from(title),
            completed: false,
        }
    }

    pub fn from_database(id: i64, title: String, completed: bool) -> Self {
        Self {
            id,
            title,
            completed,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn title(&self) -> &str {
        return &self.title;
    }

    pub fn completed(&self) -> bool {
        self.completed
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = String::from(title);
    }

    pub fn complete(&mut self) {
        if self.completed == false {
            self.completed = true;
        }
    }

    pub fn is_completed(&self) -> bool {
        return self.completed == true;
    }
}