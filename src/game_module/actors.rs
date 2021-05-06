use rust_engine_3d::utilities::system::RcRefCell;
use rust_engine_3d::renderer::render_object::RenderObjectData;

use crate::game_module::actor_controller::ActorController;

pub trait ActorBase {
    fn initialize_actor(&mut self, id: u64, render_object: &RcRefCell<RenderObjectData>);
    fn update_actor(&mut self);
}

pub struct PlayerActor {
    pub _id: u64,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _controller: ActorController,
}

impl PlayerActor {
}

impl ActorBase for PlayerActor {
    fn initialize_actor(&mut self, id: u64, render_object: &RcRefCell<RenderObjectData>) {
        println!("{}, ", id);
        self._id = id;
        self._render_object = render_object.clone();
        self._controller = ActorController::create_actor_controller();
    }

    fn update_actor(&mut self) {
        self._controller.update_controller();
    }
}