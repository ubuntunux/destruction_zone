use crate::renderer::project_ui::ProjectUIManager;

pub struct GameUIManager {
    pub _project_ui_manager: *const ProjectUIManager,
}

impl GameUIManager {
    pub fn create_game_ui_manager() -> Box<GameUIManager> {
        Box::new(GameUIManager {
            _project_ui_manager: std::ptr::null(),
        })
    }

    pub fn get_project_ui_manager(&self) -> &ProjectUIManager {
        unsafe { &*self._project_ui_manager }
    }

    pub fn get_project_ui_manager_mut(&self) -> &mut ProjectUIManager {
        unsafe { &mut *(self._project_ui_manager as *mut ProjectUIManager) }
    }

    pub fn initialize_game_ui_manager(&mut self, project_ui_manager: *const ProjectUIManager) {
        self._project_ui_manager = project_ui_manager;


    }

    pub fn update_game_ui(&mut self) {

    }
}