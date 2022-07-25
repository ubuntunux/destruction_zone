use std::rc::Rc;
use nalgebra::Vector3;

use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;
use rust_engine_3d::renderer::render_object::{RenderObjectData};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor_data::ActorData;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_controller::{ GameViewMode, GameController };
use crate::game_module::game_constants::{CHECK_TARGET_DISTANCE_MAX};
use crate::game_module::ship::ship::{ShipInstance, ShipData};
use crate::game_module::ship::ship_controller::{ ShipController };

// ActorController
pub struct ActorController {
    pub _id: u64,
    pub _actor_data: ActorData,
    pub _ship: ShipInstance,
    pub _target_position: Vector3<f32>,
    pub _command_move_to_target: bool,
    pub _command_rotate_to_target: bool,
    pub _is_player_actor: bool,
}

impl ActorController {
    pub fn create_actor_controller(
        id: u64,
        ship_data: &RcRefCell<ShipData>,
        render_object: &RcRefCell<RenderObjectData>,
        is_player_actor: bool
    ) -> Rc<ActorController> {
        Rc::new(ActorController {
            _id: id,
            _actor_data: ActorData {},
            _ship: ShipInstance::create_ship_instance(ship_data, render_object),
            _target_position: Vector3::zeros(),
            _command_move_to_target: false,
            _command_rotate_to_target: false,
            _is_player_actor: is_player_actor,
        })
    }

    pub fn initialize_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.initialize_ship_instance(self, project_scene_manager);
    }
    pub fn remove_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.remove_ship_instance(project_scene_manager);
    }
    pub fn get_actor_id(&self) -> u64 {
        self._id
    }
    pub fn is_player_actor(&self) -> bool {
        self._is_player_actor
    }
    pub fn get_actor_data(&self) -> &ActorData {
        &self._actor_data
    }
    pub fn get_actor_data_mut(&mut self) -> &mut ActorData {
        &mut self._actor_data
    }
    pub fn get_ship(&self) -> &ShipInstance {
        &self._ship
    }
    pub fn get_ship_mut(&mut self) -> &mut ShipInstance {
        &mut self._ship
    }
    pub fn get_controller(&self) -> &ShipController {
        &self._ship._controller
    }
    pub fn get_controller_mut(&mut self) -> &mut ShipController {
        &mut self._ship._controller
    }
    pub fn get_transform(&self) -> &TransformObjectData { self._ship.get_transform() }
    pub fn get_transform_mut(&self) -> &mut TransformObjectData {
        self._ship.get_transform_mut()
    }
    pub fn get_velocity(&self) -> &Vector3<f32> { self.get_controller().get_velocity() }
    pub fn actor_fire(&mut self, game_client: &GameClient, game_view_mode: &GameViewMode) {
        let project_scene_manager = game_client.get_project_scene_manager();
        let mut fire_start: Vector3<f32> = Vector3::zeros();
        let mut fire_dir: Vector3<f32> = Vector3::zeros();
        match game_view_mode {
            GameViewMode::FpsViewMode => {
                let main_camera = project_scene_manager.get_main_camera();
                fire_start.clone_from(main_camera.get_camera_position());
                fire_dir = -main_camera.get_camera_front() as Vector3<f32>;
            },
            GameViewMode::TopViewMode => {
                fire_start.clone_from(self.get_transform().get_position());
                fire_dir = -self.get_transform().get_front() as Vector3<f32>;
            },
            _ => assert!(false, "Not implemented."),
        };
        let mut target_position: Vector3<f32> = &fire_start + &fire_dir * CHECK_TARGET_DISTANCE_MAX;
        project_scene_manager.get_height_map_collision_point(&fire_start, &fire_dir, CHECK_TARGET_DISTANCE_MAX, &mut target_position);

        self._ship.ship_fire(game_client, &fire_start, &fire_dir, &target_position);
    }
    pub fn actor_move(&mut self, target_position: &Vector3<f32>) {
        self._target_position.clone_from(target_position);
        self._command_move_to_target = true;
        self._command_rotate_to_target = true;
    }
    pub fn cancle_actor_move(&mut self) {
        self._command_move_to_target = false;
        self._command_rotate_to_target = false;
    }
    pub fn update_actor_move(&mut self, delta_time: f32, _project_scene_manager: &ProjectSceneManager, _game_controller: &GameController) {
        if self._command_move_to_target || self._command_rotate_to_target {
            let ship_controller = ptr_as_mut(&self.get_ship()._controller);
            let mut to_target = &self._target_position - ship_controller.get_position();
            to_target.y = 0.0;
            let distance = to_target.norm();
            if 0.0 < distance {
                to_target /= distance;
            } else {
                self._command_move_to_target = true;
                self._command_rotate_to_target = true;
                return;
            }

            let mut front = self.get_ship().get_transform().get_front().clone_owned();
            front.y = 0.0;
            front.normalize_mut();

            let front_dot_target = front.dot(&to_target);

            // rotate to target
            if self._command_rotate_to_target {
                let velocity_yaw = ship_controller.get_velocity_yaw();
                let yaw_delta = velocity_yaw * delta_time;
                let yaw_diff = (0.5 - front_dot_target * 0.5) * std::f32::consts::PI;
                if yaw_delta < yaw_diff {
                    let braking_time = velocity_yaw / ship_controller._controller_data.borrow()._rotation_damping;
                    let braking_distance = velocity_yaw * 0.5 * braking_time;
                    if braking_distance < yaw_diff {
                        let mut left = self.get_ship().get_transform().get_left().clone_owned();
                        left.y = 0.0;
                        left.normalize_mut();
                        let accel_yaw = if 0.0 <= left.dot(&to_target) { 1.0 } else { -1.0 };
                        ship_controller.acceleration_yaw(accel_yaw);
                    }
                } else {
                    let goal_yaw: f32 = to_target.x.atan2(to_target.z);
                    ship_controller.set_yaw(goal_yaw);
                    ship_controller.set_velocity_yaw(0.0);
                    self._command_rotate_to_target = false;
                }
            }

            // move to target
            if self._command_move_to_target && false == self._command_rotate_to_target {
                let velocity = ship_controller.get_velocity();
                let ground_speed = (velocity.x * velocity.x + velocity.z * velocity.z).sqrt();
                let move_delta = ground_speed * delta_time;
                if move_delta < distance {
                    let braking_time = ground_speed / ship_controller._controller_data.borrow()._damping;
                    let braking_distance = ground_speed * 0.5 * braking_time;
                    if braking_distance < distance && 0.0 < front_dot_target {
                        ship_controller.acceleration_forward();
                    }
                } else {
                    ship_controller.set_velocity(&Vector3::zeros());
                    let mut position = ship_controller.get_position().clone_owned();
                    position.x = self._target_position.x;
                    position.z = self._target_position.z;
                    ship_controller.set_position(&position);
                    self._command_move_to_target = false;
                }
            }
        }
    }

    pub fn update_actor_controller(&mut self, delta_time: f32, project_scene_manager: &ProjectSceneManager, game_controller: &GameController) {
        if self._is_player_actor {
            self.update_actor_move(delta_time, project_scene_manager, game_controller);
        }

        let transform = ptr_as_mut(self.get_ship()._transform_object);
        let ship_controller = ptr_as_mut(&self.get_ship()._controller);

        ship_controller.update_controller(delta_time, transform, project_scene_manager);

        transform.set_rotation(ship_controller.get_rotation());
        transform.set_position(ship_controller.get_position());
        transform.update_matrix();

        // update ship
        self.get_ship_mut().update_ship(delta_time);
    }
}
