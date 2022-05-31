use nalgebra::{Vector2, Vector3};
use winit::event::VirtualKeyCode;

use rust_engine_3d::application::application::TimeData;
use rust_engine_3d::application::input::{KeyboardInputData, MouseMoveData, MouseInputData};
use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;
use rust_engine_3d::utilities::math;
use crate::application::project_application::ProjectApplication;
use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::game_constants::{
    CAMERA_DISTANCE_MIN,
    CAMERA_DISTANCE_MAX,
    CAMERA_DISTANCE_SPEED,
};
use crate::game_module::game_ui::GameUIManager;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameViewMode {
    TopViewMode,
    FpsViewMode,
    Count
}

pub struct GameController {
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32,
    pub _game_view_mode: GameViewMode,
}

impl GameController {
    pub fn create_game_controller() -> Box<GameController> {
        let default_camera_distance = (CAMERA_DISTANCE_MIN + CAMERA_DISTANCE_MAX) * 0.5;
        Box::new(GameController {
            _camera_distance: default_camera_distance,
            _camera_goal_distance: default_camera_distance,
            _game_view_mode: GameViewMode::TopViewMode,
        })
    }

    pub fn initialize_game_controller(&mut self, game_ui_manager: &mut GameUIManager) {
        self.change_view_mode(game_ui_manager, GameViewMode::TopViewMode);
    }

    pub fn is_view_mode(&self, target_view_mode: GameViewMode) -> bool {
        if target_view_mode == self._game_view_mode { true } else { false }
    }

    pub fn change_view_mode(&mut self, game_ui_manager: &mut GameUIManager, view_mode: GameViewMode) {
        game_ui_manager.show_crosshair(GameViewMode::FpsViewMode == view_mode);
        self._game_view_mode = view_mode;
    }

    pub fn toggle_view_mode(&mut self, game_ui_manager: &mut GameUIManager) {
        let next_view_mode = (self._game_view_mode as i32 + 1) % GameViewMode::Count as i32;
        self.change_view_mode(game_ui_manager, unsafe { std::mem::transmute(next_view_mode) });
    }

    pub fn get_camera_distance_ratio(&self) -> f32 {
        (self._camera_distance - CAMERA_DISTANCE_MIN) / (CAMERA_DISTANCE_MAX - CAMERA_DISTANCE_MIN)
    }

    pub fn update_camera_distance(&mut self, distance: f32) {
        self._camera_goal_distance += distance;
        if self._camera_goal_distance < CAMERA_DISTANCE_MIN {
            self._camera_goal_distance = CAMERA_DISTANCE_MIN;
        } else if CAMERA_DISTANCE_MAX < self._camera_goal_distance {
            self._camera_goal_distance = CAMERA_DISTANCE_MAX;
        }
    }

    pub fn update_game_controller(&mut self, delta_time: f32) {
        if self._camera_goal_distance != self._camera_distance {
            self._camera_distance = math::lerp(self._camera_distance, self._camera_goal_distance, 1.0f32.min(delta_time * CAMERA_DISTANCE_SPEED));
        }
    }

    pub fn update_event_for_top_view_mode(
        &mut self,
        _time_data: &TimeData,
        _keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        _mouse_input_data: &MouseInputData,
        _mouse_delta: &Vector2<f32>,
        _project_application: &ProjectApplication,
    ) {
        //actor_transform.get_front().clone() as Vector3<f32>,
    }

    pub fn update_event_for_fps_view_mode(
        &mut self,
        _time_data: &TimeData,
        keyboard_input_data: &KeyboardInputData,
        _mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        mouse_delta: &Vector2<f32>,
        project_application: &ProjectApplication,
    ) {
        let btn_left: bool = mouse_input_data._btn_l_pressed;
        let hold_key_a = keyboard_input_data.get_key_hold(VirtualKeyCode::A);
        let hold_key_d = keyboard_input_data.get_key_hold(VirtualKeyCode::D);
        let hold_key_w = keyboard_input_data.get_key_hold(VirtualKeyCode::W);
        let hold_key_s = keyboard_input_data.get_key_hold(VirtualKeyCode::S);
        let hold_key_q = keyboard_input_data.get_key_hold(VirtualKeyCode::Q);
        let hold_key_e = keyboard_input_data.get_key_hold(VirtualKeyCode::E);
        let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);

        let player_actor = project_application.get_game_client()._actor_manager.get_player_actor_mut();

        // fire
        if btn_left {
            let main_camera = project_application.get_project_scene_manager().get_main_camera().borrow();
            let fire_dir: &Vector3<f32> = &main_camera.get_camera_front();
            player_actor.actor_fire(project_application, fire_dir);
        }

        // move
        let player_ship_controller = player_actor.get_ship_mut().get_controller_mut();
        if 0.0 != mouse_delta.x {
            player_ship_controller.acceleration_yaw(-mouse_delta.x);
        }

        if 0.0 != mouse_delta.y {
            player_ship_controller.acceleration_pitch(-mouse_delta.y);
        }

        if modifier_keys_shift {
            player_ship_controller.boost_on();
        }

        if hold_key_w {
            player_ship_controller.acceleration_forward();
        }
        else if hold_key_s {
            player_ship_controller.acceleration_backward();
        }

        if hold_key_a {
            player_ship_controller.acceleration_left();
        }
        else if hold_key_d {
            player_ship_controller.acceleration_right();
        }

        if hold_key_q {
            player_ship_controller.acceleration_down();
        }
        else if hold_key_e {
            player_ship_controller.acceleration_up();
        }
    }
}