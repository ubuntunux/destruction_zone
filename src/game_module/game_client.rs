use winit::event::VirtualKeyCode;

use rust_engine_3d::application::scene_manager::ProjectSceneManagerBase;

use crate::application::project_application::Application;
use crate::game_module::actor_manager::ActorManager;

pub struct GameClient {
    pub _actor_manager: Box<ActorManager>
}

impl GameClient {
    pub fn create_game_client() -> Box<GameClient> {
        Box::new(GameClient {
            _actor_manager: ActorManager::create_actor_manager(),
        })
    }

    pub fn initialize_game_client(&mut self, project_application: &Application) {
        // open scene
        project_application.get_project_scene_manager_mut().open_scene_data("default");

        self._actor_manager.initialize_actor_manager(project_application);
    }

    pub fn update_event(&self, project_application: &Application) {
        let application_data = project_application.get_application_data();
        let time_data = &application_data._time_data;
        let mouse_move_data = &application_data._mouse_move_data;
        let mouse_input_data = &application_data._mouse_input_data;
        let keyboard_input_data = &application_data._keyboard_input_data;

        const MOUSE_DELTA_RATIO: f32 = 500.0;
        let _delta_time = time_data._delta_time;
        let _mouse_pos = &mouse_move_data._mouse_pos;
        let mouse_delta = &mouse_move_data._mouse_pos_delta;
        let _btn_left: bool = mouse_input_data._btn_l_hold;
        let _btn_right: bool = mouse_input_data._btn_r_hold;
        let _btn_middle: bool = mouse_input_data._btn_m_hold;

        let pressed_key_a = keyboard_input_data.get_key_hold(VirtualKeyCode::A);
        let pressed_key_d = keyboard_input_data.get_key_hold(VirtualKeyCode::D);
        let pressed_key_w = keyboard_input_data.get_key_hold(VirtualKeyCode::W);
        let pressed_key_s = keyboard_input_data.get_key_hold(VirtualKeyCode::S);
        let pressed_key_q = keyboard_input_data.get_key_hold(VirtualKeyCode::Q);
        let pressed_key_e = keyboard_input_data.get_key_hold(VirtualKeyCode::E);
        // let pressed_key_comma = keyboard_input_data.get_key_hold(VirtualKeyCode::Comma);
        // let pressed_key_period = keyboard_input_data.get_key_hold(VirtualKeyCode::Period);
        // let released_key_left_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::LBracket);
        // let released_key_right_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::RBracket);
        // let released_key_subtract = keyboard_input_data.get_key_released(VirtualKeyCode::Minus);
        // let released_key_equals = keyboard_input_data.get_key_released(VirtualKeyCode::Equals);
        // let pressed_key_tab = keyboard_input_data.get_key_hold(VirtualKeyCode::Tab);

        let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);

        let player_actor = self._actor_manager.get_player_actor_mut();

        if 0 != mouse_delta.x {
            player_actor._controller.acceleration_yaw(-mouse_delta.x);
        }

        if 0 != mouse_delta.y {
            player_actor._controller.acceleration_pitch(-mouse_delta.y);
        }

        if modifier_keys_shift {
            player_actor._controller.boost_on();
        }

        if pressed_key_w {
            player_actor._controller.acceleration_forward();
        }
        else if pressed_key_s {
            player_actor._controller.acceleration_backward();
        }

        if pressed_key_a {
            player_actor._controller.acceleration_left();
        }
        else if pressed_key_d {
            player_actor._controller.acceleration_right();
        }

        if pressed_key_q {
            player_actor._controller.acceleration_up();
        }
        else if pressed_key_e {
            player_actor._controller.acceleration_down();
        }
    }

    pub fn update_game_client(&mut self, project_application: *mut Application) {
        let project_application = unsafe { &(*project_application) };
        let delta_time = project_application.get_application_data()._time_data._delta_time as f32;
        self._actor_manager.update_actor_manager(project_application, delta_time);
    }
}