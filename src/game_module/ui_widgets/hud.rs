use nalgebra::Vector2;

use rust_engine_3d::renderer::ui::*;
use rust_engine_3d::resource::resource::ProjectResourcesBase;
use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use crate::game_module::ui_widgets::hit_point_widgets::{ShieldPointWidget, HullPointWidget};
use crate::resource::project_resource::ProjectResources;


pub struct TargetHud {
    pub _widget: *mut WidgetDefault,
    pub _distance: *mut WidgetDefault,
    pub _hull_point_widget: Option<HullPointWidget>,
    pub _shield_point_widget: Option<ShieldPointWidget>,
}

pub struct PlayerHud {
    pub _widget: *mut WidgetDefault,
    pub _hull_point_widget: Option<HullPointWidget>,
    pub _shield_point_widget: Option<ShieldPointWidget>,
}

pub struct CrossHair {
    pub _widget: *const WidgetDefault,
    pub _pos: Vector2<i32>,
    pub _tracking_mouse: bool,
}

pub struct SelectionArea {
    pub _widget: *const WidgetDefault,
    pub _start_pos: Vector2<i32>,
    pub _drag_mouse: bool,
}

// implement //

// CrossHair
impl Default for CrossHair {
    fn default() -> CrossHair {
        CrossHair {
            _widget: std::ptr::null(),
            _pos: Vector2::zeros(),
            _tracking_mouse: true,
        }
    }
}

impl CrossHair {
    pub fn initialize_crosshair(&mut self, project_resources: &ProjectResources, root_widget: &mut dyn Widget, window_center: &Vector2<f32>) {
        let crosshair_widget: *mut WidgetDefault = UIManager::create_widget("cursor", UIWidgetTypes::Default) as *mut WidgetDefault;
        let ui_component = unsafe { &mut crosshair_widget.as_mut().unwrap().get_ui_component_mut() };
        let ui_size = 50.0f32;
        ui_component.set_pos(window_center.x - ui_size * 0.5, window_center.y - ui_size * 0.5);
        ui_component.set_size(ui_size, ui_size);
        ui_component.set_material_instance(&project_resources.get_material_instance_data("ui/crosshair"));
        root_widget.add_widget(crosshair_widget);
        self._widget = crosshair_widget;
    }
}

// TargetHud
impl Default for TargetHud {
    fn default() -> TargetHud {
        TargetHud {
            _widget: std::ptr::null_mut(),
            _distance: std::ptr::null_mut(),
            _hull_point_widget: None,
            _shield_point_widget: None,
        }
    }
}

impl TargetHud {
    pub fn initlialize_target_hud(&mut self, root_widget: &mut dyn Widget, center: &Vector2<f32>) {
        let hud_layer_width: f32 = 100.0;
        let hud_layer_height: f32 = 100.0;
        let hud_layer_padding: f32 = 10.0;
        let hud_ui_width: f32 = 100.0;
        let hud_ui_height: f32 = 25.0;
        let hud_ui_margine: f32 = 2.0;
        let hud_ui_padding: f32 = 4.0;

        let target_widget = unsafe { &mut *(UIManager::create_widget("target_widget", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = target_widget.get_ui_component_mut();
        ui_component.set_size(hud_layer_width, hud_layer_height);
        ui_component.set_center(center.x, center.y);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_padding(hud_layer_padding);
        ui_component.set_color(get_color32(255, 255, 255, 10));
        ui_component.set_opacity(0.5);
        root_widget.add_widget(target_widget);
        self._widget = target_widget;

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
        target_widget.add_widget(target_distance);
        self._distance = target_distance;
        self._hull_point_widget = Some(HullPointWidget::create_hull_point_widget(target_widget));
        self._shield_point_widget = Some(ShieldPointWidget::create_shield_point_widget(target_widget));
    }
}

// PlayerHud
impl Default for PlayerHud {
    fn default() -> PlayerHud {
        PlayerHud {
            _widget: std::ptr::null_mut(),
            _hull_point_widget: None,
            _shield_point_widget: None,
        }
    }
}

impl PlayerHud {
    pub fn initialize_player_hud(&mut self, root_widget: &mut dyn Widget, pos: &Vector2<f32>) {
        let hud_layer_width: f32 = 100.0;
        let hud_layer_height: f32 = 100.0;
        let hud_layer_padding: f32 = 10.0;

        let player_widget = unsafe { &mut *(UIManager::create_widget("player_widget", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = player_widget.get_ui_component_mut();
        ui_component.set_size(hud_layer_width, hud_layer_height);
        ui_component.set_pos(pos.x, pos.y);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_layout_orientation(Orientation::VERTICAL);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_expandable(true);
        ui_component.set_padding(hud_layer_padding);
        ui_component.set_color(get_color32(255, 255, 255, 10));
        root_widget.add_widget(player_widget);
        self._widget = player_widget;
        self._hull_point_widget = Some(HullPointWidget::create_hull_point_widget(player_widget));
        self._shield_point_widget = Some(ShieldPointWidget::create_shield_point_widget(player_widget));
    }
}


// Selection Area
impl Default for SelectionArea {
    fn default() -> SelectionArea {
        SelectionArea {
            _widget: std::ptr::null_mut(),
            _start_pos: Vector2::zeros(),
            _drag_mouse: false,
        }
    }
}

impl SelectionArea {
    pub fn initialize_selection_area(&mut self, root_widget: &mut dyn Widget) {
        let widget = unsafe { &mut *(UIManager::create_widget("selection_area_widget", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = widget.get_ui_component_mut();
        ui_component.set_size(300.0, 300.0);
        ui_component.set_pos(100.0, 100.0);
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_halign(HorizontalAlign::CENTER);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(255, 255, 0, 128));
        ui_component.set_border_color(get_color32(255, 255, 0, 255));
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_resizable(true);
        ui_component.set_touchable(true);

        static TOUCH_DOWN: CallbackTouchEvent = SelectionArea::touch_down;
        static TOUCH_MOVE: CallbackTouchEvent = SelectionArea::touch_move;
        static TOUCH_UP: CallbackTouchEvent = SelectionArea::touch_up;

        ui_component.set_callback_touch_down(&TOUCH_DOWN);
        ui_component.set_callback_touch_move(&TOUCH_MOVE);
        ui_component.set_callback_touch_up(&TOUCH_UP);
        ui_component.set_visible(false);
        root_widget.add_widget(widget);
        self._widget = widget;
    }

    pub fn touch_down(ui_component: &mut UIComponentInstance, touched_pos: &Vector2<f32>, touched_pos_delta: &Vector2<f32>) {
        ui_component.set_visible(true);
    }

    pub fn touch_move(ui_component: &mut UIComponentInstance, touched_pos: &Vector2<f32>, touched_pos_delta: &Vector2<f32>) {
        let size: Vector2<f32> = ui_component.get_touch_start_pos() - touched_pos;
        ui_component.set_pos_x(ui_component.get_touch_start_pos().x - 0f32.max(size.x));
        ui_component.set_pos_y(ui_component.get_touch_start_pos().y - 0f32.max(size.y));
        ui_component.set_size(size.x.abs(), size.y.abs());
    }

    pub fn touch_up(ui_component: &mut UIComponentInstance, touched_pos: &Vector2<f32>, touched_pos_delta: &Vector2<f32>) {
        ui_component.set_visible(false);
    }
}