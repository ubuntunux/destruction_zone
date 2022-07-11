use std::rc::Rc;
use nalgebra::Vector3;

use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;
use rust_engine_3d::renderer::render_object::{RenderObjectData};
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell, ptr_as_mut};
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::game_module::actors::actor_data::{ ActorData, ActorTrait };
use crate::game_module::game_client::GameClient;
use crate::game_module::game_controller::{ GameViewMode, GameController };
use crate::game_module::game_constants::{CHECK_TARGET_DISTANCE_MAX};
use crate::game_module::ship::ship::{ShipInstance, ShipData};
use crate::game_module::ship::ship_controller::{ ShipController };

pub struct PlayerActor {
    pub _id: u64,
    pub _actor_data: ActorData,
    pub _ship: ShipInstance,
    pub _target_position: Vector3<f32>,
}

impl ActorTrait for PlayerActor {
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
        true
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
    fn actor_fire(&mut self, game_client: &GameClient, game_view_mode: &GameViewMode) {
        let project_scene_manager = game_client.get_project_scene_manager();
        let mut fire_start: Vector3<f32> = Vector3::zeros();
        let mut fire_dir: Vector3<f32> = Vector3::zeros();
        match game_view_mode {
            GameViewMode::FpsViewMode => {
                let main_camera = project_scene_manager.get_main_camera();
                fire_start.clone_from(main_camera.get_camera_position());
                fire_dir = -main_camera.get_camera_front() as Vector3<f32>;
            },
            GameViewMode::TopViewMode => {
                fire_start.clone_from(self.get_transform().get_position());
                fire_dir = -self.get_transform().get_front() as Vector3<f32>;
            },
            _ => assert!(false, "Not implemented."),
        };
        let mut target_position: Vector3<f32> = &fire_start + &fire_dir * CHECK_TARGET_DISTANCE_MAX;
        project_scene_manager.get_height_map_collision_point(&fire_start, &fire_dir, CHECK_TARGET_DISTANCE_MAX, &mut target_position);

        self._ship.ship_fire(game_client, &fire_start, &fire_dir, &target_position);
    }

    fn actor_move(&mut self, target_position: &Vector3<f32>) {
        self._target_position.clone_from(target_position);
    }

    fn update_actor(&mut self, delta_time: f32, project_scene_manager: &ProjectSceneManager, game_controller: &GameController) {
        let transform = ptr_as_mut(self._ship._transform_object);
        let ship_controller = ptr_as_mut(&self._ship._controller);

        // move to target
        let acceleration = &self._target_position - ship_controller.get_position();
        if acceleration.x != 0f32 && acceleration.y != 0f32 && acceleration.z != 0f32 {
            ship_controller.acceleration_forward();
            let yaw: f32 = acceleration.x.atan2(acceleration.z);
            ship_controller.set_yaw(yaw);
        }

        ship_controller.update_controller(delta_time, transform, project_scene_manager);

        // update player transform
        if GameViewMode::TopViewMode == game_controller._game_view_mode {
            // transform.rotation_pitch(ship_controller.get_velocity_pitch() * delta_time);
            // transform.rotation_yaw(ship_controller.get_velocity_yaw() * delta_time);
            let roll_weight: f32 = 0.0;
            let yaw = ship_controller.get_yaw() - ship_controller.get_roll() * roll_weight;
            transform.set_yaw(yaw);
        } else if GameViewMode::FpsViewMode == game_controller._game_view_mode {
            // apply roll weight to pitch
            let roll_weight: f32 = 0.0;
            let yaw = ship_controller.get_yaw() - ship_controller.get_roll() * roll_weight;
            transform.set_yaw(yaw);
        } else {
            assert!(false, "Not implemented.");
        }
        transform.set_roll(ship_controller.get_rotation().z);
        transform.set_position(ship_controller.get_position());
        transform.update_matrix();

        // update ship
        self.get_ship_mut().update_ship(delta_time);
    }
}

impl PlayerActor {
    pub fn create_player_actor(
        id: u64,
        ship_data: &RcRefCell<ShipData>,
        render_object: &RcRefCell<RenderObjectData>
    ) -> Rc<PlayerActor> {
        Rc::new(PlayerActor {
            _id: id,
            _actor_data: ActorData {},
            _ship: ShipInstance::create_ship_instance(ship_data, render_object),
            _target_position: Vector3::zeros(),
        })
    }
}