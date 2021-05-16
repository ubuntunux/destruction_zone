use nalgebra::Vector3;

use rust_engine_3d::renderer::camera::CameraObjectData;
use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::actor_controller::{ ControllerDataType, ActorController };
use crate::game_module::actor_manager::calc_floating_height;
use crate::game_module::base_actor::BaseActor;
use crate::game_module::height_map_data::HeightMapData;

pub struct PlayerActor {
    pub _id: u64,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _transform_object: *mut TransformObjectData,
    pub _controller: ActorController,
    pub _floating_height: f32,
}

impl PlayerActor {
    pub fn create_player_actor(id: u64, controller_type: ControllerDataType, render_object: &RcRefCell<RenderObjectData>) -> Box<PlayerActor> {
        let transform_object = (&mut render_object.borrow_mut()._transform_object as *mut TransformObjectData).clone();
        let floating_height = calc_floating_height(&render_object.borrow());
        Box::new(PlayerActor {
            _id: id,
            _render_object: render_object.clone(),
            _transform_object: transform_object,
            _controller: ActorController::create_actor_controller(controller_type),
            _floating_height: floating_height,
        })
    }
}

impl BaseActor for PlayerActor {
    fn initialize_actor(&mut self) {
    }

    fn is_player_actor(&self) -> bool {
        true
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

    fn update_actor(&mut self, _delta_time: f32, _height_map_data: &HeightMapData) {
    }
}

impl PlayerActor {
    pub fn update_player_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData, main_camera: &mut CameraObjectData) {
        let transform = unsafe { &mut *(self._transform_object as *mut TransformObjectData) };

        self._controller.update_controller(delta_time, transform, self._floating_height, height_map_data);

        const CAMERA_OFFSET_Y: f32 = 3.0;
        const CAMERA_OFFSET_Z: f32 = 8.0;
        let camera_pos = transform.get_position() + main_camera._transform_object.get_front() * CAMERA_OFFSET_Z + main_camera._transform_object.get_up() * CAMERA_OFFSET_Y;
        main_camera._transform_object.set_position(&camera_pos);
        main_camera._transform_object.rotation_pitch(self._controller.get_velocity_pitch());
        main_camera._transform_object.rotation_yaw(self._controller.get_velocity_yaw());

        let roll = self._controller.get_roll();
        let yaw = std::f32::consts::PI - roll * 0.5;
        transform.set_yaw(main_camera._transform_object.get_yaw() + yaw);
        transform.set_roll(roll);
        transform.set_position(self._controller.get_position());
    }
}