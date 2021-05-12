use rust_engine_3d::vulkan_context::vulkan_context::get_color32;
use rust_engine_3d::renderer::ui::{ProjectUIManagerBase, UIManagerData, Widget, UIWidgetTypes};

use crate::renderer::project_ui::ProjectUIManager;
use crate::resource::project_resource::ProjectResources;
use rust_engine_3d::resource::resource::ProjectResourcesBase;

pub struct GameUIManager {
    pub _project_ui_manager: *const ProjectUIManager,
}

impl GameUIManager {
    pub fn create_game_ui_manager() -> Box<GameUIManager> {
        Box::new(GameUIManager {
            _project_ui_manager: std::ptr::null(),
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
        let btn0 = UIManagerData::create_widget("btn0", UIWidgetTypes::Default);
        let ui_component = unsafe { &mut btn0.as_mut().unwrap().get_ui_component_mut() };
        ui_component.set_pos(250.0,255.0);
        ui_component.set_size(200.0, 100.0);
        ui_component.set_color(get_color32(255, 255, 255, 255));
        ui_component.set_font_color(get_color32(0, 0, 0, 255));
        ui_component.set_border_color(get_color32(255, 0, 0, 255));
        ui_component.set_margine(5.0);
        ui_component.set_round(10.0);
        ui_component.set_border(5.0);
        ui_component.set_dragable(true);
        ui_component.set_touchable(true);
        ui_component.set_expandable(true);
        ui_component.set_text(String::from("btn0\nbtn0 Child Test"));
        ui_component.set_material_instance(&project_resources.get_material_instance_data("ui/render_ui_test"));
        ui_component._callback_touch_down = Some(&touch_down);
        ui_component._callback_touch_up = Some(&touch_up);
        ui_component._callback_touch_move = Some(&touch_move);
        root_widget.add_widget(btn0);
    }

    pub fn update_game_ui(&mut self) {

    }
}