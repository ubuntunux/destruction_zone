use rust_engine_3d::renderer::transform_object::TransformObjectData;

use crate::game_module::height_map_data::HeightMapData;

pub trait BaseActor {
    fn initialize_actor(&mut self);
    fn is_player_actor(&self) -> bool;
    fn get_transform(&self) -> &TransformObjectData;
    fn get_transform_mut(&self) -> &mut TransformObjectData;
    fn update_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData);
}
