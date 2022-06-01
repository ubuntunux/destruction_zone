use nalgebra::Vector2;
use winit::event::VirtualKeyCode;

use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;

use crate::application::project_application::ProjectApplication;
use crate::game_module::actor_manager::ActorManager;
use crate::game_module::game_constants::SCROLL_DELTA_TO_CAMERA_DISTANCE_SPEED;
use crate::game_module::game_controller::{GameViewMode, GameController};
use crate::game_module::game_ui::GameUIManager;
use crate::game_module::weapon_manager::WeaponManager;

pub struct GameClient {
    pub _actor_manager: Box<ActorManager>,
    pub _game_controller: Box<GameController>,
    pub _game_ui_manager: Box<GameUIManager>,
    pub _weapon_manager: Box<WeaponManager>
}

impl GameClient {
    pub fn create_game_client() -> Box<GameClient> {
        Box::new(GameClient {
            _actor_manager: ActorManager::create_actor_manager(),
            _game_controller: GameController::create_game_controller(),
            _game_ui_manager: GameUIManager::create_game_ui_manager(),
            _weapon_manager: WeaponManager::create_weapon_manager(),
        })
    }

    pub fn initialize_game_client(&mut self, project_application: &ProjectApplication) {
        // open scene
        project_application.get_project_scene_manager_mut().open_scene_data("default");

        // initialize game clients
        let main_camera = project_application.get_project_scene_manager().get_main_camera();
        self._game_ui_manager.initialize_game_ui_manager(project_application);
        self._game_controller.initialize_game_controller(&self._game_ui_manager, main_camera);
        self._actor_manager.initialize_actor_manager(project_application);
        self._weapon_manager.initialize_weapon_manager(project_application);
    }

    pub fn destroy_game_client(&mut self) {
        self._weapon_manager.destroy_weapon_manager();
        self._actor_manager.destroy_actor_manager();
        self._game_ui_manager.destroy_game_ui_manager();
    }

    pub fn update_event(&mut self, project_application: &ProjectApplication) {
        let engine_application = project_application.get_engine_application();
        let time_data = &engine_application._time_data;
        let mouse_move_data = &engine_application._mouse_move_data;
        let mouse_input_data = &engine_application._mouse_input_data;
        let keyboard_input_data = &engine_application._keyboard_input_data;
        let mouse_speed_ratio = engine_application._window_size.y as f32 / 1080.0;
        let mouse_delta: Vector2<f32> = Vector2::new(mouse_move_data._mouse_pos_delta.x as f32 / mouse_speed_ratio, mouse_move_data._mouse_pos_delta.y as f32 / mouse_speed_ratio);
        let scroll_delta = &mouse_move_data._scroll_delta;
        let pressed_key_c = keyboard_input_data.get_key_pressed(VirtualKeyCode::C);

        if 0 != scroll_delta.y {
            self._game_controller.update_camera_distance(-scroll_delta.y as f32 * SCROLL_DELTA_TO_CAMERA_DISTANCE_SPEED);
        }

        if pressed_key_c {
            self._game_controller.toggle_view_mode();
        }

        match self._game_controller._game_view_mode {
            GameViewMode::TopViewMode => self._game_controller.update_event_for_top_view_mode(
                time_data,
                &keyboard_input_data,
                &mouse_move_data,
                &mouse_input_data,
                &mouse_delta,
                project_application
            ),
            GameViewMode::FpsViewMode => self._game_controller.update_event_for_fps_view_mode(
                time_data,
                &keyboard_input_data,
                &mouse_move_data,
                &mouse_input_data,
                &mouse_delta,
                project_application
            ),
            _ => assert!(false, "Not implemented."),
        };
    }

    pub fn update_game_client(&mut self, project_application: *mut ProjectApplication) {
        let project_application = unsafe { &(*project_application) };
        let delta_time = project_application.get_engine_application()._time_data._delta_time as f32;
        self._game_controller.update_game_controller(delta_time, project_application);
        self._actor_manager.update_actor_manager(delta_time, project_application, self._game_controller.as_ref());
        self._weapon_manager.update_weapon_manager(delta_time, project_application, self._actor_manager.as_mut());
        self._game_ui_manager.update_game_ui(delta_time, project_application, self._actor_manager.as_ref());
    }
}