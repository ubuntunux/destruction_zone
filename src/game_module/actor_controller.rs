use nalgebra::{ Vector2, Vector3 };

use rust_engine_3d::renderer::transform_object::TransformObjectData;

use crate::game_module::height_map_data::HeightMapData;

pub struct ActorController {
    pub _prev_ground_velocity: Vector2<f32>,
    pub _ground_velocity: Vector2<f32>,
    pub _prev_floating_velocity: f32,
    pub _floating_velocity: f32,
    pub _acceleration: Vector3<f32>,
}

impl ActorController {
    pub fn create_actor_controller() -> ActorController {
        ActorController {
            _prev_ground_velocity: Vector2::zeros(),
            _ground_velocity: Vector2::zeros(),
            _prev_floating_velocity: 0.0,
            _floating_velocity: 0.0,
            _acceleration: Vector3::zeros(),
        }
    }

    pub fn acceleration_forward(&mut self) { self._acceleration.z = 1.0; }
    pub fn acceleration_backward(&mut self) { self._acceleration.z = -1.0; }
    pub fn acceleration_left(&mut self) {
        self._acceleration.x = 1.0;
    }
    pub fn acceleration_right(&mut self) {
        self._acceleration.x = -1.0;
    }
    pub fn acceleration_up(&mut self) {
        self._acceleration.y = 1.0;
    }
    pub fn acceleration_down(&mut self) {
        self._acceleration.y = -1.0;
    }

    pub fn update_controller(&mut self, delta_time: f32, transform: &mut TransformObjectData, floating_height: f32, height_map_data: &HeightMapData) {
        const MAX_GROUND_SPEED: f32 = 50.0;
        const FORWARD_SPEED: f32 = 30.0;
        const SIDE_SPEED: f32 = 30.0;
        const FLOATING_SPEED: f32 = 30.0;
        const DAMPING: f32 = 30.0;
        const ROLL: f32 = 0.5;
        const ROLL_SPEED: f32 = 2.0;
        const GRAVITY: f32 = 9.8;

        let mut goal_roll = 0.0;
        let mut acelleration_on_ground = false;
        if self._acceleration != Vector3::zeros() {
            if 0.0 != self._acceleration.x {
                let dir_side = Vector2::new(transform.get_left().x, transform.get_left().z).normalize();
                self._ground_velocity += dir_side * self._acceleration.x * SIDE_SPEED * delta_time;
                goal_roll = -ROLL * self._acceleration.x;
                acelleration_on_ground = true;
            }

            if 0.0 != self._acceleration.y {
                self._floating_velocity += self._acceleration.y * FLOATING_SPEED * delta_time;
            }

            if 0.0 != self._acceleration.z {
                let dir_forward = Vector2::new(transform.get_front().x, transform.get_front().z).normalize();
                self._ground_velocity += dir_forward * self._acceleration.z * FORWARD_SPEED * delta_time;
                acelleration_on_ground = true;
            }
        }

        // ground speed
        let mut ground_speed = self._ground_velocity.norm();
        if 0.0 != ground_speed && false == acelleration_on_ground {
            let damping = DAMPING * delta_time;
            ground_speed -= damping;
            if ground_speed < 0.0 {
                ground_speed = 0.0;
            }
            // ground speed limit
            ground_speed = ground_speed.min(MAX_GROUND_SPEED);

            self._ground_velocity = self._ground_velocity.normalize() * ground_speed;
        }

        // apply gravity
        if 0.0 == self._acceleration.y {
            self._floating_velocity -= GRAVITY * delta_time;
        }

        // apply velocity
        let mut velocity = Vector3::new(self._ground_velocity.x, self._floating_velocity, self._ground_velocity.y);
        let mut position = transform.get_position().clone() + &velocity * delta_time;

        // check height map
        if position != *transform.get_position() {
            let floating_height = height_map_data.get_height(&position, 0) + floating_height;
            if position.y < floating_height {
                position.y = floating_height;
                self._floating_velocity = 0.0;
            }
            transform.set_position(&position);
        }

        // roll
        let mut roll = transform.get_roll();
        if goal_roll != roll {
            let roll_diff = goal_roll - roll;
            let sign = if 0.0 <= roll_diff { 1.0 } else { -1.0 };
            let mut roll_speed = ROLL_SPEED * delta_time * sign;
            if roll_diff.abs() < roll_speed.abs() {
                roll = goal_roll;
            } else {
                roll += roll_speed * roll_diff.abs() / ROLL;
            }
            transform.set_roll(roll);
        }

        self._prev_ground_velocity.clone_from(&self._ground_velocity);
        self._prev_floating_velocity = self._floating_velocity;
        self._acceleration = Vector3::zeros();
    }
}