use nalgebra::{ Vector2 };

use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use rust_engine_3d::renderer::ui::{
    ProjectUIManagerBase,
    UIManager, Widget,
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
use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::ui_widgets::hit_point_widgets::{ HullPointWidget, ShieldPointWidget };
use crate::renderer::project_ui::ProjectUIManager;


pub struct GameUIManager {
    pub _project_ui_manager: *const ProjectUIManager,
    pub _crosshair_widget: *const WidgetDefault,
    pub _crosshair_pos: Vector2<i32>,
    pub _crosshair_tracking_mouse: bool,
    pub _target_hud_layer: *mut WidgetDefault,
    pub _target_distance: *mut WidgetDefault,
    pub _target_hull_point_widget: Option<HullPointWidget>,
    pub _target_shield_point_widget: Option<ShieldPointWidget>,
    pub _player_hud_layer: *mut WidgetDefault,
    pub _player_hull_point_widget: Option<HullPointWidget>,
    pub _player_shield_point_widget: Option<ShieldPointWidget>,
}

impl GameUIManager {
    pub fn create_game_ui_manager() -> Box<GameUIManager> {
        Box::new(GameUIManager {
            _project_ui_manager: std::ptr::null(),
            _crosshair_widget: std::ptr::null(),
            _crosshair_pos: Vector2::zeros(),
            _crosshair_tracking_mouse: true,
            _target_hud_layer: std::ptr::null_mut(),
            _target_distance: std::ptr::null_mut(),
            _target_hull_point_widget: None,
            _target_shield_point_widget: None,
            _player_hud_layer: std::ptr::null_mut(),
            _player_hull_point_widget: None,
            _player_shield_point_widget: None,
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

        static TOUCH_DOWN: fn(widget: *const dyn Widget) = |_widget: *const dyn Widget| {
            // println!("touch_down");
        };
        static TOUCH_MOVE: fn(widget: *const dyn Widget) = |_widget: *const dyn Widget| {
            // println!("touch_move");
        };
        static TOUCH_UP: fn(widget: *const dyn Widget) = |_widget: *const dyn Widget| {
            // println!("touch_up");
        };

        let window_size = &project_application.get_engine_application()._window_size;
        let window_center = Vector2::<f32>::new(window_size.x as f32 * 0.5, window_size.y as f32 * 0.5,);

        //
        let crosshair_widget: *mut WidgetDefault = UIManager::create_widget("cursor", UIWidgetTypes::Default) as *mut WidgetDefault;
        let ui_component = unsafe { &mut crosshair_widget.as_mut().unwrap().get_ui_component_mut() };
        let ui_size = 50.0f32;
        ui_component.set_pos(window_center.x - ui_size * 0.5, window_center.y - ui_size * 0.5);
        ui_component.set_size(ui_size, ui_size);
        ui_component.set_material_instance(&project_resources.get_material_instance_data("ui/crosshair"));
        ui_component._callback_touch_down = Some(&TOUCH_DOWN);
        ui_component._callback_touch_up = Some(&TOUCH_UP);
        ui_component._callback_touch_move = Some(&TOUCH_MOVE);
        root_widget.add_widget(crosshair_widget);
        self._crosshair_widget = crosshair_widget;

        let hud_layer_width: f32 = 100.0;
        let hud_layer_height: f32 = 100.0;
        let hud_layer_padding: f32 = 10.0;
        let hud_ui_width: f32 = 100.0;
        let hud_ui_height: f32 = 25.0;
        let hud_ui_margine: f32 = 2.0;
        let hud_ui_padding: f32 = 4.0;

        // Target Hud
        let target_hud_layer = unsafe { &mut *(UIManager::create_widget("target_hud_layer", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = target_hud_layer.get_ui_component_mut();
        ui_component.set_size(hud_layer_width, hud_layer_height);
        ui_component.set_center(window_center.x, window_center.y);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_padding(hud_layer_padding);
        ui_component.set_color(get_color32(255, 255, 255, 10));
        ui_component.set_opacity(0.5);
        root_widget.add_widget(target_hud_layer);
        self._target_hud_layer = target_hud_layer;

        let target_distance = unsafe { &mut *(UIManager::create_widget("target_distance", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = target_distance.get_ui_component_mut();
        ui_component.set_text("100m");
        ui_component.set_size(hud_ui_width, hud_ui_height);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(255, 0, 0, 20));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_margine(hud_ui_margine);
        ui_component.set_padding(hud_ui_padding);
        ui_component.set_expandable(true);
        target_hud_layer.add_widget(target_distance);
        self._target_distance = target_distance;

        self._target_hull_point_widget = Some(HullPointWidget::create_hull_point_widget(target_hud_layer));
        self._target_shield_point_widget = Some(ShieldPointWidget::create_shield_point_widget(target_hud_layer));

        // Player Hud
        let player_hud_layer = unsafe { &mut *(UIManager::create_widget("player_hud_layer", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = player_hud_layer.get_ui_component_mut();
        ui_component.set_size(hud_layer_width, hud_layer_height);
        ui_component.set_pos(window_size.x as f32 - 200.0, window_center.y);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_padding(hud_layer_padding);
        ui_component.set_color(get_color32(255, 255, 255, 10));
        root_widget.add_widget(player_hud_layer);
        self._player_hud_layer = player_hud_layer;

        self._player_hull_point_widget = Some(HullPointWidget::create_hull_point_widget(player_hud_layer));
        self._player_shield_point_widget = Some(ShieldPointWidget::create_shield_point_widget(player_hud_layer));
    }

    pub fn destroy_game_ui_manager(&mut self) {
    }

    pub fn get_crosshair_mut(&mut self) -> &mut WidgetDefault {
        unsafe { &mut *(self._crosshair_widget as *mut WidgetDefault) }
    }

    pub fn show_crosshair(&mut self, show: bool) {
        let ui_component = self.get_crosshair_mut().get_ui_component_mut();
        ui_component.set_visible(show);
    }

    pub fn set_crosshair_tracking_mouse(&mut self, tracking: bool) {
        self._crosshair_tracking_mouse = tracking;
    }

    pub fn set_crosshair_pos(&mut self, pos: &Vector2<i32>) {
        self._crosshair_pos.clone_from(pos);
    }

    pub fn update_game_ui(&mut self, _delta_time: f32, project_application: &ProjectApplication, actor_manager: &ActorManager) {
        let main_camera = &mut project_application.get_project_scene_manager()._main_camera.borrow_mut();
        let window_size = &project_application.get_engine_application()._window_size;

        // Cross Hair
        let crosshair_widget = unsafe { &mut *(self._crosshair_widget as *mut WidgetDefault) };
        if crosshair_widget._ui_component.get_visible() {
            let crosshair_pos_x: i32;
            let crosshair_pos_y: i32;

            if self._crosshair_tracking_mouse {
                crosshair_pos_x = self._crosshair_pos.x;
                crosshair_pos_y = self._crosshair_pos.y;
            } else {
                crosshair_pos_x = window_size.x / 2;
                crosshair_pos_y = window_size.y / 2;
            }
            let ui_component = crosshair_widget.get_ui_component_mut();
            ui_component.set_center(crosshair_pos_x as f32, crosshair_pos_y as f32);
        }

        // Player Hud
        {
            let player_actor = actor_manager.get_player_actor();
            let ship = player_actor.get_ship();
            self._target_hull_point_widget.as_ref().unwrap().update_hull_point_widget(ship.get_hull_point() / 2.0, ship.get_max_hull_point());
            self._target_shield_point_widget.as_ref().unwrap().update_shield_point_widget(ship.get_shield_point() / 2.0, ship.get_max_shield_point());
        }

        // Target Hud
        let player_actor_pos = actor_manager.get_player_actor().get_transform().get_position();
        for (_id, actor) in actor_manager._actors.iter() {
            if false == actor.is_player_actor() {
                let actor_pos = actor.get_transform().get_position();
                let distance = (actor_pos - player_actor_pos).norm();
                let ship = actor.get_ship();
                let clamp: bool = true;
                let screen_pos: Vector2<f32> = main_camera.convert_world_to_screen(actor_pos, clamp);
                let target_hud_layer = unsafe { self._target_hud_layer.as_mut().unwrap().get_ui_component_mut() };
                target_hud_layer.set_center(screen_pos.x, screen_pos.y);

                let target_distance = unsafe { self._target_distance.as_mut().unwrap().get_ui_component_mut() };
                target_distance.set_text(&format!("{}m", distance as i32));

                self._target_hull_point_widget.as_ref().unwrap().update_hull_point_widget(ship.get_hull_point() / 2.0, ship.get_max_hull_point());
                self._target_shield_point_widget.as_ref().unwrap().update_shield_point_widget(ship.get_shield_point() / 2.0, ship.get_max_shield_point());
                break;
            }
        }
    }
}