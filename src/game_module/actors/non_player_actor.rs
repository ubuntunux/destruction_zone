use std::rc::Rc;
use nalgebra::Vector3;

use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor_data::{ ActorData, ActorTrait, ActorBase };
use crate::game_module::game_client::GameClient;
use crate::game_module::game_controller::{GameViewMode, GameController};
use crate::game_module::ship::ship_controller::{ ShipController };
use crate::game_module::ship::ship::{ShipInstance, ShipData};


pub struct NonPlayerActor {
    pub _id: u64,
    pub _actor_data: ActorData,
    pub _ship: ShipInstance,
}

impl ActorTrait for NonPlayerActor {
    fn initialize_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.initialize_ship_instance(self, project_scene_manager);
    }
    fn remove_actor(&mut self, project_scene_manager: &mut ProjectSceneManager) {
        self._ship.remove_ship_instance(project_scene_manager);
    }
    fn get_actor_id(&self) -> u64 {
        self._id
    }
    fn is_player_actor(&self) -> bool {
        false
    }
    fn get_actor_data(&self) -> &ActorData {
        &self._actor_data
    }
    fn get_actor_data_mut(&mut self) -> &mut ActorData {
        &mut self._actor_data
    }
    fn get_ship(&self) -> &ShipInstance {
        &self._ship
    }
    fn get_ship_mut(&mut self) -> &mut ShipInstance {
        &mut self._ship
    }
    fn get_controller(&self) -> &ShipController {
        &self._ship._controller
    }
    fn get_controller_mut(&mut self) -> &mut ShipController {
        &mut self._ship._controller
    }
    fn get_transform(&self) -> &TransformObjectData { self._ship.get_transform() }
    fn get_transform_mut(&self) -> &mut TransformObjectData {
        self._ship.get_transform_mut()
    }
    fn get_velocity(&self) -> &Vector3<f32> { self.get_controller().get_velocity() }
    fn actor_fire(&mut self, _game_client: &GameClient, _game_view_mode: &GameViewMode) {
        unimplemented!()
    }
    fn actor_move(&mut self, _target_position: &Vector3<f32>) {
        unimplemented!()
    }
    fn update_actor(&mut self, delta_time: f32, project_scene_manager: &ProjectSceneManager, game_controller: &GameController) {
        self.update_actor_base(delta_time, project_scene_manager, game_controller);
    }
}

impl NonPlayerActor {
    pub fn create_actor(
        id: u64,
        ship_data: &RcRefCell<ShipData>,
        render_object: &RcRefCell<RenderObjectData>
    ) -> Rc<NonPlayerActor> {
        Rc::new(NonPlayerActor {
            _id: id,
            _actor_data: ActorData {},
            _ship: ShipInstance::create_ship_instance(ship_data, render_object),
        })
    }
}