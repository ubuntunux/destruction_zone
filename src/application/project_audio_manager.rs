use rust_engine_3d::utilities::system::{ self, newRcRefCell, RcRefCell };

use crate::application::project_application::ProjectApplication;
use crate::resource::project_resource::ProjectResources;

pub struct AudioDataCreateInfo {
    pub _audio_name: String,
    pub _audio_source: bool,
}

#[derive(Clone)]
pub struct AudioInstance {
    pub _audio_data: RcRefCell<AudioDataCreateInfo>,
}

pub struct ProjectAudioManager {
    pub _project_application: *const ProjectApplication,
    pub _project_resources: *const ProjectResources,
    pub _audios: Vec<RcRefCell<AudioInstance>>,
    pub _bgm: Option<Box<AudioInstance>>,
}

impl AudioInstance {
    pub fn create_audio(audio_data: &RcRefCell<AudioDataCreateInfo>) -> RcRefCell<AudioInstance> {
        newRcRefCell(AudioInstance {
            _audio_data: audio_data.clone(),
        })
    }
}

impl ProjectAudioManager {
    pub fn create_audio_manager() -> Box<ProjectAudioManager> {
        Box::new(ProjectAudioManager {
            _project_application: std::ptr::null(),
            _project_resources: std::ptr::null(),
            _audios: Vec::new(),
            _bgm: None,
        })
    }

    pub fn initialize_audio_manager(&mut self, project_application: *const ProjectApplication, project_resources: *const ProjectResources) {
        self._project_application = project_application;
        self._project_resources = project_resources;
        self.create_audio("game_load");
    }

    pub fn get_project_application(&self) -> &ProjectApplication {
        unsafe { &*self._project_application }
    }

    pub fn get_project_resources(&self) -> &ProjectResources {
        unsafe { &*self._project_resources }
    }

    pub fn create_audio(&mut self, audio_name: &str) -> RcRefCell<AudioInstance> {
        let audio_data = self.get_project_resources().get_audio_data(audio_name);
        let audio_instance = AudioInstance::create_audio(&audio_data);
        self._audios.push(audio_instance.clone());

        audio_instance
    }

    pub fn update_audio_manager(&mut self) {

    }
}
