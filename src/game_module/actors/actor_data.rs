use nalgebra::Vector3;

use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::ptr_as_mut;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::game_client::GameClient;
use crate::game_module::game_controller::{GameViewMode, GameController};
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
    fn actor_fire(&mut self, game_client: &GameClient, game_view_mode: &GameViewMode);
    fn actor_move(&mut self, target_position: &Vector3<f32>);
    fn cancle_actor_move(&mut self);
    fn update_actor(&mut self, delta_time: f32, project_scene_manager: &ProjectSceneManager, game_controller: &GameController);
}

pub trait ActorBase {
    fn update_actor_base(&mut self, delta_time: f32, project_scene_manager: &ProjectSceneManager, game_controller: &GameController);
}

impl<T> ActorBase for T where T: ActorTrait {
    fn update_actor_base(&mut self, delta_time: f32, project_scene_manager: &ProjectSceneManager, _game_controller: &GameController) {
        let transform = ptr_as_mut(self.get_ship()._transform_object);
        let ship_controller = ptr_as_mut(&self.get_ship()._controller);

        ship_controller.update_controller(delta_time, transform, project_scene_manager);

        transform.set_rotation(ship_controller.get_rotation());
        transform.set_position(ship_controller.get_position());
        transform.update_matrix();

        // update ship
        self.get_ship_mut().update_ship(delta_time);
    }
}