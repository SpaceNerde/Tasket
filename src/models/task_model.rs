// Holds the info for each task, like name, description and more

#[derive(Debug, Clone)]
pub(crate) struct Task {
    title: String,
    content: String,
    status: bool,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Task {
            title: title.to_string(),
            content: String::new(),
            status: false,
        }
    }

    pub fn change_status(&mut self) {
        self.status = !self.status;
    }

    pub fn change_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
}
