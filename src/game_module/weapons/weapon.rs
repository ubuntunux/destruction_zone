use serde::{ Serialize, Deserialize };

use rust_engine_3d::renderer::transform_object::TransformObjectData;

use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::height_map_data::HeightMapData;
use crate::game_module::weapons::bullet::{Bullet, BulletType, BulletData};
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell};
use rust_engine_3d::renderer::render_object::RenderObjectData;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum WeaponType {
    BeamEmitter,
    Gatling,
    LaserEmitter,
    PlasmaEmitter,
    Shotgun,
}

pub const WEAPON_TYPES: [WeaponType; 5] = [
    WeaponType::BeamEmitter,
    WeaponType::Gatling,
    WeaponType::LaserEmitter,
    WeaponType::PlasmaEmitter,
    WeaponType::Shotgun
];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct WeaponDataCreateInfo {
    pub _weapon_type: WeaponType,
    pub _rate_of_fire: f32,
    pub _bullet_amount: i32,
    pub _bullet_data_name: String,
    pub _model_data_name: String,
}

impl Default for WeaponDataCreateInfo {
    fn default() -> WeaponDataCreateInfo {
        WeaponDataCreateInfo {
            _weapon_type: WeaponType::BeamEmitter,
            _rate_of_fire: 1.0,
            _bullet_amount: 1,
            _bullet_data_name: "".to_string(),
            _model_data_name: "".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct WeaponData {
    pub _weapon_data_name: String,
    pub _weapon_type: WeaponType,
    pub _rate_of_fire: f32,
    pub _bullet_amount: i32,
    pub _bullet_data: RcRefCell<BulletData>,
    pub _model_data_name: String,
}

pub trait WeaponTrait {
    fn initialize_weapon(&mut self);
    fn get_owner_actor(&self) -> &dyn ActorTrait;
    fn get_bullet_type(&self) -> BulletType;
    fn get_bullet_data(&self) -> &BulletData;
    fn get_weapon_type(&self) -> WeaponType;
    fn get_weapon_data(&self) -> &WeaponData;
    fn update_weapon(&mut self, delta_time: f32, height_map_data: &HeightMapData);
}

pub struct BeamEmitter {
    pub _owner_actor: *const dyn ActorTrait,
    pub _weapon_data: *const WeaponData,
    pub _initial_offset_transform: TransformObjectData,
    pub _transform_object: TransformObjectData,
    pub _weapon_render_object: RcRefCell<RenderObjectData>,
    pub _bullets: Vec<Box<Bullet>>,
}

// Implementation
impl WeaponData {
    pub fn create_weapon_data(weapon_data_name: &str, weapon_data_create_info: &WeaponDataCreateInfo, bullet_data: &RcRefCell<BulletData>) -> RcRefCell<WeaponData> {
        newRcRefCell(WeaponData {
            _weapon_data_name: weapon_data_name.to_string(),
            _weapon_type: weapon_data_create_info._weapon_type,
            _rate_of_fire: weapon_data_create_info._rate_of_fire,
            _bullet_amount: weapon_data_create_info._bullet_amount,
            _bullet_data: bullet_data.clone(),
            _model_data_name: weapon_data_create_info._model_data_name.clone(),
        })
    }
}

impl BeamEmitter {
    pub fn create_beam_emitter(
        owner_actor: *const dyn ActorTrait,
        weapon_data: *const WeaponData,
        offset_transform: &TransformObjectData,
        weapon_render_object: &RcRefCell<RenderObjectData>,
    ) -> Box<BeamEmitter> {
        Box::new(BeamEmitter {
            _weapon_data: weapon_data,
            _owner_actor: owner_actor,
            _initial_offset_transform: offset_transform.clone(),
            _transform_object: TransformObjectData::new_transform_object_data(),
            _weapon_render_object: weapon_render_object.clone(),
            _bullets: vec![],
        })
    }
}

impl WeaponTrait for BeamEmitter {
    fn initialize_weapon(&mut self) {
    }

    fn get_owner_actor(&self) -> &dyn ActorTrait { unsafe { &*self._owner_actor } }
    fn get_bullet_type(&self) -> BulletType { self.get_bullet_data()._bullet_type }
    fn get_bullet_data(&self) -> &BulletData { unsafe { &*self.get_weapon_data()._bullet_data.as_ptr() } }
    fn get_weapon_type(&self) -> WeaponType { self.get_weapon_data()._weapon_type }
    fn get_weapon_data(&self) -> &WeaponData {
        unsafe { &*self._weapon_data }
    }
    fn update_weapon(&mut self, _delta_time: f32, _height_map_data: &HeightMapData) {
    }
}