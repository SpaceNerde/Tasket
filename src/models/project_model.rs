// Stores Tasks, and gives user a enviorment to work in

use crate::models::task_model::Task;

#[derive(Debug, Clone)]
pub(crate) struct Project {
    title: String,
    description: String,
    tasks: Vec<Task>
}

impl Project {
    pub fn new(title: &str) -> Self {
        Project { 
            title: title.to_string(), 
            description: String::new(), 
            tasks: vec![] 
        }
    }

    pub fn change_description(&mut self, description: &str) {
        self.description = description.to_string();
    }

    pub fn push_task(&mut self, task: Task) {
        self.tasks.push(task);
    }

    pub fn pop_task(&mut self) {
        self.tasks.pop();
    }

    pub fn insert_task(&mut self, task: Task, index: usize) {
        self.tasks.insert(index, task);
    }

    pub fn remove_task(&mut self, index: usize) {
        self.tasks.remove(index);
    }
}
