use nalgebra::Vector2;
use rust_engine_3d::renderer::ui::WidgetDefault;
use crate::game_module::ui_widgets::hit_point_widgets::{ShieldPointWidget, HullPointWidget};

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