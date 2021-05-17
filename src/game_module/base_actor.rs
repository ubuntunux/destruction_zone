use rust_engine_3d::renderer::transform_object::TransformObjectData;

use crate::game_module::height_map_data::HeightMapData;
use crate::game_module::actor_controller::ActorController;
use crate::game_module::armor::ArmorInstance;

pub trait BaseActor {
    fn initialize_actor(&mut self);
    fn is_player_actor(&self) -> bool;
    fn get_armor(&self) -> &ArmorInstance;
    fn get_armor_mut(&mut self) -> &mut ArmorInstance;
    fn get_controller(&self) -> &ActorController;
    fn get_controller_mut(&mut self) -> &mut ActorController;
    fn get_transform(&self) -> &TransformObjectData;
    fn get_transform_mut(&self) -> &mut TransformObjectData;
    fn update_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData);
}
