use rust_engine_3d::utilities::system::RcRefCell;
use rust_engine_3d::renderer::render_object::RenderObjectData;

use crate::game_module::actor_controller::ActorController;

pub trait ActorBase {
    fn initialize_actor(&mut self);
    fn update_actor(&mut self);
}

pub struct PlayerActor {
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _controller: ActorController,
}

impl PlayerActor {
    pub fn create_player_actor(render_object: &RcRefCell<RenderObjectData>) -> Box<PlayerActor> {
        Box::new(PlayerActor {
            _render_object: render_object.clone(),
            _controller: ActorController::create_actor_controller(),
        })
    }
}

impl ActorBase for PlayerActor {
    fn initialize_actor(&mut self) {

    }

    fn update_actor(&mut self) {
        self._controller.update_controller();
    }
}