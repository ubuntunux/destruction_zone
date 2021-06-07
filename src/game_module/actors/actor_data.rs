use rust_engine_3d::renderer::transform_object::TransformObjectData;

use crate::game_module::ship::ship::{ ShipInstance };
use crate::game_module::ship::ship_controller::ShipController;
use crate::game_module::height_map_data::HeightMapData;

pub struct ActorCreateInfo {

}

pub struct ActorData {

}

pub trait ActorTrait {
    fn initialize_actor(&mut self);
    fn get_actor_id(&self) -> u64;
    fn is_player_actor(&self) -> bool;
    fn get_actor_data(&self) -> &ActorData;
    fn get_actor_data_mut(&mut self) -> &mut ActorData;
    fn get_ship(&self) -> &ShipInstance;
    fn get_ship_mut(&mut self) -> &mut ShipInstance;
    fn get_controller(&self) -> &ShipController;
    fn get_controller_mut(&mut self) -> &mut ShipController;
    fn get_transform(&self) -> &TransformObjectData;
    fn get_transform_mut(&self) -> &mut TransformObjectData;
    fn fire(&mut self);
    fn update_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData);
}