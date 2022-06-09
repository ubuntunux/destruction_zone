use std::rc::Rc;
use nalgebra::Vector3;

use rust_engine_3d::renderer::render_object::{RenderObjectData};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;
use crate::application::project_application::ProjectApplication;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor_data::{ ActorData, ActorTrait };
use crate::game_module::game_controller::{ GameViewMode, GameController };
use crate::game_module::height_map_data::HeightMapData;
use crate::game_module::ship::ship::{ShipInstance, ShipData};
use crate::game_module::ship::ship_controller::{ ShipController };
use crate::game_module::game_constants::{
    BULLET_DISTANCE_MAX,
    BULLET_CHECK_STEP,
};


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
    fn get_transform(&self) -> &TransformObjectData { self._ship.get_transform() }
    fn get_transform_mut(&self) -> &mut TransformObjectData {
        self._ship.get_transform_mut()
    }
    fn get_velocity(&self) -> &Vector3<f32> { self.get_controller().get_velocity() }
    fn actor_fire(&mut self, project_application: &ProjectApplication, fire_dir: &Vector3<f32>) {
        let height_map_data = project_application.get_project_scene_manager().get_height_map_data();
        let transform = unsafe { &*(self._ship._transform_object) };
        self._target_position.clone_from(transform.get_position());
        let loop_count: usize = (BULLET_DISTANCE_MAX / BULLET_CHECK_STEP).ceil() as usize;
        for i in 0..loop_count {
            let check_dist = BULLET_CHECK_STEP * i as f32;
            self._target_position += fire_dir * check_dist;
            let floating_height = height_map_data.get_height(&self._target_position, 0);
            if self._target_position.y < floating_height {
                break;
            }
        }
        self._ship.ship_fire(project_application, &self._target_position);
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
    ) -> Rc<PlayerActor> {
        Rc::new(PlayerActor {
            _id: id,
            _actor_data: ActorData {},
            _ship: ShipInstance::create_ship_instance(ship_data, render_object),
            _target_position: Vector3::zeros(),
        })
    }

    pub fn update_player_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData, game_controller: &GameController) {
        let transform = unsafe { &mut *(self._ship._transform_object as *mut TransformObjectData) };

        self._ship._controller.update_controller(delta_time, transform, height_map_data);

        // update player transform
        let ship_controller = &self._ship._controller;
        if GameViewMode::TopViewMode == game_controller._game_view_mode {
            transform.rotation_pitch(ship_controller.get_velocity_pitch() * delta_time);
            transform.rotation_yaw(ship_controller.get_velocity_yaw() * delta_time);
        } else if GameViewMode::FpsViewMode == game_controller._game_view_mode {
            // apply roll weight to pitch
            let roll_weight: f32 = 0.0;
            let yaw = ship_controller.get_rotation().y - ship_controller.get_rotation().z * roll_weight;
            transform.set_yaw(yaw);
        } else {
            assert!(false, "Not implemented.");
        }
        transform.set_roll(ship_controller.get_rotation().z);
        transform.set_position(ship_controller.get_position());
        transform.update_matrix();

        // update ship
        self.get_ship_mut().update_ship(delta_time, height_map_data);
    }
}