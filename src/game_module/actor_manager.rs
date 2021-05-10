use std::collections::HashMap;

use crate::application::project_application::Application;
use crate::game_module::actor_controller::ControllerDataType;
use crate::game_module::base_actor::BaseActor;
use crate::game_module::player_actor::PlayerActor;
use crate::game_module::ai_actor::AIActor;

pub struct ActorManager {
    pub _id_generator: u64,
    pub _player_actor: *const PlayerActor,
    pub _actors: HashMap<u64, Box<dyn BaseActor>>,
}

impl ActorManager {
    pub fn create_actor_manager() -> Box<ActorManager> {
        Box::new(ActorManager {
            _id_generator: 0,
            _player_actor: std::ptr::null(),
            _actors: HashMap::new(),
        })
    }

    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }

    pub fn get_player_actor(&self) -> &PlayerActor {
        unsafe { &*self._player_actor }
    }

    pub fn get_player_actor_mut(&self) -> &mut PlayerActor {
        unsafe { &mut *(self._player_actor as *mut PlayerActor) }
    }

    pub fn initialize_actor_manager(&mut self, project_application: &Application) {
        // PLayer Actor
        {
            let id = self.generate_id();
            let player_render_object = project_application.get_project_scene_manager().get_skeletal_render_object("Player").unwrap();
            self._actors.insert(id, PlayerActor::create_player_actor(id, ControllerDataType::Default, player_render_object));
            self._player_actor = (self._actors.get(&id).unwrap().as_ref() as *const dyn BaseActor) as *const PlayerActor;
            let player_actor = unsafe { &mut *(self._player_actor as *mut PlayerActor) };
            player_actor.initialize_actor();
        }

        // PLayer Actor
        let actor_names = project_application.get_project_scene_manager()._skeletal_render_object_map.keys();
        for actor_name in actor_names {
            if actor_name.starts_with("Enemy") {
                let id = self.generate_id();
                let actor_render_object = project_application.get_project_scene_manager().get_skeletal_render_object(actor_name).unwrap();
                self._actors.insert(id, AIActor::create_ai_actor(id, ControllerDataType::Default, actor_render_object));
                let actor = (self._actors.get(&id).unwrap().as_ref() as *const dyn BaseActor) as *const AIActor;
                let actor = unsafe { &mut *(actor as *mut AIActor) };
                actor.initialize_actor();
            }
        }
    }

    pub fn update_actor_manager(&mut self, project_application: &Application, delta_time: f32) {
        let height_map_data = project_application.get_project_scene_manager().get_height_map_data();

        let mut main_camera = &mut project_application.get_project_scene_manager()._main_camera.borrow_mut();
        let player_actor = self.get_player_actor_mut();
        player_actor.update_player_actor(delta_time, height_map_data, &mut main_camera);

        for actor in self._actors.values_mut() {
            if false == actor.is_player_actor() {
                actor.update_actor(delta_time, height_map_data);
            }
        }
    }
}