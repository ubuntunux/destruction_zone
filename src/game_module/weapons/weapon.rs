use crate::game_module::height_map_data::HeightMapData;

pub struct WeaponData {
    pub _reload_rate: f32,
    pub _bullet_amount: i32,
}

pub trait BaseWeapon {
    fn initialize_weapon(&mut self);
    fn update_weapon(&mut self, delta_time: f32, height_map_data: &HeightMapData);
}