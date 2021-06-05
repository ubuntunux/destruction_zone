use nalgebra::{ Vector2, Vector3 };
use serde::{ Serialize, Deserialize };

use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell};

use crate::game_module::game_constants::GRAVITY;
use crate::game_module::height_map_data::HeightMapData;

// Declare
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ShipControllerDataType {
    ShipController,
    TankController,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShipControllerData {
    pub _controller_data_name: String,
    pub _controller_data_type: ShipControllerDataType,
    pub _max_ground_speed: f32,
    pub _forward_acceleration: f32,
    pub _side_acceleration: f32,
    pub _floating_acceleration: f32,
    pub _damping: f32,
    pub _side_step_roll: f32,
    pub _side_step_roll_speed: f32,
    pub _boost_acceleration: f32,
    pub _max_rotation_speed: f32,
    pub _rotation_acceleration: f32,
    pub _rotation_damping: f32,
}

impl Default for ShipControllerData {
    fn default() -> ShipControllerData {
        ShipControllerData {
            _controller_data_name: "".to_string(),
            _controller_data_type: ShipControllerDataType::ShipController,
            _max_ground_speed: 50.0,
            _forward_acceleration: 50.0,
            _side_acceleration: 50.0,
            _floating_acceleration: 30.0,
            _damping: 30.0,
            _side_step_roll: 0.3,
            _side_step_roll_speed: 2.0,
            _boost_acceleration: 1.5,
            _max_rotation_speed: 10.0,
            _rotation_acceleration: 0.5,
            _rotation_damping: 0.1,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ShipController {
    pub _controller_data: RcRefCell<ShipControllerData>,
    pub _prev_ground_velocity: Vector2<f32>,
    pub _ground_velocity: Vector2<f32>,
    pub _prev_floating_velocity: f32,
    pub _floating_velocity: f32,
    pub _floating_height: f32,
    pub _acceleration: Vector3<f32>,
    pub _rotation_velocity: Vector2<f32>,
    pub _rotation_acceleration: Vector2<f32>,
    pub _position: Vector3<f32>,
    pub _roll: f32,
    pub _boost: bool,
    pub _on_ground: bool,
}

// Implementation
pub fn create_controller_data(controller_type: ShipControllerDataType) -> RcRefCell<ShipControllerData> {
    let controller_data = match controller_type {
        ShipControllerDataType::ShipController => ShipControllerData {
            _controller_data_name: "LightShipController".to_string(),
            ..Default::default()
        },
        ShipControllerDataType::TankController => ShipControllerData {
            _controller_data_name: "LightTankController".to_string(),
            ..Default::default()
        },
    };
    newRcRefCell(controller_data)
}

impl ShipController {
    pub fn create_ship_controller(controller_data: &RcRefCell<ShipControllerData>, floating_height: f32) -> ShipController {
        ShipController {
            _controller_data: controller_data.clone(),
            _prev_ground_velocity: Vector2::zeros(),
            _ground_velocity: Vector2::zeros(),
            _prev_floating_velocity: 0.0,
            _floating_velocity: 0.0,
            _floating_height: floating_height,
            _acceleration: Vector3::zeros(),
            _rotation_acceleration: Vector2::zeros(),
            _rotation_velocity: Vector2::zeros(),
            _position: Vector3::zeros(),
            _roll: 0.0,
            _boost: false,
            _on_ground: false,
        }
    }

    pub fn boost_on(&mut self) { self._boost = true; }
    pub fn acceleration_forward(&mut self) { self._acceleration.z = 1.0; }
    pub fn acceleration_backward(&mut self) { self._acceleration.z = -1.0; }
    pub fn acceleration_left(&mut self) { self._acceleration.x = 1.0; }
    pub fn acceleration_right(&mut self) { self._acceleration.x = -1.0; }
    pub fn acceleration_up(&mut self) { self._acceleration.y = 1.0; }
    pub fn acceleration_down(&mut self) { self._acceleration.y = -1.0; }
    pub fn acceleration_pitch(&mut self, acceleration: f32) { self._rotation_acceleration.x = acceleration; }
    pub fn acceleration_yaw(&mut self, acceleration: f32) { self._rotation_acceleration.y = acceleration; }
    pub fn get_velocity_pitch(&self) -> f32 { self._rotation_velocity.x as f32 }
    pub fn get_velocity_yaw(&self) -> f32 { self._rotation_velocity.y as f32 }
    pub fn get_position(&self) -> &Vector3<f32> { &self._position }
    pub fn get_roll(&self) -> f32 { self._roll }

    pub fn update_controller(&mut self, delta_time: f32, transform: &TransformObjectData, height_map_data: &HeightMapData) {
        let mut goal_roll = 0.0;

        let controller_data = self._controller_data.borrow();
        let boost_acceleration = if self._boost { controller_data._boost_acceleration } else { 1.0 };

        if 0.0 != self._acceleration.x {
            let dir_side = Vector2::new(transform.get_left().x, transform.get_left().z).normalize();
            self._ground_velocity += dir_side * self._acceleration.x * controller_data._side_acceleration * boost_acceleration * delta_time;
            goal_roll = -controller_data._side_step_roll * self._acceleration.x;
        }

        if 0.0 != self._acceleration.y {
            self._floating_velocity += self._acceleration.y * controller_data._floating_acceleration * boost_acceleration * delta_time;
        }

        if 0.0 != self._acceleration.z {
            let dir_forward = Vector2::new(transform.get_front().x, transform.get_front().z).normalize();
            self._ground_velocity += dir_forward * self._acceleration.z * controller_data._forward_acceleration * boost_acceleration * delta_time;
        }

        // ground speed
        if 0.0 != self._ground_velocity.x || 0.0 != self._ground_velocity.y {
            let mut ground_speed = self._ground_velocity.norm();
            self._ground_velocity /= ground_speed;
            let damping = controller_data._damping * delta_time;
            ground_speed = 0.0f32.max(ground_speed - damping);
            ground_speed = controller_data._max_ground_speed.min(ground_speed);
            self._ground_velocity *= ground_speed;
        }

        // apply gravity
        if 0.0 == self._acceleration.y && false == self._on_ground {
            self._floating_velocity -= GRAVITY * delta_time;
        }

        // apply velocity
        let velocity = Vector3::new(self._ground_velocity.x, self._floating_velocity, self._ground_velocity.y);
        let mut position = transform.get_position().clone() + &velocity * delta_time;
        if position != *transform.get_position() || false == self._on_ground {
            self._on_ground = false;
            let floating_height = height_map_data.get_height(&position, 0) + self._floating_height;
            if position.y < floating_height {
                position.y = floating_height;
                self._floating_velocity = 0.0;
                self._on_ground = true;
            }
            self._position = position;
        }

        // rotation speed
        if 0.0 != self._rotation_acceleration.x || 0.0 != self._rotation_acceleration.y {
            self._rotation_velocity = &self._rotation_acceleration * controller_data._rotation_acceleration;
            let rotation_velocity = self._rotation_velocity.norm();
            if controller_data._max_rotation_speed < rotation_velocity {
                self._rotation_velocity = &self._rotation_velocity / rotation_velocity * controller_data._max_rotation_speed;
            }
        } else {
            self._rotation_velocity = Vector2::zeros();
        }

        // roll
        let mut roll = transform.get_roll();
        if goal_roll != roll {
            let roll_diff = goal_roll - roll;
            let sign = if 0.0 <= roll_diff { 1.0 } else { -1.0 };
            let roll_speed = controller_data._side_step_roll_speed * delta_time * sign;
            if roll_diff.abs() < roll_speed.abs() {
                roll = goal_roll;
            } else {
                roll += roll_speed * roll_diff.abs() / controller_data._side_step_roll;
            }
            self._roll = roll;
        }

        self._prev_ground_velocity.clone_from(&self._ground_velocity);
        self._prev_floating_velocity = self._floating_velocity;
        self._acceleration = Vector3::zeros();
        self._rotation_acceleration = Vector2::zeros();
        self._boost = false;
    }
}