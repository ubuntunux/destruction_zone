use nalgebra::Vector3;
use winit::event::VirtualKeyCode;

use crate::application_constants;
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

    pub fn initialize_game_client(&mut self) {
        self._actor_manager.initialize_actor_manager();
    }

    pub fn update_event(&self, project_application: &Application) {
        let application_data = project_application.get_application_data();
        let time_data = &application_data._time_data;
        let mouse_move_data = &application_data._mouse_move_data;
        let mouse_input_data = &application_data._mouse_input_data;
        let keyboard_input_data = &application_data._keyboard_input_data;

        const MOUSE_DELTA_RATIO: f32 = 500.0;
        let delta_time = time_data._delta_time;
        let _mouse_pos = &mouse_move_data._mouse_pos;
        let mouse_delta_x = mouse_move_data._mouse_pos_delta.x as f32 / application_data._window_size.0 as f32 * MOUSE_DELTA_RATIO;
        let mouse_delta_y = mouse_move_data._mouse_pos_delta.y as f32 / application_data._window_size.1 as f32 * MOUSE_DELTA_RATIO;
        let btn_left: bool = mouse_input_data._btn_l_hold;
        let btn_right: bool = mouse_input_data._btn_r_hold;
        let _btn_middle: bool = mouse_input_data._btn_m_hold;

        let pressed_key_a = keyboard_input_data.get_key_hold(VirtualKeyCode::A);
        let pressed_key_d = keyboard_input_data.get_key_hold(VirtualKeyCode::D);
        let pressed_key_w = keyboard_input_data.get_key_hold(VirtualKeyCode::W);
        let pressed_key_s = keyboard_input_data.get_key_hold(VirtualKeyCode::S);
        let pressed_key_q = keyboard_input_data.get_key_hold(VirtualKeyCode::Q);
        let pressed_key_e = keyboard_input_data.get_key_hold(VirtualKeyCode::E);
        let pressed_key_z = keyboard_input_data.get_key_hold(VirtualKeyCode::Z);
        let pressed_key_c = keyboard_input_data.get_key_hold(VirtualKeyCode::C);
        // let pressed_key_comma = keyboard_input_data.get_key_hold(VirtualKeyCode::Comma);
        // let pressed_key_period = keyboard_input_data.get_key_hold(VirtualKeyCode::Period);
        // let released_key_left_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::LBracket);
        // let released_key_right_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::RBracket);
        // let released_key_subtract = keyboard_input_data.get_key_released(VirtualKeyCode::Minus);
        // let released_key_equals = keyboard_input_data.get_key_released(VirtualKeyCode::Equals);
        // let pressed_key_tab = keyboard_input_data.get_key_hold(VirtualKeyCode::Tab);

        let mut main_camera = project_application.get_project_scene_manager()._main_camera.borrow_mut();
        let mut main_light = project_application.get_project_scene_manager()._main_light.borrow_mut();
        let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);
        let camera_move_speed_multiplier = if modifier_keys_shift { 2.0 } else { 1.0 };
        let move_speed: f32 = application_constants::CAMERA_MOVE_SPEED * camera_move_speed_multiplier * delta_time as f32;
        let pan_speed = application_constants::CAMERA_PAN_SPEED * camera_move_speed_multiplier;
        let _rotation_speed = application_constants::CAMERA_ROTATION_SPEED;
        #[cfg(target_os = "android")]
        let rotation_speed = 0.02 * delta_time as f32;
        #[cfg(not(target_os = "android"))]
        let rotation_speed = delta_time as f32;

        let mut player = project_application.get_project_scene_manager().get_skeletal_render_object("Player").unwrap().borrow_mut();

        if btn_right {
            main_camera._transform_object.rotation_pitch(-rotation_speed * mouse_delta_y as f32);
            main_camera._transform_object.rotation_yaw(-rotation_speed * mouse_delta_x as f32);
        }

        let mut player_pos: Vector3<f32> = player._transform_object.get_position().clone() as Vector3<f32>;

        if pressed_key_w {
            player_pos += player._transform_object.get_front() * move_speed;
        }
        else if pressed_key_s {
            player_pos -= player._transform_object.get_front() * move_speed;
        }

        if pressed_key_a {
            player_pos += player._transform_object.get_left() * move_speed;
        }
        else if pressed_key_d {
            player_pos -= player._transform_object.get_left() * move_speed;
        }

        if pressed_key_q {
            player_pos += player._transform_object.get_up() * move_speed;
        }
        else if pressed_key_e {
            player_pos -= player._transform_object.get_up() * move_speed;
        }

        let height_map_data = project_application.get_project_scene_manager().get_height_map_data();
        let height_pos_y = height_map_data.get_height(&player_pos, 1) + 3.0;
        if player_pos.y < height_pos_y {
            player_pos.y = height_pos_y;
        } else {

        }
        player._transform_object.set_yaw(main_camera._transform_object.get_yaw() + std::f32::consts::PI);
        player._transform_object.set_position(&player_pos);

        let camera_pos = &player_pos + main_camera._transform_object.get_front() * 8.0 + Vector3::new(0.0, 2.0, 0.0);
        main_camera._transform_object.set_position(&camera_pos);
    }

    pub fn update_game_client(&mut self) {
        self._actor_manager.update_actor_manager();
    }
}