use nalgebra::Vector2;
use winit::event::VirtualKeyCode;

use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;

use crate::application::project_application::ProjectApplication;
use crate::game_module::actor_manager::ActorManager;
use crate::game_module::game_controller::GameController;
use crate::game_module::game_ui::GameUIManager;
use crate::game_module::weapon_manager::WeaponManager;
use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::game_constants::SCROLL_DELTA_TO_CAMERA_DISTANCE_SPEED;


pub struct GameClient {
    pub _game_controller: Box<GameController>,
    pub _actor_manager: Box<ActorManager>,
    pub _game_ui_manager: Box<GameUIManager>,
    pub _weapon_manager: Box<WeaponManager>
}

impl GameClient {
    pub fn create_game_client() -> Box<GameClient> {
        Box::new(GameClient {
            _game_controller: GameController::create_game_controller(),
            _actor_manager: ActorManager::create_actor_manager(),
            _game_ui_manager: GameUIManager::create_game_ui_manager(),
            _weapon_manager: WeaponManager::create_weapon_manager(),
        })
    }

    pub fn initialize_game_client(&mut self, project_application: &ProjectApplication) {
        // open scene
        project_application.get_project_scene_manager_mut().open_scene_data("default");

        self._game_ui_manager.initialize_game_ui_manager(project_application);
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
        let _delta_time = time_data._delta_time;
        let _mouse_pos = &mouse_move_data._mouse_pos;
        let mouse_delta: Vector2<f32> = Vector2::new(mouse_move_data._mouse_pos_delta.x as f32 / mouse_speed_ratio, mouse_move_data._mouse_pos_delta.y as f32 / mouse_speed_ratio);
        let scroll_delta = &mouse_move_data._scroll_delta;
        let btn_left: bool = mouse_input_data._btn_l_pressed;
        let _btn_right_hold: bool = mouse_input_data._btn_r_hold;

        let hold_key_aa = keyboard_input_data.get_key_hold(VirtualKeyCode::A);
        let hold_key_ad = keyboard_input_data.get_key_hold(VirtualKeyCode::D);
        let hold_key_aw = keyboard_input_data.get_key_hold(VirtualKeyCode::W);
        let hold_key_as = keyboard_input_data.get_key_hold(VirtualKeyCode::S);
        let hold_key_aq = keyboard_input_data.get_key_hold(VirtualKeyCode::Q);
        let hold_key_ae = keyboard_input_data.get_key_hold(VirtualKeyCode::E);
        let pressed_key_c = keyboard_input_data.get_key_pressed(VirtualKeyCode::C);
        // let hold_key_acomma = keyboard_input_data.get_key_hold(VirtualKeyCode::Comma);
        // let hold_key_aperiod = keyboard_input_data.get_key_hold(VirtualKeyCode::Period);
        // let released_key_left_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::LBracket);
        // let released_key_right_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::RBracket);
        // let released_key_subtract = keyboard_input_data.get_key_released(VirtualKeyCode::Minus);
        // let released_key_equals = keyboard_input_data.get_key_released(VirtualKeyCode::Equals);
        // let hold_key_atab = keyboard_input_data.get_key_hold(VirtualKeyCode::Tab);

        let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);

        if 0 != scroll_delta.y {
            self._game_controller.update_camera_distance(-scroll_delta.y as f32 * SCROLL_DELTA_TO_CAMERA_DISTANCE_SPEED);
        }

        if pressed_key_c {
            self._game_controller.toggle_view_mode();
        }

        let player_ship_controller = self._actor_manager.get_player_actor_mut().get_ship_mut().get_controller_mut();

        if btn_left {
            self._actor_manager.get_player_actor_mut().actor_fire(project_application, &self._game_controller);
        }

        if 0.0 != mouse_delta.x {
            player_ship_controller.acceleration_yaw(-mouse_delta.x);
        }

        if 0.0 != mouse_delta.y {
            player_ship_controller.acceleration_pitch(-mouse_delta.y);
        }

        if modifier_keys_shift {
            player_ship_controller.boost_on();
        }

        if hold_key_aw {
            player_ship_controller.acceleration_forward();
        }
        else if hold_key_as {
            player_ship_controller.acceleration_backward();
        }

        if hold_key_aa {
            player_ship_controller.acceleration_left();
        }
        else if hold_key_ad {
            player_ship_controller.acceleration_right();
        }

        if hold_key_aq {
            player_ship_controller.acceleration_down();
        }
        else if hold_key_ae {
            player_ship_controller.acceleration_up();
        }
    }

    pub fn update_game_client(&mut self, project_application: *mut ProjectApplication) {
        let project_application = unsafe { &(*project_application) };
        let delta_time = project_application.get_engine_application()._time_data._delta_time as f32;
        self._game_controller.update_game_controller(delta_time);
        self._actor_manager.update_actor_manager(delta_time, project_application, self._game_controller.as_ref());
        self._weapon_manager.update_weapon_manager(delta_time, project_application, self._actor_manager.as_mut());
        self._game_ui_manager.update_game_ui(delta_time, project_application, self._actor_manager.as_ref());
    }
}