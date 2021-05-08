use std::collections::HashMap;

use crate::application::project_application::Application;
use crate::game_module::actors::{ ActorBase, PlayerActor };

pub struct ActorManager {
    pub _id_generator: u64,
    pub _player_actor: *const PlayerActor,
    pub _actors: HashMap<u64, Box<dyn ActorBase>>,
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
        let id = self.generate_id();
        let player_render_object = project_application.get_project_scene_manager().get_skeletal_render_object("Player").unwrap();
        self._actors.insert(id, PlayerActor::create_player_actor(id, player_render_object));
        self._player_actor = (self._actors.get(&id).unwrap().as_ref() as *const dyn ActorBase) as *const PlayerActor;
        let player_actor = unsafe { &mut *(self._player_actor as *mut PlayerActor) };
        player_actor.initialize_actor();
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