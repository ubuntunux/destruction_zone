use nalgebra::Vector3;

use rust_engine_3d::renderer::transform_object::TransformObjectData;

use crate::application::project_application::ProjectApplication;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::height_map_data::HeightMapData;
use crate::game_module::ship::ship::{ ShipInstance };
use crate::game_module::ship::ship_controller::ShipController;

pub struct ActorCreateInfo {

}

pub struct ActorData {

}

pub trait ActorTrait {
    fn initialize_actor(&mut self, project_scene_manager: &mut ProjectSceneManager);
    fn remove_actor(&mut self, project_scene_manager: &mut ProjectSceneManager);
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
    fn get_velocity(&self) -> &Vector3<f32>;
    fn actor_fire(&mut self, project_application: &ProjectApplication, fire_dir: &Vector3<f32>);
    fn update_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData);
}