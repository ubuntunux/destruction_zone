use std::fs::{ self, File };
use std::io::prelude::*;
use std::path::{ Path, PathBuf };
use std::collections::HashMap;
use byteorder::{ LittleEndian, ReadBytesExt };

use serde_json::{ self, Value, json };
use bincode;

use rust_engine_3d::application::scene_manager::SceneManagerData;
use rust_engine_3d::constants;
use rust_engine_3d::resource::resource::{ ResourceDataMap, ProjectResourcesBase, Resources };
use rust_engine_3d::renderer::renderer::{ RendererData };
use rust_engine_3d::utilities::system::{ self, RcRefCell, newRcRefCell };

use crate::application::project_scene_manager::{ SceneDataCreateInfo };

pub const SCENE_FILE_PATH: &str = "resource/scenes";

pub const EXT_SCENE: &str = "scene";

#[derive(Clone)]
pub struct ProjectResources {
    _engine_resources: *const Resources,
}

impl ProjectResourcesBase for ProjectResources {
    fn initialize_project_resources(&mut self, engine_resources: &Resources, engine_renderer: &mut RendererData) {
        self._engine_resources = engine_resources;
    }

    fn destroy_project_resources(&mut self, engine_renderer: &mut RendererData) {

    }

    fn load_graphics_datas(&mut self, engine_renderer: &mut RendererData) {

    }

    fn unload_graphics_datas(&mut self, engine_renderer: &mut RendererData) {

    }

    fn regist_resource(&mut self) {

    }

    fn unregist_resource(&mut self) {

    }
}

impl ProjectResources {
    pub fn create_project_resources() -> Box<ProjectResources> {
        Box::new(ProjectResources {
            _engine_resources: std::ptr::null(),
        })
    }

    pub fn get_engine_resources(&self) -> &Resources {
        unsafe { &*self._engine_resources }
    }

    pub fn get_engine_resources_mut(&self) -> &mut Resources {
        unsafe { &mut *(self._engine_resources as *mut Resources) }
    }

    pub fn load_scene_manager_datas(&mut self, _renderer_data: &RendererData) {

    }

    pub fn unload_scene_manager_datas(&mut self, _renderer_data: &RendererData) {

    }
}
