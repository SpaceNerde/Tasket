use crate::models::{project_model::Project, task_model::{self, Task}};

pub(crate) mod models;
pub(crate) mod managers;

pub(crate) mod message;
pub(crate) mod ui;
pub(crate) mod renderer;

// Trying to follow the ELM model, dont know lets see what will happen :P

fn main() {
    let mut task = Task::new("Test Task");

    task.change_status();
    task.change_content("This is a test Task");

    let mut project = Project::new("Test Project");

    project.change_description("This is a test Project");
    project.push_task(task);

    println!("{:?}", project);
}
