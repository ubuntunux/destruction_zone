use std::collections::HashMap;
use std::mem::MaybeUninit;

use crate::application::project_application::Application;
use crate::game_module::actors::{ ActorBase, PlayerActor };
use crate::game_module::game_client::GameClient;

pub struct ActorManager {
    pub _id_generator: u64,
    pub _player_actor: Box<PlayerActor>,
    pub _actors: HashMap<u64, Box<dyn ActorBase>>,
}

impl ActorManager {
    pub fn create_actor_manager() -> Box<ActorManager> {
        Box::new(ActorManager {
            _id_generator: 0,
            _player_actor: unsafe { Box::new(MaybeUninit::uninit().assume_init()) },
            _actors: HashMap::new(),
        })
    }

    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }

    pub fn initialize_actor_manager(&mut self, project_application: &Application) {
        let id = self.generate_id();
        //self._player_actor.initialize_actor(id, );
    }

    pub fn update_actor_manager(&mut self) {

    }
}