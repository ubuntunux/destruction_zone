use rust_engine_3d::renderer::camera::CameraObjectData;
use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::actor_controller::actor_controller::{ ControllerDataType, ActorController };
use crate::game_module::actors::actor_data::{ ActorData, ActorTrait };
use crate::game_module::height_map_data::HeightMapData;
use crate::game_module::armor::armor::{ArmorInstance, ArmorDataType};

pub struct PlayerActor {
    pub _id: u64,
    pub _actor_data: ActorData,
}

impl ActorTrait for PlayerActor {
    fn initialize_actor(&mut self) {
    }

    fn get_actor_id(&self) -> u64 {
        self._id
    }

    fn is_player_actor(&self) -> bool {
        true
    }

    fn get_actor_data(&self) -> &ActorData {
        &self._actor_data
    }

    fn get_actor_data_mut(&mut self) -> &mut ActorData {
        &mut self._actor_data
    }

    fn get_armor(&self) -> &ArmorInstance {
        &self._actor_data._armor
    }

    fn get_armor_mut(&mut self) -> &mut ArmorInstance {
        &mut self._actor_data._armor
    }

    fn get_controller(&self) -> &ActorController {
        &self._actor_data._controller
    }

    fn get_controller_mut(&mut self) -> &mut ActorController {
        &mut self._actor_data._controller
    }

    fn get_transform(&self) -> &TransformObjectData {
        self._actor_data.get_transform()
    }

    fn get_transform_mut(&self) -> &mut TransformObjectData {
        self._actor_data.get_transform_mut()
    }

    fn update_actor(&mut self, _delta_time: f32, _height_map_data: &HeightMapData) {
        unimplemented!()
    }

    fn update_player_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData, main_camera: &mut CameraObjectData) {
        let transform = unsafe { &mut *(self._actor_data._transform_object as *mut TransformObjectData) };

        // update actor controller
        self._actor_data._controller.update_controller(delta_time, transform, height_map_data);

        // update camera transform
        let actor_controller = &self._actor_data._controller;
        main_camera._transform_object.rotation_pitch(actor_controller.get_velocity_pitch() * delta_time);
        main_camera._transform_object.rotation_yaw(actor_controller.get_velocity_yaw() * delta_time);
        main_camera._transform_object.update_transform_object();

        // set camera offset
        const CAMERA_OFFSET_Y: f32 = 3.0;
        const CAMERA_OFFSET_Z: f32 = 8.0;
        let camera_pos = actor_controller.get_position() + main_camera._transform_object.get_front() * CAMERA_OFFSET_Z + main_camera._transform_object.get_up() * CAMERA_OFFSET_Y;
        main_camera._transform_object.set_position(&camera_pos);

        // update player transform
        let roll = actor_controller.get_roll();
        let yaw = std::f32::consts::PI - roll * 0.5;
        transform.set_yaw(main_camera._transform_object.get_yaw() + yaw);
        transform.set_roll(roll);
        transform.set_position(actor_controller.get_position());
    }
}

impl PlayerActor {
    pub fn create_player_actor(
        id: u64,
        controller_type: ControllerDataType,
        armor_type: ArmorDataType,
        render_object: &RcRefCell<RenderObjectData>
    ) -> Box<PlayerActor> {
        Box::new(PlayerActor {
            _id: id,
            _actor_data: ActorData::create_actor_data(controller_type, armor_type, render_object),
        })
    }
}