use nalgebra::Vector2;

use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use rust_engine_3d::renderer::ui::{ProjectUIManagerBase, UIManagerData, Widget, UIWidgetTypes, WidgetDefault, HorizontalAlign, VerticalAlign};
use rust_engine_3d::resource::resource::ProjectResourcesBase;

use crate::application::project_application::Application;
use crate::renderer::project_ui::ProjectUIManager;
use crate::resource::project_resource::ProjectResources;

pub struct GameUIManager {
    pub _project_ui_manager: *const ProjectUIManager,
    pub _crosshair_widget: *const WidgetDefault,
    pub _crosshair_pos: Vector2<f32>,
}

impl GameUIManager {
    pub fn create_game_ui_manager() -> Box<GameUIManager> {
        Box::new(GameUIManager {
            _project_ui_manager: std::ptr::null(),
            _crosshair_widget: std::ptr::null(),
            _crosshair_pos: Vector2::zeros(),
        })
    }

    pub fn get_project_ui_manager(&self) -> &ProjectUIManager {
        unsafe { &*self._project_ui_manager }
    }

    pub fn get_project_ui_manager_mut(&self) -> &mut ProjectUIManager {
        unsafe { &mut *(self._project_ui_manager as *mut ProjectUIManager) }
    }

    pub fn initialize_game_ui_manager(&mut self, project_ui_manager: &ProjectUIManager, project_resources: &ProjectResources) {
        self._project_ui_manager = project_ui_manager;

        let project_ui_manager = self.get_project_ui_manager_mut();
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

        //
        let crosshair_widget: *mut WidgetDefault = UIManagerData::create_widget("cursor", UIWidgetTypes::Default) as *mut WidgetDefault;
        let ui_component = unsafe { &mut crosshair_widget.as_mut().unwrap().get_ui_component_mut() };
        ui_component.set_pos(250.0,255.0);
        ui_component.set_size(100.0, 100.0);
        ui_component.set_material_instance(&project_resources.get_material_instance_data("ui/crosshair"));
        ui_component._callback_touch_down = Some(&touch_down);
        ui_component._callback_touch_up = Some(&touch_up);
        ui_component._callback_touch_move = Some(&touch_move);
        root_widget.add_widget(crosshair_widget);
        self._crosshair_widget = crosshair_widget;
    }

    pub fn update_game_ui(&mut self, project_application: &Application, delta_time: f32) {
        let window_size = &project_application.get_application_data()._window_size;
        self._crosshair_pos.x = window_size.x as f32 * 0.5;
        self._crosshair_pos.y = window_size.y as f32 * 0.5;
        let crosshair_widget = unsafe { &mut *(self._crosshair_widget as *mut WidgetDefault) };
        let ui_component = crosshair_widget.get_ui_component_mut();
        ui_component.set_pos_x(self._crosshair_pos.x as f32 - ui_component.get_size_x() * 0.5);
        ui_component.set_pos_y(self._crosshair_pos.y as f32 - ui_component.get_size_y() * 0.5);
    }
}