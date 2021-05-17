use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::actor_controller::{ ControllerDataType, ActorController };
use crate::game_module::actor_manager::calc_floating_height;
use crate::game_module::base_actor::BaseActor;
use crate::game_module::height_map_data::HeightMapData;
use crate::game_module::armor::{ArmorInstance, ArmorDataType};

pub struct AIActor {
    pub _id: u64,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _transform_object: *mut TransformObjectData,
    pub _controller: ActorController,
    pub _armor: ArmorInstance,
}

impl AIActor {
    pub fn create_ai_actor(id: u64, controller_type: ControllerDataType, armor_type: ArmorDataType, render_object: &RcRefCell<RenderObjectData>) -> Box<AIActor> {
        let transform_object = (&mut render_object.borrow_mut()._transform_object as *mut TransformObjectData).clone();
        let floating_height = calc_floating_height(&render_object.borrow());
        Box::new(AIActor {
            _id: id,
            _render_object: render_object.clone(),
            _transform_object: transform_object,
            _controller: ActorController::create_actor_controller(controller_type, floating_height),
            _armor: ArmorInstance::create_armor_instance(armor_type),
        })
    }
}

impl BaseActor for AIActor {
    fn initialize_actor(&mut self) {
        unimplemented!()
    }

    fn is_player_actor(&self) -> bool {
        false
    }

    fn get_armor(&self) -> &ArmorInstance {
        &self._armor
    }

    fn get_armor_mut(&mut self) -> &mut ArmorInstance {
        &mut self._armor
    }

    fn get_controller(&self) -> &ActorController {
        &self._controller
    }

    fn get_controller_mut(&mut self) -> &mut ActorController {
        &mut self._controller
    }

    fn get_transform(&self) -> &TransformObjectData {
        unsafe { &(*self._transform_object) }
    }

    fn get_transform_mut(&self) -> &mut TransformObjectData {
        unsafe { &mut *(self._transform_object as *mut TransformObjectData) }
    }

    fn update_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData) {
        let transform = unsafe { &mut *(self._transform_object as *mut TransformObjectData) };
        self._controller.update_controller(delta_time, transform, height_map_data);

        transform.rotation_pitch(self._controller.get_velocity_pitch());
        transform.rotation_yaw(self._controller.get_velocity_yaw());
        transform.set_roll(self._controller.get_roll());
        transform.set_position(self._controller.get_position());
    }
}

impl AIActor {
}