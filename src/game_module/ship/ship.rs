use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell};

use crate::game_module::actor_manager::calc_floating_height;
use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::ship::ship_controller::{ShipController, ControllerDataType, create_controller_data, ControllerData};
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
    pub _contoller_data: RcRefCell<ControllerData>,
}

pub struct ShipInstance {
    pub _ship_data: RcRefCell<ShipData>,
    pub _hull: f32,
    pub _shields: f32,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _transform_object: *mut TransformObjectData,
    pub _controller: ShipController,
    pub _weapons: Vec<Box<dyn WeaponTrait>>,
}

// Implementation
impl ShipData {
    pub fn create_ship_data(ship_data_type: ShipDataType) -> RcRefCell<ShipData> {
        let controller_data = create_controller_data(ControllerDataType::ShipController);
        let ship_data = match ship_data_type {
            ShipDataType::Scout => ShipData {
                _ship_name: "".to_string(),
                _ship_type: ShipDataType::Scout,
                _model_data_name: "".to_string(),
                _hull_armor: 0.0,
                _shield_armor: 0.0,
                _max_hull: 100.0,
                _max_shields: 10.0,
                _contoller_data: controller_data,
            }
        };
        newRcRefCell(ship_data)
    }
}

impl ShipInstance {
    pub fn create_ship_instance(
        ship_data: &RcRefCell<ShipData>,
        render_object: &RcRefCell<RenderObjectData>
    ) -> ShipInstance {
        let transform_object = (&mut render_object.borrow_mut()._transform_object as *mut TransformObjectData).clone();
        let floating_height = calc_floating_height(&render_object.borrow());
        ShipInstance {
            _ship_data: ship_data.clone(),
            _hull: 0.0,
            _shields: 0.0,
            _render_object: render_object.clone(),
            _transform_object: transform_object,
            _controller: ShipController::create_ship_controller(&ship_data.borrow()._contoller_data, floating_height),
            _weapons: Vec::new(),
        }
    }

    pub fn initialize_ship_instance(&mut self, owner_actor: *const dyn ActorTrait, weapon_data: *const WeaponData) {
        let ship_data = unsafe { &*self._ship_data.as_ptr() };
        self._hull = ship_data._max_hull;
        self._shields = ship_data._max_shields;

        let weapon = BeamEmitter::create_beam_emitter(
            owner_actor,
            weapon_data,
            &TransformObjectData::new_transform_object_data()
        );
        self._weapons.push(weapon);
    }

    pub fn get_ship_data(&self) -> &ShipData {
        unsafe { &*self._ship_data.as_ptr() }
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
        self.get_ship_data()._max_hull
    }

    pub fn get_shield_point(&self) -> f32 {
        self._shields
    }

    pub fn get_max_shield_point(&self) -> f32 {
        self.get_ship_data()._max_shields
    }
}