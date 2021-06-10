use serde::{ Serialize, Deserialize };

use rust_engine_3d::renderer::render_object::{RenderObjectData, RenderObjectCreateInfo};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell};

use crate::application::project_application::ProjectApplication;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actor_manager::calc_floating_height;
use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::ship::ship_controller::{ShipController, ShipControllerData};
use crate::game_module::weapons::weapon::{WeaponTrait, WeaponData};
use crate::game_module::weapons::weapon::BeamEmitter;
use crate::game_module::height_map_data::HeightMapData;

#[derive(Serialize, Deserialize,Clone, Copy, Debug, PartialEq)]
pub enum ShipDataType {
    Scout,
    Tank,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ShipDataCreateInfo {
    pub _ship_type: ShipDataType,
    pub _model_data_name: String,
    pub _hull_armor: f32,
    pub _shield_armor: f32,
    pub _max_hull: f32,
    pub _max_shields: f32,
    pub _controller_data_name: String,
}

impl Default for ShipDataCreateInfo {
    fn default() -> ShipDataCreateInfo {
        ShipDataCreateInfo {
            _ship_type: ShipDataType::Scout,
            _model_data_name: "".to_string(),
            _hull_armor: 0.0,
            _shield_armor: 0.0,
            _max_hull: 100.0,
            _max_shields: 10.0,
            _controller_data_name: "".to_string(),
        }
    }
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
    pub _contoller_data: RcRefCell<ShipControllerData>,
}

pub struct ShipInstance {
    pub _ship_data: RcRefCell<ShipData>,
    pub _hull: f32,
    pub _shields: f32,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _transform_object: *mut TransformObjectData,
    pub _controller: ShipController,
    pub _weapons: Vec<Box<dyn WeaponTrait>>,
    pub _current_weapon_index: usize,
}

// Implementation
impl ShipData {
    pub fn create_ship_data(ship_data_name: &str, ship_data_create_info: &ShipDataCreateInfo, controller_data: &RcRefCell<ShipControllerData>) -> RcRefCell<ShipData> {
        newRcRefCell(ShipData {
            _ship_name: ship_data_name.to_string(),
            _ship_type: ship_data_create_info._ship_type,
            _model_data_name: ship_data_create_info._model_data_name.clone(),
            _hull_armor: ship_data_create_info._hull_armor,
            _shield_armor: ship_data_create_info._shield_armor,
            _max_hull: ship_data_create_info._max_hull,
            _max_shields: ship_data_create_info._max_shields,
            _contoller_data: controller_data.clone(),
        })
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
            _current_weapon_index: 0,
        }
    }

    pub fn initialize_ship_instance(&mut self, owner_actor: *const dyn ActorTrait, project_scene_manager: &mut ProjectSceneManager) {
        let ship_data = unsafe { &*self._ship_data.as_ptr() };
        self._hull = ship_data._max_hull;
        self._shields = ship_data._max_shields;

        // add weapons
        let weapon_data: RcRefCell<WeaponData> = project_scene_manager.get_project_resources().get_weapon_data("beam_emitter").clone();
        let render_object_create_info = RenderObjectCreateInfo {
            _model_data_name: weapon_data.borrow()._model_data_name.clone(),
            _position: self.get_transform().get_position().clone_owned(),
            ..Default::default()
        };
        let weapon_render_object = project_scene_manager.add_skeletal_render_object("weapon", &render_object_create_info);
        let mut weapon_offset_transform = TransformObjectData::new_transform_object_data();
        weapon_offset_transform.set_position_x(2.0);
        weapon_offset_transform.set_position_y(2.0);
        let weapon = BeamEmitter::create_beam_emitter(
            owner_actor,
            weapon_data.as_ptr().clone(),
            &weapon_offset_transform,
            &weapon_render_object,
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

    pub fn get_current_weapon(&self) -> Option<*const dyn WeaponTrait> {
        if self._current_weapon_index < self._weapons.len() {
            Some(self._weapons[self._current_weapon_index].as_ref())
        } else {
            None
        }
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

    pub fn fire(&mut self, project_application: &ProjectApplication) {
        let weapon: Option<*const dyn WeaponTrait> = self.get_current_weapon();
        if weapon.is_some() {
            let weapon: &mut dyn WeaponTrait = unsafe { &mut *(weapon.unwrap() as *mut dyn WeaponTrait) };
            weapon.fire(project_application);
        }
    }

    pub fn update_ship(&mut self, delta_time: f32, height_map_data: &HeightMapData) {
        let ship_transform = unsafe { &mut *(self._transform_object as *mut TransformObjectData) };

        for weapon in self._weapons.iter_mut() {
            weapon.update_weapon(ship_transform, delta_time, height_map_data);
        }
    }
}