// Manger for Projects, loading, editing, deleting, etc.

use crate::{managers::database_manager::DataBaseManager, models::project_model::Project};

pub(crate) struct ProjectManager {
    current_project: Project,
    database_manager: DataBaseManager
}
