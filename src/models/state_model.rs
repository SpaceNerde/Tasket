// Application Model, defining the current state

use crate::{managers::project_manager::ProjectManager, models::project_model::Project};

struct AppModel {
    project_manager: ProjectManager,
    state: AppState
}

#[derive(Default)]
enum AppState {
    #[default]
    Running,
    Done,
}
