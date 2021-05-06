use nalgebra::{ Vector3 };
use rust_engine_3d::renderer::transform_object::TransformObjectData;

pub struct ActorController {
    pub _velocity: Vector3<f32>,
    pub _transform: TransformObjectData,
}

impl ActorController {
    pub fn create_actor_controller() -> ActorController {
        ActorController {
            _velocity: Vector3::zeros(),
            _transform: TransformObjectData::new_transform_object_data(),
        }
    }

    pub fn update_controller(&mut self) {
        self._transform.update_transform_object();
    }
}