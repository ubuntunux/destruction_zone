use rust_engine_3d::utilities::system::RcRefCell;
use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;

use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::height_map_data::HeightMapData;
use crate::game_module::weapons::bullet::{Bullet, BulletType};

#[derive(Clone, Copy)]
pub enum WeaponType {
    BeamEmitter,
    Gatling,
    LaserEmitter,
    PlasmaEmitter,
    Shotgun,
}

pub struct WeaponData {
    pub _rate_of_fire: f32,
    pub _bullet_amount: i32,
}

pub fn get_weapon_data(weapon_type: WeaponType) -> &'static WeaponData {
    static BEAM_EMITTER_DATA: WeaponData = WeaponData {
        _rate_of_fire: 1.0,
        _bullet_amount: 1,
    };
    match weapon_type {
        WeaponType::BeamEmitter => &BEAM_EMITTER_DATA,
        _ => &BEAM_EMITTER_DATA
    }
}

pub trait WeaponTrait {
    fn initialize_weapon(&mut self);
    fn update_weapon(&mut self, delta_time: f32, height_map_data: &HeightMapData);
}

pub struct BeamEmitter {
    pub _weapon_type: WeaponType,
    pub _weapon_data: *const WeaponData,
    pub _owner_actor: *const dyn ActorTrait,
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _transform_object: *mut TransformObjectData,
    pub _bullet_type: BulletType,
    pub _bullets: Vec<Box<Bullet>>,
    pub _bullets_id: u64,
}

// Implementation
impl BeamEmitter {
    pub fn create_beam_emitter(
        owner_actor: *const dyn ActorTrait,
        render_object: &RcRefCell<RenderObjectData>,
        offset_transform: &TransformObjectData
    ) -> Box<BeamEmitter> {
        render_object.borrow_mut()._transform_object = offset_transform.clone();
        let transform_object = (&mut render_object.borrow_mut()._transform_object as *mut TransformObjectData).clone();
        let weapon_type = WeaponType::BeamEmitter;
        Box::new(BeamEmitter {
            _weapon_type: weapon_type,
            _weapon_data: get_weapon_data(weapon_type),
            _owner_actor: owner_actor,
            _render_object: render_object.clone(),
            _transform_object: transform_object,
            _bullet_type: BulletType::Beam,
            _bullets: vec![],
            _bullets_id: std::u64::MAX,
        })
    }
}

impl WeaponTrait for BeamEmitter {
    fn initialize_weapon(&mut self) {
    }

    fn update_weapon(&mut self, delta_time: f32, height_map_data: &HeightMapData) {
        unimplemented!()
    }
}