use rust_engine_3d::renderer::camera::CameraObjectData;
use rust_engine_3d::renderer::render_object::{RenderObjectData};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::application::project_application::ProjectApplication;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor_data::{ ActorData, ActorTrait };
use crate::game_module::height_map_data::HeightMapData;
use crate::game_module::ship::ship::{ShipInstance, ShipData};
use crate::game_module::ship::ship_controller::{ ShipController };
use nalgebra::Vector3;
use crate::game_module::game_constants::{BULLET_DISTANCE_MAX, BULLET_CHECK_STEP};

pub struct PlayerActor {
    pub _id: u64,
    pub _actor_data: ActorData,
    pub _ship: ShipInstance,
    pub _target_position: Vector3<f32>,
}

impl ActorTrait for PlayerActor {
    fn initialize_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.initialize_ship_instance(self, project_scene_manager);
    }
    fn remove_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.remove_ship_instance(project_scene_manager);
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
    fn get_ship(&self) -> &ShipInstance {
        &self._ship
    }
    fn get_ship_mut(&mut self) -> &mut ShipInstance {
        &mut self._ship
    }
    fn get_controller(&self) -> &ShipController {
        &self._ship._controller
    }
    fn get_controller_mut(&mut self) -> &mut ShipController {
        &mut self._ship._controller
    }
    fn get_transform(&self) -> &TransformObjectData {
        self._ship.get_transform()
    }
    fn get_transform_mut(&self) -> &mut TransformObjectData {
        self._ship.get_transform_mut()
    }
    fn fire(&mut self, project_application: &ProjectApplication) {
        let height_map_data = project_application.get_project_scene_manager().get_height_map_data();
        let main_camera = &project_application.get_project_scene_manager().get_main_camera().borrow();
        let camera_position: &Vector3<f32> = main_camera.get_camera_position();
        let camera_dir = main_camera._transform_object.get_front();
        let loop_count: usize = (BULLET_DISTANCE_MAX / BULLET_CHECK_STEP).ceil() as usize;
        for i in 0..loop_count {
            let check_dist = BULLET_CHECK_STEP * i as f32;
            self._target_position = camera_position - camera_dir * check_dist;
            let floating_height = height_map_data.get_height(&self._target_position, 0);
            if self._target_position.y < floating_height {
                break;
            }
        }
        self._ship.fire(project_application, &self._target_position);
    }
    fn update_actor(&mut self, _delta_time: f32, _height_map_data: &HeightMapData) {
        unimplemented!()
    }
}

impl PlayerActor {
    pub fn create_player_actor(
        id: u64,
        ship_data: &RcRefCell<ShipData>,
        render_object: &RcRefCell<RenderObjectData>
    ) -> Box<PlayerActor> {
        Box::new(PlayerActor {
            _id: id,
            _actor_data: ActorData {},
            _ship: ShipInstance::create_ship_instance(ship_data, render_object),
            _target_position: Vector3::zeros(),
        })
    }

    pub fn update_player_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData, main_camera: &mut CameraObjectData, camera_distance: f32) {
        let transform = unsafe { &mut *(self._ship._transform_object as *mut TransformObjectData) };

        // update actor controller
        self._ship._controller.update_controller(delta_time, transform, height_map_data);

        // update camera transform
        let ship_controller = &self._ship._controller;
        main_camera._transform_object.rotation_pitch(ship_controller.get_velocity_pitch() * delta_time);
        main_camera._transform_object.rotation_yaw(ship_controller.get_velocity_yaw() * delta_time);
        main_camera._transform_object.update_transform_object();

        // set camera offset
        let mut cockpit_offset = main_camera._transform_object.get_front().clone();
        {
            cockpit_offset.y = 0.0;
            cockpit_offset.normalize_mut();
            let bound_box = &self._ship._render_object.borrow()._bound_box;
            const BOUND_BOX_MIN: f32 = 2.0;
            cockpit_offset = cockpit_offset * -BOUND_BOX_MIN.max(bound_box._size.z * 0.5);
            cockpit_offset.y = BOUND_BOX_MIN.max(bound_box._size.y * 0.5);
        }

        let mut camera_pos = ship_controller.get_position() + main_camera._transform_object.get_front() * camera_distance + cockpit_offset;
        let floating_height = height_map_data.get_height(&camera_pos, 0) + 1.0;
        if camera_pos.y < floating_height {
            camera_pos.y = floating_height;
        }
        main_camera._transform_object.set_position(&camera_pos);

        // update player transform
        let roll = ship_controller.get_roll();
        let yaw = std::f32::consts::PI - roll * 0.5;
        transform.set_yaw(main_camera._transform_object.get_yaw() + yaw);
        transform.set_roll(roll);
        transform.set_position(ship_controller.get_position());
        transform.update_matrix();

        // update weapon
        self.get_ship_mut().update_ship(delta_time, height_map_data);
    }
}