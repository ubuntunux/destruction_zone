use std::rc::Rc;
use nalgebra::Vector3;

use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;
use rust_engine_3d::renderer::render_object::{RenderObjectData};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor_data::ActorData;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_controller::{ GameController };
use crate::game_module::game_constants::{CHECK_TARGET_DISTANCE_MAX};
use crate::game_module::ship::ship::{ShipInstance, ShipData};
use crate::game_module::ship::ship_controller::{ ShipController };

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActorControllerState {
    None,
    Attack,
    Move,
    Patrol,
    Trace,
}

// ActorController
pub struct ActorController {
    pub _id: u64,
    pub _actor_data: ActorData,
    pub _ship: ShipInstance,
    pub _actor_controller_state: ActorControllerState,
    pub _target_position: Vector3<f32>,
    pub _is_player_actor: bool,
    pub _command_move: bool,
    pub _command_rotate: bool,
    pub _command_attack: bool
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
            _actor_controller_state: ActorControllerState::None,
            _target_position: Vector3::zeros(),
            _is_player_actor: is_player_actor,
            _command_move: false,
            _command_rotate: false,
            _command_attack: false,
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
    pub fn can_manual_controll(&self) -> bool {
        false == self._command_move && false == self._command_rotate
    }

    pub fn manual_actor_attack(&mut self, game_client: &GameClient) {
        let project_scene_manager = game_client.get_project_scene_manager();
        let main_camera = project_scene_manager.get_main_camera();
        let fire_start = main_camera.get_camera_position();
        let fire_dir = -main_camera.get_camera_front() as Vector3<f32>;
        let mut target_position: Vector3<f32> = fire_start + &fire_dir * CHECK_TARGET_DISTANCE_MAX;
        project_scene_manager.get_height_map_collision_point(fire_start, &fire_dir, CHECK_TARGET_DISTANCE_MAX, &mut target_position);

        self._ship.ship_fire(game_client, &fire_start, &fire_dir, &target_position);
    }

    pub fn set_command_actor_attack(&mut self, target_position: &Vector3<f32>) {
        self.cancle_command_of_actor();
        self._target_position.clone_from(target_position);
        self._command_attack = true;
        self._command_rotate = true;
        self._actor_controller_state = ActorControllerState::Attack;
    }

    pub fn set_command_actor_move(&mut self, target_position: &Vector3<f32>) {
        self.cancle_command_of_actor();
        self._target_position.clone_from(target_position);
        self._command_move = true;
        self._command_rotate = true;
        self._actor_controller_state = ActorControllerState::Move;
    }
    pub fn cancle_command_of_actor(&mut self) {
        self._command_attack = false;
        self._command_move = false;
        self._command_rotate = false;
        self._actor_controller_state = ActorControllerState::None;
    }

    fn roate_to_target(ship_controller: &mut ShipController, to_target: &Vector3<f32>, actor_left: &Vector3<f32>, actor_front: &Vector3<f32>, delta_time: f32) -> bool {
        let front_dot_target = actor_front.dot(&to_target);
        let velocity_yaw = ship_controller.get_velocity_yaw();
        let yaw_delta = velocity_yaw * delta_time;
        let yaw_diff = (0.5 - front_dot_target * 0.5) * std::f32::consts::PI;
        if yaw_delta < yaw_diff {
            let braking_time = velocity_yaw / ship_controller._controller_data.borrow()._rotation_damping;
            let braking_distance = velocity_yaw * 0.5 * braking_time;
            if braking_distance < yaw_diff {
                let accel_yaw = if 0.0 <= actor_left.dot(&to_target) { 1.0 } else { -1.0 };
                ship_controller.acceleration_yaw(accel_yaw);
            }
        } else {
            let goal_yaw: f32 = to_target.x.atan2(to_target.z);
            ship_controller.set_yaw(goal_yaw);
            ship_controller.set_velocity_yaw(0.0);
            return true;
        }
        false
    }

    fn move_to_target(ship_controller: &mut ShipController, target_position: &Vector3<f32>, to_target: &Vector3<f32>, distance: f32, actor_front: &Vector3<f32>, delta_time: f32) -> bool {
        let velocity = ship_controller.get_velocity();
        let ground_speed = (velocity.x * velocity.x + velocity.z * velocity.z).sqrt();
        let move_delta = ground_speed * delta_time;
        if move_delta < distance {
            let braking_time = ground_speed / ship_controller._controller_data.borrow()._damping;
            let braking_distance = ground_speed * 0.5 * braking_time;
            let front_dot_target = actor_front.dot(&to_target);
            if braking_distance < distance && 0.0 < front_dot_target {
                ship_controller.acceleration_forward();
            }
        } else {
            ship_controller.set_velocity(&Vector3::zeros());
            let mut position = ship_controller.get_position().clone_owned();
            position.x = target_position.x;
            position.z = target_position.z;
            ship_controller.set_position(&position);
            return true;
        }
        false
    }

    pub fn update_command_actor_move(&mut self, delta_time: f32) {
        if self._command_move || self._command_rotate {
            let ship_controller = ptr_as_mut(&self.get_ship()._controller);
            let mut to_target = &self._target_position - ship_controller.get_position();
            to_target.y = 0.0;
            let distance = to_target.norm();
            if 0.0 < distance {
                to_target /= distance;
            } else {
                self.cancle_command_of_actor();
                return;
            }

            let mut front = self.get_ship().get_transform().get_front().clone_owned();
            front.y = 0.0;
            front.normalize_mut();

            let mut left = self.get_ship().get_transform().get_left().clone_owned();
            left.y = 0.0;
            left.normalize_mut();

            if self._command_rotate {
                if ActorController::roate_to_target(ship_controller, &to_target, &left, &front, delta_time) {
                    self._command_rotate = false;
                }
            }

            if self._command_move && false == self._command_rotate {
                if ActorController::move_to_target(ship_controller, &self._target_position, &to_target, distance, &front, delta_time) {
                    self._command_rotate = false;
                }
            }
        }
    }

    pub fn update_command_actor_attack(&mut self, delta_time: f32, game_controller: &GameController) {
        if self._command_attack || self._command_rotate {
            let ship_controller = ptr_as_mut(&self.get_ship()._controller);

            if self._command_rotate {
                let mut to_target = &self._target_position - ship_controller.get_position();
                to_target.y = 0.0;
                let distance = to_target.norm();
                if 0.0 < distance {
                    to_target /= distance;

                    let mut front = self.get_ship().get_transform().get_front().clone_owned();
                    front.y = 0.0;
                    front.normalize_mut();

                    let mut left = self.get_ship().get_transform().get_left().clone_owned();
                    left.y = 0.0;
                    left.normalize_mut();
                    if ActorController::roate_to_target(ship_controller, &to_target, &left, &front, delta_time) {
                        self._command_rotate = false;
                    }
                } else {
                    self._command_rotate = false;
                }
            }

            if self._command_attack && false == self._command_rotate {
                // fire
                let game_client = game_controller.get_game_client();
                let fire_start = self.get_transform().get_position().clone_owned();
                let fire_dir = (&self._target_position - ship_controller.get_position()).normalize();
                let target_position: Vector3<f32> = &fire_start + &fire_dir * CHECK_TARGET_DISTANCE_MAX;
                self._ship.ship_fire(game_client, &fire_start, &fire_dir, &target_position);

                // stop
                self.cancle_command_of_actor();
            }
        }
    }

    pub fn update_actor_controller(&mut self, delta_time: f32, project_scene_manager: &ProjectSceneManager, game_controller: &GameController) {
        if self._is_player_actor {
            if ActorControllerState::Move == self._actor_controller_state {
                self.update_command_actor_move(delta_time);
            } else if ActorControllerState::Attack == self._actor_controller_state {
                self.update_command_actor_attack(delta_time, game_controller);
            }
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
