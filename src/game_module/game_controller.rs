use nalgebra::{Vector2, Vector3};
use winit::event::VirtualKeyCode;

use rust_engine_3d::application::application::TimeData;
use rust_engine_3d::application::input::{KeyboardInputData, MouseMoveData, MouseInputData};
use rust_engine_3d::renderer::camera::CameraObjectData;
use rust_engine_3d::utilities::math;
use rust_engine_3d::utilities::system::{RcRefCell, WeakRefCell, into_WeakRefCell};
use crate::application::project_application::ProjectApplication;
use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::actors::player_actor::PlayerActor;
use crate::game_module::game_constants::{
    CAMERA_DISTANCE_MIN,
    CAMERA_DISTANCE_MAX,
    CAMERA_DISTANCE_SPEED,
    MOUSE_PITCH_MIN,
    MOUSE_PITCH_MAX,
    MOUSE_ROTATION_SPEED
};
use crate::game_module::game_ui::GameUIManager;
use crate::game_module::height_map_data::HeightMapData;


#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameViewMode {
    TopViewMode,
    FpsViewMode,
    Count
}

pub struct GameController {
    pub _game_ui_manager: *const GameUIManager,
    pub _main_camera: WeakRefCell<CameraObjectData>,
    pub _camera_distance: f32,
    pub _camera_goal_distance: f32,
    pub _game_view_mode: GameViewMode,
}

impl GameController {
    pub fn create_game_controller() -> Box<GameController> {
        let default_camera_distance = (CAMERA_DISTANCE_MIN + CAMERA_DISTANCE_MAX) * 0.5;
        Box::new(GameController {
            _game_ui_manager: std::ptr::null(),
            _main_camera: WeakRefCell::new(),
            _camera_distance: default_camera_distance,
            _camera_goal_distance: default_camera_distance,
            _game_view_mode: GameViewMode::TopViewMode,
        })
    }

    pub fn initialize_game_controller(&mut self, game_ui_manager: &GameUIManager, main_camera: &RcRefCell<CameraObjectData>) {
        self._game_ui_manager = game_ui_manager;
        self._main_camera = into_WeakRefCell(main_camera);

        self.change_view_mode(GameViewMode::TopViewMode);
    }

    pub fn get_game_ui_manager(&self) -> &GameUIManager { unsafe { &*self._game_ui_manager } }
    pub fn get_game_ui_manager_mut(&self) -> &mut GameUIManager { unsafe { &mut *(self._game_ui_manager as *mut GameUIManager) } }

    pub fn is_view_mode(&self, target_view_mode: GameViewMode) -> bool {
        if target_view_mode == self._game_view_mode { true } else { false }
    }

    pub fn change_view_mode(&mut self, view_mode: GameViewMode) {
        self.get_game_ui_manager_mut().set_crosshair_tracking_mouse(GameViewMode::TopViewMode == view_mode);
        self._game_view_mode = view_mode;
    }

    pub fn toggle_view_mode(&mut self) {
        let next_view_mode = (self._game_view_mode as i32 + 1) % GameViewMode::Count as i32;
        self.change_view_mode(unsafe { std::mem::transmute(next_view_mode) });
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

    pub fn update_event_for_top_view_mode(
        &mut self,
        _time_data: &TimeData,
        _keyboard_input_data: &KeyboardInputData,
        mouse_move_data: &MouseMoveData,
        mouse_input_data: &MouseInputData,
        mouse_delta: &Vector2<f32>,
        project_application: &ProjectApplication,
    ) {
        let btn_left: bool = mouse_input_data._btn_l_pressed;
        let btn_right_hold: bool = mouse_input_data._btn_r_hold;

        let main_camera_ref = self._main_camera.upgrade().unwrap();
        let mut main_camera = main_camera_ref.borrow_mut();
        let player_actor = project_application.get_game_client()._actor_manager.get_player_actor_mut();

        if btn_right_hold && 0.0 != mouse_delta.x {
            let yaw = main_camera._transform_object.get_yaw() - mouse_delta.x * MOUSE_ROTATION_SPEED;
            main_camera._transform_object.set_yaw(yaw);
        }

        self.get_game_ui_manager_mut().set_crosshair_pos(&mouse_move_data._mouse_pos);

        // fire
        if btn_left {
            let fire_dir: Vector3<f32> = -player_actor.get_transform_mut().get_front() as Vector3<f32>;
            player_actor.actor_fire(project_application, &fire_dir);
        }

        //actor_transform.get_front().clone() as Vector3<f32>,

        // let main_camera = project_application.get_project_scene_manager().get_main_camera().borrow();
        // let world_pos = main_camera.convert_screen_to_world(&mouse_move_data._mouse_pos);
        // let world_pos = world_pos + world_pos.normalize() * 100.0;
        // player_actor.get_transform_mut()
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

        let main_camera_ref = self._main_camera.upgrade().unwrap();
        let mut main_camera = main_camera_ref.borrow_mut();
        let player_actor = project_application.get_game_client()._actor_manager.get_player_actor_mut();

        // fire
        if btn_left {
            let fire_dir: Vector3<f32> = -main_camera.get_camera_front() as Vector3<f32>;
            player_actor.actor_fire(project_application, &fire_dir);
        }

        // move
        let player_ship_controller = player_actor.get_ship_mut().get_controller_mut();
        if 0.0 != mouse_delta.x {
            player_ship_controller.acceleration_yaw(-mouse_delta.x);
        }

        if 0.0 != mouse_delta.y {
            let pitch = MOUSE_PITCH_MIN.max(MOUSE_PITCH_MAX.min(main_camera._transform_object.get_pitch() - mouse_delta.y * MOUSE_ROTATION_SPEED));
            main_camera._transform_object.set_pitch(pitch);
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

    pub fn update_camera(
        &mut self,
        delta_time: f32,
        height_map_data: &HeightMapData,
        player_actor: &PlayerActor
    ) {
        let main_camera_ref = self._main_camera.upgrade().unwrap();
        let mut main_camera = main_camera_ref.borrow_mut();
        let ship_controller = player_actor.get_controller();
        let dist_ratio: f32 = self.get_camera_distance_ratio();
        if self._game_view_mode == GameViewMode::TopViewMode {
            let pitch: f32 = math::lerp(-25.0, -75.0, dist_ratio);
            main_camera._transform_object.set_pitch(math::degree_to_radian(pitch));
            //main_camera._transform_object.set_yaw(0.0);
        } else if self._game_view_mode == GameViewMode::FpsViewMode {
            main_camera._transform_object.rotation_yaw(ship_controller.get_velocity_yaw() * delta_time);
        } else {
            assert!(false, "Not implemented.");
        }
        main_camera._transform_object.update_transform_object();

        // set camera offset
        let mut cockpit_offset = main_camera._transform_object.get_front().clone();
        {
            cockpit_offset.y = 0.0;
            cockpit_offset.normalize_mut();
            if main_camera._transform_object.get_up().y < 0.0 {
                cockpit_offset = -cockpit_offset;
            }

            let bound_box = &player_actor.get_ship()._render_object.borrow()._bound_box;
            const BOUND_BOX_MIN: f32 = 2.0;
            cockpit_offset = cockpit_offset * -BOUND_BOX_MIN.max(bound_box._size.z * 0.5);
            cockpit_offset.y = BOUND_BOX_MIN.max(bound_box._size.y * 0.5);
        }

        let mut camera_pos = ship_controller.get_position() + main_camera._transform_object.get_front() * self._camera_distance + cockpit_offset;
        let floating_height = height_map_data.get_height(&camera_pos, 0) + 1.0;
        if camera_pos.y < floating_height {
            camera_pos.y = floating_height;
        }
        main_camera._transform_object.set_position(&camera_pos);
    }

    pub fn update_game_controller(&mut self, delta_time: f32, project_application: &ProjectApplication) {
        if self._camera_goal_distance != self._camera_distance {
            self._camera_distance = math::lerp(self._camera_distance, self._camera_goal_distance, 1.0f32.min(delta_time * CAMERA_DISTANCE_SPEED));
        }

        let height_map_data = project_application.get_project_scene_manager().get_height_map_data();
        let player_actor = project_application.get_game_client()._actor_manager.get_player_actor();
        self.update_camera(delta_time, height_map_data, player_actor);
    }
}