use log::LevelFilter;

use ash::vk;
use sdl2::{ self, Sdl };
use winit::event::VirtualKeyCode;
use rust_engine_3d::constants;
use rust_engine_3d::application::application::{self, ApplicationBase, EngineApplication };

use crate::application_constants;
use crate::application::project_audio_manager::ProjectAudioManager;
use crate::application::project_scene_manager::ProjectSceneManager;
use crate::effect::effect_manager::ProjectEffectManager;
use crate::renderer::project_renderer::ProjectRenderer;
use crate::renderer::project_ui::ProjectUIManager;
use crate::resource::project_resource::ProjectResources;
use crate::game_module::game_client::GameClient;


pub struct ProjectApplication {
    pub _engine_application: *const EngineApplication,
    pub _project_resources: Box<ProjectResources>,
    pub _project_renderer: Box<ProjectRenderer>,
    pub _project_scene_manager: Box<ProjectSceneManager>,
    pub _project_effect_manager: Box<ProjectEffectManager>,
    pub _project_ui_manager: Box<ProjectUIManager>,
    pub _project_audio_manager: Box<ProjectAudioManager>,
    pub _game_client: Box<GameClient>,
    pub _is_game_mode: bool,
    pub _sdl: Sdl
}

impl ApplicationBase for ProjectApplication {
    fn initialize_application(&mut self, engine_application: &EngineApplication) {
        self._engine_application = engine_application;
        self._project_audio_manager.initialize_audio_manager(self, self._project_resources.as_ref());
        self._project_effect_manager.initialize_project_effect_manager();
        self.get_game_client_mut().initialize_game_client(self);
    }

    fn terminate_application(&mut self) {
        self._game_client.destroy_game_client();
        self._project_audio_manager.destroy_audio_manager();
        self._project_effect_manager.destroy_effect_manager();
    }

    fn update_event(&mut self) {
        if self._is_game_mode {
            self.get_game_client_mut().update_event(self);
        }

        if self.get_engine_application()._keyboard_input_data.get_key_pressed(VirtualKeyCode::Tab) {
            self.toggle_game_mode();
        }

        // EditorMode
        if false == self._is_game_mode {
            let engine_application = self.get_engine_application();
            let time_data = &engine_application._time_data;
            let mouse_move_data = &engine_application._mouse_move_data;
            let mouse_input_data = &engine_application._mouse_input_data;
            let keyboard_input_data = &engine_application._keyboard_input_data;

            const MOUSE_DELTA_RATIO: f32 = 500.0;
            let delta_time = time_data._delta_time;
            let _mouse_pos = &mouse_move_data._mouse_pos;
            let mouse_delta_x = mouse_move_data._mouse_pos_delta.x as f32 / engine_application._window_size.x as f32 * MOUSE_DELTA_RATIO;
            let mouse_delta_y = mouse_move_data._mouse_pos_delta.y as f32 / engine_application._window_size.y as f32 * MOUSE_DELTA_RATIO;
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
            let pressed_key_comma = keyboard_input_data.get_key_hold(VirtualKeyCode::Comma);
            let pressed_key_period = keyboard_input_data.get_key_hold(VirtualKeyCode::Period);
            let released_key_left_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::LBracket);
            let released_key_right_bracket = keyboard_input_data.get_key_released(VirtualKeyCode::RBracket);
            let released_key_subtract = keyboard_input_data.get_key_released(VirtualKeyCode::Minus);
            let released_key_equals = keyboard_input_data.get_key_released(VirtualKeyCode::Equals);

            let mut main_camera = self.get_project_scene_manager()._main_camera.borrow_mut();
            let mut main_light = self.get_project_scene_manager()._main_light.borrow_mut();
            let modifier_keys_shift = keyboard_input_data.get_key_hold(VirtualKeyCode::LShift);
            let camera_move_speed_multiplier = if modifier_keys_shift { 2.0 } else { 1.0 };
            let move_speed: f32 = application_constants::CAMERA_MOVE_SPEED * camera_move_speed_multiplier * delta_time as f32;
            let pan_speed = application_constants::CAMERA_PAN_SPEED * camera_move_speed_multiplier;
            let _rotation_speed = application_constants::CAMERA_ROTATION_SPEED;

            if released_key_left_bracket {
                self.get_project_renderer_mut().prev_debug_render_target();
            } else if released_key_right_bracket {
                self.get_project_renderer_mut().next_debug_render_target();
            }

            if released_key_subtract {
                self.get_project_renderer_mut().prev_debug_render_target_miplevel();
            } else if released_key_equals {
                self.get_project_renderer_mut().next_debug_render_target_miplevel();
            }

            #[cfg(target_os = "android")]
                let rotation_speed = 0.02 * delta_time as f32;
            #[cfg(not(target_os = "android"))]
                let rotation_speed = delta_time as f32;

            if pressed_key_comma {
                main_light._transform_object.rotation_pitch(rotation_speed);
            } else if pressed_key_period {
                main_light._transform_object.rotation_pitch(-rotation_speed);
            }

            if btn_left && btn_right {
                main_camera._transform_object.move_left(-pan_speed * mouse_delta_x as f32);
                main_camera._transform_object.move_up(pan_speed * mouse_delta_y as f32);
            }
            else if btn_right {
                main_camera._transform_object.rotation_pitch(-rotation_speed * mouse_delta_y as f32);
                main_camera._transform_object.rotation_yaw(-rotation_speed * mouse_delta_x as f32);
            }

            if pressed_key_z {
                main_camera._transform_object.rotation_roll(-rotation_speed * delta_time as f32);
            }
            else if pressed_key_c {
                main_camera._transform_object.rotation_roll(rotation_speed * delta_time as f32);
            }

            if pressed_key_w {
                main_camera._transform_object.move_front(-move_speed);
            }
            else if pressed_key_s {
                main_camera._transform_object.move_front(move_speed);
            }

            if pressed_key_a {
                main_camera._transform_object.move_left(-move_speed);
            }
            else if pressed_key_d {
                main_camera._transform_object.move_left(move_speed);
            }

            if pressed_key_q {
                main_camera._transform_object.move_up(-move_speed);
            }
            else if pressed_key_e {
                main_camera._transform_object.move_up(move_speed);
            }
        }
    }

    fn update_application(&mut self) {
        let application = self as *mut ProjectApplication;
        if self._is_game_mode {
            self._game_client.update_game_client(application);
        }
        self.get_project_audio_manager_mut().update_audio_manager();
    }
}

impl ProjectApplication {
    pub fn get_sdl(&self) -> &Sdl {
        &self._sdl
    }
    pub fn get_engine_application(&self) -> &EngineApplication {
        unsafe { &*self._engine_application }
    }
    pub fn get_engine_application_mut(&self) -> &mut EngineApplication {
        unsafe { &mut *(self._engine_application as *mut EngineApplication) }
    }
    pub fn get_project_effect_manager(&self) -> &ProjectEffectManager {
        &self._project_effect_manager
    }
    pub fn get_project_effect_manager_mut(&self) -> &mut ProjectEffectManager {
        unsafe { &mut *((self._project_effect_manager.as_ref() as *const ProjectEffectManager) as *mut ProjectEffectManager) }
    }
    pub fn get_project_resources(&self) -> &ProjectResources {
        &self._project_resources
    }
    pub fn get_project_resources_mut(&self) -> &mut ProjectResources {
        unsafe { &mut *((self._project_resources.as_ref() as *const ProjectResources) as *mut ProjectResources) }
    }
    pub fn get_project_scene_manager(&self) -> &ProjectSceneManager {
        &self._project_scene_manager
    }
    pub fn get_project_scene_manager_mut(&self) -> &mut ProjectSceneManager {
        unsafe { &mut *((self._project_scene_manager.as_ref() as *const ProjectSceneManager) as *mut ProjectSceneManager) }
    }
    pub fn get_project_renderer(&self) -> &ProjectRenderer {
        &self._project_renderer
    }
    pub fn get_project_renderer_mut(&self) -> &mut ProjectRenderer {
        unsafe { &mut *((self._project_renderer.as_ref() as *const ProjectRenderer) as *mut ProjectRenderer) }
    }
    pub fn get_project_ui_manager(&self) -> &ProjectUIManager {
        &self._project_ui_manager
    }
    pub fn get_project_ui_manager_mut(&self) -> &mut ProjectUIManager {
        unsafe { &mut *((self._project_ui_manager.as_ref() as *const ProjectUIManager) as *mut ProjectUIManager) }
    }
    pub fn get_project_audio_manager(&self) -> &ProjectAudioManager {
        &self._project_audio_manager
    }
    pub fn get_project_audio_manager_mut(&self) -> &mut ProjectAudioManager {
        unsafe { &mut *((self._project_audio_manager.as_ref() as *const ProjectAudioManager) as *mut ProjectAudioManager) }
    }
    pub fn get_game_client(&self) -> &GameClient {
        &self._game_client
    }
    pub fn get_game_client_mut(&self) -> &mut GameClient {
        unsafe { &mut *((self._game_client.as_ref() as *const GameClient) as *mut GameClient) }
    }

    pub fn toggle_game_mode(&mut self) {
        self.set_game_mode(!self._is_game_mode);
    }

    pub fn set_game_mode(&mut self, is_game_mode: bool) {
        self._is_game_mode = is_game_mode;
        self.get_engine_application_mut().set_grab_mode(is_game_mode);
    }
}

pub fn run_application() {
    let vulkan_api_version: u32;
    let enable_immediate_mode: bool;
    let enable_validation_layer: bool;
    let is_concurrent_mode: bool;

    #[cfg(target_os = "android")]
    {
        vulkan_api_version = vk::make_version(1, 0, 0);
        enable_immediate_mode = false;
        enable_validation_layer = false;
        is_concurrent_mode = false;
    }
    #[cfg(not(target_os = "android"))]
    {
        vulkan_api_version = vk::make_version(1, 2, 0);
        enable_immediate_mode = true;
        enable_validation_layer = true;
        is_concurrent_mode = true;
    }

    unsafe {
        constants::VULKAN_API_VERSION = vulkan_api_version;
        constants::DEBUG_MESSAGE_LEVEL = vk::DebugUtilsMessageSeverityFlagsEXT::WARNING;
        constants::VULKAN_LAYERS = vec!["VK_LAYER_LUNARG_standard_validation".to_string()];
        constants::REQUIRE_DEVICE_EXTENSIONS = vec!["VK_KHR_swapchain".to_string()];
        constants::ENABLE_IMMEDIATE_MODE = enable_immediate_mode;
        constants::ENABLE_VALIDATION_LAYER = enable_validation_layer;
        constants::IS_CONCURRENT_MODE = is_concurrent_mode;
        constants::METER_PER_UNIT = 1.0;
        constants::NEAR = 0.1;
        constants::FAR = 2000.0;
        constants::FOV = 60.0;
        // shadow
        constants::SHADOW_MAP_SIZE = 2048;
        constants::SHADOW_SAMPLES = 4;
        constants::SHADOW_BIAS = 0.004;
        constants::SHADOW_DISTANCE = 200.0;
        constants::SHADOW_DEPTH = 1000.0;
        constants::SHADOW_EXP = 500.0;
        // effect
        constants::MAX_EMITTER_COUNT = 1024;
        constants::MAX_PARTICLE_COUNT = 262144;
    }

    // create
    let sdl = sdl2::init().expect("failed to sdl2::init");
    let project_resources = ProjectResources::create_project_resources();
    let mut project_renderer = ProjectRenderer::create_project_renderer();
    let mut project_scene_manager = ProjectSceneManager::create_project_scene_manager();
    let project_effect_manager = ProjectEffectManager::create_project_effect_manager();
    let project_ui_manager = ProjectUIManager::create_project_ui_manager();
    let project_audio_manager = ProjectAudioManager::create_audio_manager(&sdl);
    let game_client = GameClient::create_game_client();

    // set manager
    project_renderer.set_project_effect_manager(project_effect_manager.as_ref());
    project_scene_manager.set_project_effect_manager(project_effect_manager.as_ref());

    // initialize
    let application = ProjectApplication {
        _engine_application: std::ptr::null(),
        _project_resources: project_resources,
        _project_renderer: project_renderer,
        _project_scene_manager: project_scene_manager,
        _project_effect_manager: project_effect_manager,
        _project_ui_manager: project_ui_manager,
        _project_audio_manager: project_audio_manager,
        _game_client: game_client,
        _sdl: sdl,
        _is_game_mode: false,
    };

    application::run_application(
        LevelFilter::Info,
        &application,
        application.get_project_resources(),
        application.get_project_scene_manager(),
        application.get_project_renderer(),
        application.get_project_ui_manager(),
    );
}