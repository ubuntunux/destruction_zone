use nalgebra::{ Vector3 };
use cgmath::num_traits::real::Real;
use rust_engine_3d::renderer::transform_object::TransformObjectData;

pub struct ActorController {
    pub _prev_velocity: Vector3<f32>,
    pub _velocity: Vector3<f32>,
    pub _acceleration: Vector3<f32>,
}

impl ActorController {
    pub fn create_actor_controller() -> ActorController {
        ActorController {
            _prev_velocity: Vector3::zeros(),
            _velocity: Vector3::zeros(),
            _acceleration: Vector3::zeros(),
        }
    }

    pub fn acceleration_forward(&mut self) {
        self._acceleration.z = 1.0;
    }

    pub fn acceleration_backward(&mut self) {
        self._acceleration.z = -1.0;
    }

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

    pub fn update_controller(&mut self, delta_time: f32, transform: &mut TransformObjectData) {
        const MAX_FORWARD_SPEED: f32 = 50.0;
        const MOVE_SPEED: f32 = 30.0;
        const DAMPING: f32 = 15.0;
        const ROLL: f32 = 0.5;
        const ROLL_SPEED: f32 = 2.0;
        const GRAVITY: f32 = -9.8;

        let mut accelration = false;
        let mut goal_roll = 0.0;
        let move_speed = MOVE_SPEED * delta_time;
        if self._acceleration != Vector3::zeros() {
            accelration = true;

            if 0.0 < self._acceleration.x {
                goal_roll = -ROLL;
                self._velocity += transform.get_left() * move_speed;
            } else if self._acceleration.x < 0.0 {
                goal_roll = ROLL;
                self._velocity -= transform.get_left() * move_speed;
            }

            if 0.0 < self._acceleration.y {
                self._velocity.y += move_speed;
            } else if self._acceleration.y < 0.0 {
                self._velocity.y -= move_speed;
            }

            if 0.0 < self._acceleration.z {
                self._velocity += transform.get_front() * move_speed;
            } else if self._acceleration.z < 0.0 {
                self._velocity -= transform.get_front() * move_speed;
            }
        }

        let mut speed = self._velocity.norm();
        if 0.0 < speed {
            // damping
            if false == accelration {
                let damping = DAMPING * delta_time;
                speed -= damping;
                if speed < 0.0 {
                    speed = 0.0;
                }
            }

            // limit
            speed = speed.min(MAX_FORWARD_SPEED);

            // apply velocity
            self._velocity = self._velocity.normalize() * speed;

            let position = transform.get_position().clone() + &self._velocity * delta_time;
            transform.set_position(&position);

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
        }

        self._prev_velocity.clone_from(&self._velocity);
        self._acceleration = Vector3::zeros();
    }
}