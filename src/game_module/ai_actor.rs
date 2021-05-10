use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::actor_controller::{ ControllerDataType, ActorController };
use crate::game_module::actor_manager::calc_floating_height;
use crate::game_module::base_actor::BaseActor;
use crate::game_module::height_map_data::HeightMapData;

pub struct AIActor {
    pub _id: u64,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _transform_object: *mut TransformObjectData,
    pub _controller: ActorController,
    pub _floating_height: f32,
}

impl AIActor {
    pub fn create_ai_actor(id: u64, controller_type: ControllerDataType, render_object: &RcRefCell<RenderObjectData>) -> Box<AIActor> {
        let transform_object = (&mut render_object.borrow_mut()._transform_object as *mut TransformObjectData).clone();
        let floating_height = calc_floating_height(&render_object.borrow());
        Box::new(AIActor {
            _id: id,
            _render_object: render_object.clone(),
            _transform_object: transform_object,
            _controller: ActorController::create_actor_controller(controller_type),
            _floating_height: floating_height,
        })
    }
}

impl BaseActor for AIActor {
    fn initialize_actor(&mut self) {
    }

    fn is_player_actor(&self) -> bool {
        false
    }

    fn get_transform(&self) -> &TransformObjectData {
        unsafe { &(*self._transform_object) }
    }

    fn get_transform_mut(&self) -> &mut TransformObjectData {
        unsafe { &mut *(self._transform_object as *mut TransformObjectData) }
    }

    fn update_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData) {
        let transform = unsafe { &mut *(self._transform_object as *mut TransformObjectData) };
        self._controller.update_controller(delta_time, transform, self._floating_height, height_map_data);
    }
}

impl AIActor {
}