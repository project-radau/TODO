pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool
}

impl Todo {
    pub fn print(&self) {
        println!("ID: {0}", self.id);
        println!("Title: {0}", self.title);
        println!("Completed: {0}", self.completed);
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