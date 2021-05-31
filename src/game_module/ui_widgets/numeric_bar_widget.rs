use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use rust_engine_3d::renderer::ui::{ UIManagerData, Widget, UIWidgetTypes, WidgetDefault, HorizontalAlign, VerticalAlign, UILayoutType };

const HUD_LAYER_WIDTH: f32 = 100.0;
const HUD_LAYER_HEIGHT: f32 = 100.0;
const HUD_LAYER_PADDING: f32 = 10.0;
const HUD_UI_WIDTH: f32 = 100.0;
const HUD_UI_HEIGHT: f32 = 25.0;
const HUD_UI_MARGINE: f32 = 0.0;
const HUD_UI_PADDING: f32 = 4.0;

pub struct HullPointWidget {
    pub _hull_point_layer: *mut WidgetDefault,
    pub _hull_point_bar: *mut WidgetDefault,
}

impl HullPointWidget {
    pub fn create_hull_point_widget(parent_widget: &mut dyn Widget) -> HullPointWidget {
        let parent_ui_component = parent_widget.get_ui_component_mut();

        let hull_point_layer = unsafe { &mut *(UIManagerData::create_widget("hull_point_layer", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = hull_point_layer.get_ui_component_mut();
        ui_component.set_layout_type(UILayoutType::BoxLayout);
        ui_component.set_text("Hull");
        ui_component.set_size(HUD_UI_WIDTH, HUD_UI_HEIGHT);
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(50, 50, 50, 255));
        ui_component.set_font_color(get_color32(255, 255, 255, 255));
        ui_component.set_border_color(get_color32(0, 0, 0, 255));
        ui_component.set_round(5.0);
        ui_component.set_border(2.0);
        ui_component.set_margine(HUD_UI_MARGINE);
        ui_component.set_padding(HUD_UI_PADDING);
        ui_component.set_expandable(true);
        parent_ui_component.add_ui_component(ui_component);

        let hull_point_bar = unsafe { &mut *(UIManagerData::create_widget("hull_point_bar", UIWidgetTypes::Default) as *mut WidgetDefault) };
        let ui_component = hull_point_bar.get_ui_component_mut();
        ui_component.set_size_hint_x(Some(0.5));
        ui_component.set_size_hint_y(Some(1.0));
        ui_component.set_halign(HorizontalAlign::LEFT);
        ui_component.set_valign(VerticalAlign::CENTER);
        ui_component.set_color(get_color32(255, 128, 0, 50));
        ui_component.set_round(1.0);
        hull_point_layer.add_widget(hull_point_bar);

        HullPointWidget {
            _hull_point_layer: hull_point_layer,
            _hull_point_bar: hull_point_bar
        }
    }

    pub fn update_hull_point_widget(&self, hull_point: i32) {
        let hull_point_ui = unsafe { self._hull_point_layer.as_mut().unwrap().get_ui_component_mut() };
        hull_point_ui.set_text(&format!("Hull: {}", hull_point));
    }
}
