use nalgebra::{ Vector2 };

use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use rust_engine_3d::renderer::ui::{
    ProjectUIManagerBase,
    UIManagerData, Widget,
    UIWidgetTypes,
    WidgetDefault,
    HorizontalAlign,
    VerticalAlign,
    UILayoutType,
    Orientation
};
use rust_engine_3d::resource::resource::ProjectResourcesBase;

use crate::application::project_application::ProjectApplication;
use crate::game_module::actor_manager::ActorManager;
use crate::game_module::actors::base_actor::BaseActor;
use crate::renderer::project_ui::ProjectUIManager;

pub struct GameUIManager {
    pub _project_ui_manager: *const ProjectUIManager,
    pub _crosshair_widget: *const WidgetDefault,
    pub _crosshair_pos: Vector2<f32>,
    pub _target_info_layer: *mut WidgetDefault,
    pub _target_distance: *mut WidgetDefault,
    pub _target_hp: *mut WidgetDefault,
    pub _target_shield: *mut WidgetDefault,
}

impl GameUIManager {
    pub fn create_game_ui_manager() -> Box<GameUIManager> {
        Box::new(GameUIManager {
            _project_ui_manager: std::ptr::null(),
            _crosshair_widget: std::ptr::null(),
            _crosshair_pos: Vector2::zeros(),
            _target_info_layer: std::ptr::null_mut(),
            _target_distance: std::ptr::null_mut(),
            _target_hp: std::ptr::null_mut(),
            _target_shield: std::ptr::null_mut(),
        })
    }

    pub fn get_project_ui_manager(&self) -> &ProjectUIManager {
        unsafe { &*self._project_ui_manager }
    }

    pub fn get_project_ui_manager_mut(&self) -> &mut ProjectUIManager {
        unsafe { &mut *(self._project_ui_manager as *mut ProjectUIManager) }
    }

    pub fn initialize_game_ui_manager(&mut self, project_application: &ProjectApplication) {
        let project_resources = project_application.get_project_resources();
        self._project_ui_manager = project_application.get_project_ui_manager();

        let project_ui_manager = unsafe { &mut *(self._project_ui_manager as *mut ProjectUIManager) };
        let root_widget = project_ui_manager.get_root_widget_mut();

        static touch_down: fn(widget: *const dyn Widget) = |_widget: *const dyn Widget| {
            // println!("touch_down");
        };
        static touch_move: fn(widget: *const dyn Widget) = |_widget: *const dyn Widget| {
            // println!("touch_move");
        };
        static touch_up: fn(widget: *const dyn Widget) = |_widget: *const dyn Widget| {
            // println!("touch_up");
        };

        let window_size = &project_application.get_engine_application()._window_size;
        let window_center = Vector2::<f32>::new(window_size.x as f32 * 0.5, window_size.y as f32 * 0.5,);

        //
        let crosshair_widget: *mut WidgetDefault = UIManagerData::create_widget("cursor", UIWidgetTypes::Default) as *mut WidgetDefault;
        let ui_component = unsafe { &mut crosshair_widget.as_mut().unwrap().get_ui_component_mut() };
        let ui_size = 50.0f32;
        ui_component.set_pos(window_center.x - ui_size * 0.5, window_center.y - ui_size * 0.5);
        ui_component.set_size(ui_size, ui_size);
        ui_component.set_material_instance(&project_resources.get_material_instance_data("ui/crosshair"));
        ui_component._callback_touch_down = Some(&touch_down);
        ui_component._callback_touch_up = Some(&touch_up);
        ui_component._callback_touch_move = Some(&touch_move);
        root_widget.add_widget(crosshair_widget);
        self._crosshair_widget = crosshair_widget;

        let target_info_layout = unsafe { &mut *(UIManagerData::create_widget("target_info_layout", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = target_info_layout.get_ui_component_mut();
        let ui_size = 200.0f32;
        ui_component.set_size(ui_size, ui_size);
        ui_component.set_center(window_center.x, window_center.y);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_color(get_color32(255, 255, 255, 10));
        root_widget.add_widget(target_info_layout);
        self._target_info_layer = target_info_layout;

        let target_distance = unsafe { &mut *(UIManagerData::create_widget("target_distance", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = target_distance.get_ui_component_mut();
        ui_component.set_text("100m");
        ui_component.set_size(100.0, 50.0);
        ui_component.set_color(get_color32(255, 0, 0, 20));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::TOP);
        ui_component.set_expandable(true);
        target_info_layout.add_widget(target_distance);
        self._target_distance = target_distance;

        let target_hp = unsafe { &mut *(UIManagerData::create_widget("target_hp", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = target_hp.get_ui_component_mut();
        ui_component.set_text("hp");
        ui_component.set_size(100.0, 50.0);
        ui_component.set_color(get_color32(0, 255, 0, 20));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        target_info_layout.add_widget(target_hp);
        self._target_hp = target_hp;

        let target_shield = unsafe { &mut *(UIManagerData::create_widget("target_shield", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = target_shield.get_ui_component_mut();
        ui_component.set_text("shield");
        ui_component.set_size(100.0, 50.0);
        ui_component.set_color(get_color32(0, 0, 255, 20));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_halign(HorizontalAlign::RIGHT);
        ui_component.set_valign(VerticalAlign::BOTTOM);
        ui_component.set_expandable(true);
        target_info_layout.add_widget(target_shield);
        self._target_shield = target_shield;
    }

    pub fn update_game_ui(&mut self, project_application: &ProjectApplication, actor_manager: &ActorManager, _delta_time: f32) {
        let main_camera = &mut project_application.get_project_scene_manager()._main_camera.borrow_mut();
        let window_size = &project_application.get_engine_application()._window_size;
        self._crosshair_pos.x = window_size.x as f32 * 0.5;
        self._crosshair_pos.y = window_size.y as f32 * 0.5;
        let crosshair_widget = unsafe { &mut *(self._crosshair_widget as *mut WidgetDefault) };
        let ui_component = crosshair_widget.get_ui_component_mut();
        ui_component.set_center(self._crosshair_pos.x, self._crosshair_pos.y);

        let player_actor_pos = actor_manager.get_player_actor().get_transform().get_position();

        for (_id, actor) in actor_manager._actors.iter() {
            if false == actor.is_player_actor() {
                let actor_pos = actor.get_transform().get_position();
                let distance = (actor_pos - player_actor_pos).norm();
                let armor = actor.get_armor();
                let clamp: bool = true;
                let screen_pos: Vector2<f32> = main_camera.convert_to_screen_pos(actor_pos, clamp);
                let target_info_layer = unsafe { self._target_info_layer.as_mut().unwrap().get_ui_component_mut() };
                target_info_layer.set_center(screen_pos.x, screen_pos.y);

                let target_distance = unsafe { self._target_distance.as_mut().unwrap().get_ui_component_mut() };
                target_distance.set_text(&format!("{}m", distance as i32));

                let target_hp = unsafe { self._target_hp.as_mut().unwrap().get_ui_component_mut() };
                target_hp.set_text(&format!("{}", armor._hit_point as i32));

                let target_shield = unsafe { self._target_shield.as_mut().unwrap().get_ui_component_mut() };
                target_shield.set_text(&format!("{}", armor._shields as i32));
                break;
            }
        }
    }
}