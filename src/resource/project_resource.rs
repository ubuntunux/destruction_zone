use std::fs::{ File };
use std::io::{Cursor, BufReader};
use std::io::prelude::*;
use std::path::{ Path, PathBuf };

use serde_json::{ self };
use rodio;

use rust_engine_3d::resource::resource::{ ResourceDataMap, ProjectResourcesBase, Resources, get_unique_resource_name };
use rust_engine_3d::renderer::renderer::{ RendererData };
use rust_engine_3d::utilities::system::{ self, RcRefCell, newRcRefCell };
use rust_engine_3d::renderer::effect::EffectData;
use rust_engine_3d::renderer::font::FontData;
use rust_engine_3d::renderer::model::ModelData;
use rust_engine_3d::renderer::mesh::MeshData;
use rust_engine_3d::vulkan_context::texture::TextureData;
use rust_engine_3d::renderer::material::MaterialData;
use rust_engine_3d::renderer::material_instance::MaterialInstanceData;
use crate::application::project_scene_manager::SceneDataCreateInfo;
use crate::application::project_audio_manager::AudioDataCreateInfo;

pub const SCENE_FILE_PATH: &str = "resource/scenes";
pub const AUDIO_FILE_PATH: &str = "resource/sounds";

pub const EXT_SCENE: &str = "scene";
pub const AUDIO_SOURCE_EXTS: [&str; 2] = ["wav", "mp3"];

pub type SceneDataCreateInfoMap = ResourceDataMap<SceneDataCreateInfo>;
pub type AudioDataCreateInfoMap = ResourceDataMap<AudioDataCreateInfo>;

#[derive(Clone)]
pub struct ProjectResources {
    _engine_resources: *const Resources,
    _scene_data_create_infos_map: SceneDataCreateInfoMap,
    _audio_data_create_infos_map: AudioDataCreateInfoMap,
}

impl ProjectResourcesBase for ProjectResources {
    fn initialize_project_resources(&mut self, engine_resources: &Resources, engine_renderer: &mut RendererData) {
        self._engine_resources = engine_resources;
        self.load_audio_datas();
        self.load_scene_datas(engine_renderer);
    }
    fn destroy_project_resources(&mut self, engine_renderer: &mut RendererData) {
        self.unload_scene_datas(engine_renderer);
        self.unload_audio_datas();
    }
    fn load_graphics_datas(&mut self, _engine_renderer: &mut RendererData) {
    }
    fn unload_graphics_datas(&mut self, _engine_renderer: &mut RendererData) {
    }
    fn regist_resource(&mut self) {
    }
    fn unregist_resource(&mut self) {
    }
    fn has_effect_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_effect_data(resource_name)
    }
    fn get_effect_data(&self, resource_name: &str) -> &RcRefCell<EffectData> {
        self.get_engine_resources().get_effect_data(resource_name)
    }
    fn get_default_font_data(&self) -> &RcRefCell<FontData> {
        self.get_engine_resources().get_default_font_data()
    }
    fn get_font_data(&self, resource_name: &str) -> &RcRefCell<FontData> {
        self.get_engine_resources().get_font_data(resource_name)
    }
    fn has_model_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_model_data(resource_name)
    }
    fn get_model_data(&self, resource_name: &str) -> &RcRefCell<ModelData> {
        self.get_engine_resources().get_model_data(resource_name)
    }
    fn has_mesh_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_mesh_data(resource_name)
    }
    fn get_mesh_data(&self, resource_name: &str) -> &RcRefCell<MeshData> {
        self.get_engine_resources().get_mesh_data(resource_name)
    }
    fn has_texture_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_texture_data(resource_name)
    }
    fn get_texture_data(&self, resource_name: &str) -> &RcRefCell<TextureData> {
        self.get_engine_resources().get_texture_data(resource_name)
    }
    fn has_material_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_material_data(resource_name)
    }
    fn get_material_data(&self, resource_name: &str) -> &RcRefCell<MaterialData> {
        self.get_engine_resources().get_material_data(resource_name)
    }
    fn has_material_instance_data(&self, resource_name: &str) -> bool {
        self.get_engine_resources().has_material_instance_data(resource_name)
    }
    fn get_material_instance_data(&self, resource_name: &str) -> &RcRefCell<MaterialInstanceData> {
        self.get_engine_resources().get_material_instance_data(resource_name)
    }
}

impl ProjectResources {
    pub fn create_project_resources() -> Box<ProjectResources> {
        Box::new(ProjectResources {
            _engine_resources: std::ptr::null(),
            _scene_data_create_infos_map: SceneDataCreateInfoMap::new(),
            _audio_data_create_infos_map: AudioDataCreateInfoMap::new(),
        })
    }
    pub fn get_engine_resources(&self) -> &Resources {
        unsafe { &*self._engine_resources }
    }
    pub fn get_engine_resources_mut(&self) -> &mut Resources {
        unsafe { &mut *(self._engine_resources as *mut Resources) }
    }
    pub fn collect_resources(&self, dir: &Path, extensions: &[&str]) -> Vec<PathBuf> {
        self.get_engine_resources().collect_resources(dir, extensions)
    }

    // SceneData
    pub fn load_scene_datas(&mut self, _renderer_data: &RendererData) {
        let scene_directory = PathBuf::from(SCENE_FILE_PATH);
        let scene_data_files: Vec<PathBuf> = self.collect_resources(&scene_directory, &[EXT_SCENE]);
        for scene_data_file in scene_data_files {
            let scene_data_name = get_unique_resource_name(&self._scene_data_create_infos_map, &scene_directory, &scene_data_file);
            let loaded_contents = system::load(&scene_data_file);
            let scene_data_create_info: SceneDataCreateInfo = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._scene_data_create_infos_map.insert(scene_data_name.clone(), newRcRefCell(scene_data_create_info));
        }
    }

    pub fn unload_scene_datas(&mut self, _renderer_data: &RendererData) {
        self._scene_data_create_infos_map.clear();
    }

    pub fn save_scene_data(&mut self, scene_data_name: &str, scene_data_create_info: &SceneDataCreateInfo) {
        let mut scene_data_filepath = PathBuf::from(SCENE_FILE_PATH);
        scene_data_filepath.push(scene_data_name);
        scene_data_filepath.set_extension(EXT_SCENE);
        let mut write_file = File::create(&scene_data_filepath).expect("Failed to create file");
        let mut write_contents: String = serde_json::to_string(&scene_data_create_info).expect("Failed to serialize.");
        write_contents = write_contents.replace(",\"", ",\n\"");
        write_file.write(write_contents.as_bytes()).expect("Failed to write");

        self._scene_data_create_infos_map.insert(String::from(scene_data_name), newRcRefCell(scene_data_create_info.clone()));
    }

    pub fn has_scene_data(&self, resource_name: &str) -> bool {
        self._scene_data_create_infos_map.get(resource_name).is_some()
    }

    pub fn get_scene_data(&self, resource_name: &str) -> &RcRefCell<SceneDataCreateInfo> {
        self._scene_data_create_infos_map.get(resource_name).unwrap()
    }

    // Audio Data
    pub fn load_audio_datas(&mut self) {
        let audio_directory = PathBuf::from(AUDIO_FILE_PATH);
        let audio_data_files: Vec<PathBuf> = self.collect_resources(&audio_directory, &AUDIO_SOURCE_EXTS);
        for audio_data_file in audio_data_files {
            let audio_data_name = get_unique_resource_name(&self._audio_data_create_infos_map, &audio_directory, &audio_data_file);
            let loaded_contents = system::load(&audio_data_file);
            let audio_source = rodio::Decoder::new(BufReader::new(loaded_contents)).unwrap();
            let audio_data_create_info = AudioDataCreateInfo {
                _audio_name: audio_data_name.clone(),
                _audio_source: audio_source,
            };
            self._audio_data_create_infos_map.insert(audio_data_name.clone(), newRcRefCell(audio_data_create_info));
        }
    }

    pub fn unload_audio_datas(&mut self) {
        self._audio_data_create_infos_map.clear();
    }

    pub fn has_audio_data(&self, resource_name: &str) -> bool {
        self._audio_data_create_infos_map.get(resource_name).is_some()
    }

    pub fn get_audio_data(&self, resource_name: &str) -> &RcRefCell<AudioDataCreateInfo> {
        self._audio_data_create_infos_map.get(resource_name).unwrap()
    }
}
