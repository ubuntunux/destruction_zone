use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::actor_manager::calc_floating_height;
use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::ship::ship_controller::{ShipController, ControllerDataType};
use crate::game_module::weapons::weapon::{WeaponTrait, WeaponData};
use crate::game_module::weapons::weapon::BeamEmitter;

#[derive(Clone, Debug)]
pub enum ShipDataType {
    Scout,
}

#[derive(Clone, Debug)]
pub struct ShipData {
    pub _ship_name: String,
    pub _ship_type: ShipDataType,
    pub _model_data_name: String,
    pub _hull_armor: f32,
    pub _shield_armor: f32,
    pub _max_hull: f32,
    pub _max_shields: f32,
}

pub struct ShipInstance {
    pub _ship_data: ShipData,
    pub _hull: f32,
    pub _shields: f32,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _transform_object: *mut TransformObjectData,
    pub _controller: ShipController,
    pub _weapons: Vec<Box<dyn WeaponTrait>>,
}

// Implementation
impl ShipData {
    pub fn create_ship_data(ship_data_type: ShipDataType) -> ShipData {
        match ship_data_type {
            ShipDataType::Scout => ShipData {
                _ship_name: "".to_string(),
                _ship_type: ShipDataType::Scout,
                _model_data_name: "".to_string(),
                _hull_armor: 0.0,
                _shield_armor: 0.0,
                _max_hull: 100.0,
                _max_shields: 10.0,
            }
        }
    }
}

impl ShipInstance {
    pub fn create_ship_instance(
        controller_type: ControllerDataType,
        ship_data_type: ShipDataType,
        render_object: &RcRefCell<RenderObjectData>
    ) -> ShipInstance {
        let ship_data = ShipData::create_ship_data(ship_data_type);
        let transform_object = (&mut render_object.borrow_mut()._transform_object as *mut TransformObjectData).clone();
        let floating_height = calc_floating_height(&render_object.borrow());
        ShipInstance {
            _ship_data: ship_data.clone(),
            _hull: ship_data._max_hull,
            _shields: ship_data._max_shields,
            _render_object: render_object.clone(),
            _transform_object: transform_object,
            _controller: ShipController::create_ship_controller(controller_type, floating_height),
            _weapons: Vec::new(),
        }
    }

    pub fn initialize_ship_instance(&mut self, owner_actor: *const dyn ActorTrait, weapon_data: *const WeaponData) {
        let weapon = BeamEmitter::create_beam_emitter(
            owner_actor,
            weapon_data,
            &TransformObjectData::new_transform_object_data()
        );
        self._weapons.push(weapon);
    }

    pub fn get_ship_data(&self) -> &ShipData {
        &self._ship_data
    }

    pub fn get_ship_mut(&mut self) -> &mut ShipData {
        &mut self._ship_data
    }

    pub fn get_controller(&self) -> &ShipController {
        &self._controller
    }

    pub fn get_controller_mut(&mut self) -> &mut ShipController {
        &mut self._controller
    }

    pub fn get_transform(&self) -> &TransformObjectData {
        unsafe { &(*self._transform_object) }
    }

    pub fn get_transform_mut(&self) -> &mut TransformObjectData {
        unsafe { &mut *(self._transform_object as *mut TransformObjectData) }
    }

    pub fn get_hull_point(&self) -> f32 {
        self._hull
    }

    pub fn get_max_hull_point(&self) -> f32 {
        self._ship_data._max_hull
    }

    pub fn get_shield_point(&self) -> f32 {
        self._shields
    }

    pub fn get_max_shield_point(&self) -> f32 {
        self._ship_data._max_shields
    }
}