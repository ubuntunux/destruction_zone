use std::collections::HashMap;

use rust_engine_3d::renderer::render_object::{RenderObjectData, RenderObjectCreateInfo};
use rust_engine_3d::utilities::math::lerp;

use crate::application::project_application::ProjectApplication;
use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::actors::player_actor::PlayerActor;
use crate::game_module::actors::non_player_actor::NonPlayerActor;
use crate::game_module::ship::ship::{ShipDataType, ShipData, ShipDataCreateInfo};
use crate::game_module::game_constants::{ CAMERA_DISTANCE_MIN, CAMERA_DISTANCE_MAX, CAMERA_DISTANCE_SPEED};
use crate::game_module::ship::ship_controller::{create_controller_data, ControllerDataType};
use nalgebra::Vector3;

pub struct ActorManager {
    pub _id_generator: u64,
    pub _player_actor: *const PlayerActor,
    pub _actors: HashMap<u64, Box<dyn ActorTrait>>,
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32,
}

pub fn calc_floating_height(render_object: &RenderObjectData) -> f32 {
    render_object._bound_box._size.y * 0.5 + 2.0
}

impl ActorManager {
    pub fn create_actor_manager() -> Box<ActorManager> {
        let default_camera_distance = (CAMERA_DISTANCE_MIN + CAMERA_DISTANCE_MAX) * 0.5;
        Box::new(ActorManager {
            _id_generator: 0,
            _player_actor: std::ptr::null(),
            _actors: HashMap::new(),
            _camera_distance: default_camera_distance,
            _camera_goal_distance: default_camera_distance,
        })
    }

    pub fn initialize_actor_manager(&mut self, project_application: &ProjectApplication) {
        // Player Actor
        {
            let id = self.generate_id();
            let ship_data_create_info = ShipDataCreateInfo {
                _ship_name: "scout".to_string(),
                _ship_type: ShipDataType::Scout,
                _model_data_name: "ship/trident".to_string(),
                _hull_armor: 0.0,
                _shield_armor: 0.0,
                _max_hull: 100.0,
                _max_shields: 10.0,
                _contoller_data_name: "ship_controller/light_ship_controller".to_string(),
            };
            let ship_controller_data = create_controller_data(ControllerDataType::ShipController);
            let render_object_create_info = RenderObjectCreateInfo {
                _model_data_name: ship_data_create_info._model_data_name.clone(),
                ..Default::default()
            };
            let ship_data = ShipData::create_ship_data(&ship_data_create_info, &ship_controller_data);
            let player_render_object = project_application.get_project_scene_manager_mut().add_skeletal_render_object("Player", &render_object_create_info);
            self._actors.insert(id, PlayerActor::create_player_actor(id, &ship_data, &player_render_object));
            self._player_actor = (self._actors.get(&id).unwrap().as_ref() as *const dyn ActorTrait) as *const PlayerActor;
            let player_actor = unsafe { &mut *(self._player_actor as *mut PlayerActor) };
            player_actor.initialize_actor();
        }

        // AI Actor
        {
            let id = self.generate_id();
            let ship_data_create_info = ShipDataCreateInfo {
                _ship_name: "scout".to_string(),
                _ship_type: ShipDataType::Scout,
                _model_data_name: "ship/trident".to_string(),
                _hull_armor: 0.0,
                _shield_armor: 0.0,
                _max_hull: 100.0,
                _max_shields: 10.0,
                _contoller_data_name: "ship_controller/light_ship_controller".to_string(),
            };
            let ship_controller_data = create_controller_data(ControllerDataType::ShipController);
            let render_object_create_info = RenderObjectCreateInfo {
                _model_data_name: ship_data_create_info._model_data_name.clone(),
                ..Default::default()
            };
            let ship_data = ShipData::create_ship_data(&ship_data_create_info, &ship_controller_data);
            let actor_render_object = project_application.get_project_scene_manager_mut().add_skeletal_render_object("Enemy", &render_object_create_info);
            self._actors.insert(id, NonPlayerActor::create_actor(id, &ship_data, &actor_render_object));
            let actor = (self._actors.get(&id).unwrap().as_ref() as *const dyn ActorTrait) as *const NonPlayerActor;
            let actor = unsafe { &mut *(actor as *mut NonPlayerActor) };
            actor.initialize_actor();
        }
    }

    pub fn destroy_actor_manager(&mut self) {

    }

    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }

    pub fn get_player_actor(&self) -> &PlayerActor {
        unsafe { &*self._player_actor }
    }

    pub fn get_player_actor_mut(&self) -> &mut PlayerActor {
        unsafe { &mut *(self._player_actor as *mut PlayerActor) }
    }

    pub fn update_camera_distance(&mut self, distance: f32) {
        self._camera_goal_distance += distance;
        if self._camera_goal_distance < CAMERA_DISTANCE_MIN {
            self._camera_goal_distance = CAMERA_DISTANCE_MIN;
        } else if CAMERA_DISTANCE_MAX < self._camera_goal_distance {
            self._camera_goal_distance = CAMERA_DISTANCE_MAX;
        }
    }

    pub fn update_actor_manager(&mut self, project_application: &ProjectApplication, delta_time: f32) {
        if self._camera_goal_distance != self._camera_distance {
            self._camera_distance = lerp(self._camera_distance, self._camera_goal_distance, 1.0f32.min(delta_time * CAMERA_DISTANCE_SPEED));
        }

        let height_map_data = project_application.get_project_scene_manager().get_height_map_data();

        let mut main_camera = &mut project_application.get_project_scene_manager()._main_camera.borrow_mut();
        let player_actor = self.get_player_actor_mut();
        player_actor.update_player_actor(delta_time, height_map_data, &mut main_camera, self._camera_distance);

        for actor in self._actors.values_mut() {
            if false == actor.is_player_actor() {
                let ship_controller = actor.get_ship_mut().get_controller_mut();
                {
                    ship_controller.acceleration_yaw(1000.0 * delta_time);
                    ship_controller.acceleration_forward();
                    ship_controller.acceleration_right();
                }
                actor.update_actor(delta_time, height_map_data);
            }
        }
    }
}