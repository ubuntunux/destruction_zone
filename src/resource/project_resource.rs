use std::fs::{ File };
use std::io::prelude::*;
use std::path::{ Path, PathBuf };

use serde_json::{ self };

use rust_engine_3d::resource::resource::{DEFAULT_MESH_NAME, ResourceDataMap, ProjectResourcesBase, Resources, get_unique_resource_name, get_resource_data};
use rust_engine_3d::renderer::renderer::{ RendererData };
use rust_engine_3d::utilities::system::{ self, RcRefCell, newRcRefCell };
use rust_engine_3d::renderer::font::FontData;
use rust_engine_3d::renderer::model::ModelData;
use rust_engine_3d::renderer::mesh::MeshData;
use rust_engine_3d::vulkan_context::texture::TextureData;
use rust_engine_3d::renderer::material::MaterialData;
use rust_engine_3d::renderer::material_instance::MaterialInstanceData;
use crate::application::project_scene_manager::SceneDataCreateInfo;
use crate::application::project_audio_manager::{AudioData, AudioBankData, AudioBankCreateInfo};
use crate::effect::effect_data::{EffectData, EffectDataCreateInfo, EmitterDataCreateInfo, EmitterData};
use crate::game_module::ship::ship::{ShipDataCreateInfo, ShipData};
use crate::game_module::ship::ship_controller::ShipControllerData;
use crate::game_module::weapons::bullet::BulletData;
use crate::game_module::weapons::weapon::{WeaponDataCreateInfo, WeaponData};

pub const SCENE_FILE_PATH: &str = "resources/scenes";
pub const AUDIO_FILE_PATH: &str = "resources/sounds";
pub const AUDIO_BANK_FILE_PATH: &str = "resources/sound_banks";
pub const EFFECT_FILE_PATH: &str = "resources/effects";
pub const BUILDING_DATA_FILE_PATH: &str = "resources/game_datas/buildings";
pub const BULLET_DATA_FILE_PATH: &str = "resources/game_datas/bullets";
pub const SHIP_CONTROLLER_DATA_FILE_PATH: &str = "resources/game_datas/ship_controllers";
pub const SHIP_DATA_FILE_PATH: &str = "resources/game_datas/ships";
pub const WEAPON_DATA_FILE_PATH: &str = "resources/game_datas/weapons";

pub const EXT_SCENE: &str = "scene";
pub const AUDIO_SOURCE_EXTS: [&str; 2] = ["wav", "mp3"];
pub const EXT_AUDIO_BANK: &str = "bank";
pub const EXT_EFFECT: &str = "effect";
pub const EXT_GAME_DATA: &str = "data";

pub const DEFAULT_EFFECT_NAME: &str = "default";
pub const DEFAULT_EFFECT_MATERIAL_INSTANCE_NAME: &str = "common/render_particle";
pub const DEFAULT_GAME_DATA_NAME: &str = "default";
pub const DEFAULT_AUDIO_NAME: &str = "default";
pub const DEFAULT_AUDIO_BANK_NAME: &str = "default";

pub type SceneDataCreateInfoMap = ResourceDataMap<SceneDataCreateInfo>;
pub type AudioDataMap = ResourceDataMap<AudioData>;
pub type AudioBankDataMap = ResourceDataMap<AudioBankData>;
pub type BuildingDataMap = ResourceDataMap<bool>;
pub type BulletDataMap = ResourceDataMap<BulletData>;
pub type EffectDataMap = ResourceDataMap<EffectData>;
pub type ShipDataMap = ResourceDataMap<ShipData>;
pub type ShipControllerDataMap = ResourceDataMap<ShipControllerData>;
pub type WeaponDataMap = ResourceDataMap<WeaponData>;

#[derive(Clone)]
pub struct ProjectResources {
    _engine_resources: *const Resources,
    _scene_data_create_infos_map: SceneDataCreateInfoMap,
    _audio_data_map: AudioDataMap,
    _audio_bank_data_map: AudioBankDataMap,
    _effect_data_map: EffectDataMap,
    _building_data_map: BuildingDataMap,
    _bullet_data_map: BulletDataMap,
    _ship_data_map: ShipDataMap,
    _ship_controller_data_map: ShipControllerDataMap,
    _weapon_data_map: WeaponDataMap,
}

impl ProjectResourcesBase for ProjectResources {
    fn initialize_project_resources(&mut self, engine_resources: &Resources, engine_renderer: &mut RendererData) {
        self._engine_resources = engine_resources;
        self.load_audio_datas();
        self.load_effect_datas(engine_renderer);
        self.load_scene_datas(engine_renderer);
        self.load_game_datas();
    }
    fn destroy_project_resources(&mut self, engine_renderer: &mut RendererData) {
        self.unload_game_datas();
        self.unload_scene_datas(engine_renderer);
        self.load_effect_datas(engine_renderer);
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
            _audio_data_map: AudioDataMap::new(),
            _audio_bank_data_map: AudioBankDataMap::new(),
            _effect_data_map: EffectDataMap::new(),
            _building_data_map: Default::default(),
            _bullet_data_map: Default::default(),
            _ship_data_map: Default::default(),
            _ship_controller_data_map: Default::default(),
            _weapon_data_map: Default::default()
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
        let audio_bank_directory = PathBuf::from(AUDIO_BANK_FILE_PATH);

        // load audio datas
        let audio_data_files: Vec<PathBuf> = self.collect_resources(&audio_directory, &AUDIO_SOURCE_EXTS);
        for audio_data_file in audio_data_files {
            let audio_data_name = get_unique_resource_name(&self._audio_data_map, &audio_directory, &audio_data_file);
            // let loaded_contents = system::load(&audio_data_file);
            let sound_chunk = sdl2::mixer::Chunk::from_file(audio_data_file).unwrap();
            let audio_data_create_info = AudioData {
                _audio_name: audio_data_name.clone(),
                _sound_chunk: sound_chunk,
            };
            self._audio_data_map.insert(audio_data_name.clone(), newRcRefCell(audio_data_create_info));
        }

        // default audio bank data
        let mut default_audio_bank_file_path: PathBuf = audio_bank_directory.clone();
        default_audio_bank_file_path.push(&DEFAULT_AUDIO_BANK_NAME);
        default_audio_bank_file_path.set_extension(EXT_AUDIO_BANK);
        #[cfg(not(target_os = "android"))]
        if false == default_audio_bank_file_path.is_file() {
            let default_audio_bank_data_create_info = AudioBankCreateInfo::default();
            let mut write_file = File::create(&default_audio_bank_file_path).expect("Failed to create file");
            let mut write_contents: String = serde_json::to_string(&default_audio_bank_data_create_info).expect("Failed to serialize.");
            write_contents = write_contents.replace(",\"", ",\n\"");
            write_file.write(write_contents.as_bytes()).expect("Failed to write");
        }

        // load audio bank datas
        let audio_bank_data_files: Vec<PathBuf> = self.collect_resources(&audio_bank_directory, &[EXT_AUDIO_BANK]);
        for audio_bank_data_file in audio_bank_data_files {
            let audio_bank_data_name = get_unique_resource_name(&self._audio_bank_data_map, &audio_bank_directory, &audio_bank_data_file);
            let loaded_contents = system::load(&audio_bank_data_file);
            let audio_bank_create_info: AudioBankCreateInfo = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            let audio_datas = audio_bank_create_info._audio_names.iter()
                .filter(|audio_name| self.has_audio_data(audio_name))
                .map(|audio_name| self.get_audio_data(audio_name).unwrap().clone())
                .collect();
            let audio_bank_data = AudioBankData {
                _audio_bank_name: audio_bank_data_name.clone(),
                _audios_datas: audio_datas,
            };
            self._audio_bank_data_map.insert(audio_bank_data_name.clone(), newRcRefCell(audio_bank_data));
        }
    }

    pub fn unload_audio_datas(&mut self) {
        self._audio_bank_data_map.clear();
        self._audio_data_map.clear();
    }

    pub fn has_audio_data(&self, resource_name: &str) -> bool {
        self._audio_data_map.get(resource_name).is_some()
    }

    pub fn get_audio_data(&self, resource_name: &str) -> Option<&RcRefCell<AudioData>> {
        self._audio_data_map.get(resource_name)
    }

    pub fn has_audio_bank_data(&self, resource_name: &str) -> bool {
        self._audio_bank_data_map.get(resource_name).is_some()
    }

    pub fn get_audio_bank_data(&self, resource_name: &str) -> Option<&RcRefCell<AudioBankData>> {
        self._audio_bank_data_map.get(resource_name)
    }

    // EffectData
    pub fn load_effect_datas(&mut self, _renderer_data: &RendererData) {
        let effect_directory = PathBuf::from(EFFECT_FILE_PATH);

        // create default effect
        let mut default_effect_file_path: PathBuf = effect_directory.clone();
        default_effect_file_path.push(&DEFAULT_EFFECT_NAME);
        default_effect_file_path.set_extension(EXT_EFFECT);
        #[cfg(not(target_os = "android"))]
        if false == default_effect_file_path.is_file() {
            let default_effect_data_create_info = EffectDataCreateInfo {
                _emitter_data_create_infos: vec![EmitterDataCreateInfo {
                    _enable: true,
                    _emitter_data_name: String::from("emitter"),
                    _emitter_lifetime: -1.0,
                    _material_instance_name: String::from(DEFAULT_EFFECT_MATERIAL_INSTANCE_NAME),
                    _mesh_name: String::from(DEFAULT_MESH_NAME),
                    ..Default::default()
                }],
                ..Default::default()
            };
            let mut write_file = File::create(&default_effect_file_path).expect("Failed to create file");
            let mut write_contents: String = serde_json::to_string(&default_effect_data_create_info).expect("Failed to serialize.");
            write_contents = write_contents.replace(",\"", ",\n\"");
            write_file.write(write_contents.as_bytes()).expect("Failed to write");
        }

        let effect_files: Vec<PathBuf> = self.collect_resources(&effect_directory, &[EXT_EFFECT]);
        for effect_file in effect_files {
            let effect_data_name = get_unique_resource_name(&self._effect_data_map, &effect_directory, &effect_file);
            let loaded_contents = system::load(&effect_file);
            let effect_data_create_info: EffectDataCreateInfo = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            let emitter_datas = effect_data_create_info._emitter_data_create_infos.iter().map(|emitter_data_create_info| {
                let material_instance_data = self.get_material_instance_data(&emitter_data_create_info._material_instance_name);
                let mesh_data = self.get_mesh_data(&emitter_data_create_info._mesh_name);
                EmitterData::create_emitter_data(
                    emitter_data_create_info,
                    material_instance_data.clone(),
                    mesh_data.clone()
                )
            }).collect();
            let effect_data = EffectData::create_effect_data(
                &effect_data_name,
                &effect_data_create_info,
                emitter_datas
            );
            self._effect_data_map.insert(effect_data_name.clone(), newRcRefCell(effect_data));
        }
    }

    pub fn unload_effect_datas(&mut self, _renderer_data: &RendererData) {
        for effect_data in self._effect_data_map.values() {
            effect_data.borrow_mut().destroy_effect_data();
        }
        self._effect_data_map.clear();
    }

    pub fn has_effect_data(&self, resource_name: &str) -> bool {
        self._effect_data_map.contains_key(resource_name)
    }

    pub fn get_effect_data(&self, resource_name: &str) -> &RcRefCell<EffectData> {
        get_resource_data(&self._effect_data_map, resource_name, DEFAULT_EFFECT_NAME)
    }

    // Game Datas
    fn load_game_datas(&mut self) {
        self.load_bullet_datas();
        self.load_weapon_datas();
        self.load_ship_controller_datas();
        self.load_ship_datas();
    }

    fn unload_game_datas(&mut self) {
        self.unload_bullet_datas();
        self.unload_weapon_datas();
        self.unload_ship_datas();
        self.unload_ship_controller_datas();
    }

    // ship controller data
    fn load_ship_controller_datas(&mut self) {
        let game_data_directory = PathBuf::from(SHIP_CONTROLLER_DATA_FILE_PATH);

        // create ship contorller data
        let mut default_ship_controller_data_file_path: PathBuf = game_data_directory.clone();
        default_ship_controller_data_file_path.push(&DEFAULT_GAME_DATA_NAME);
        default_ship_controller_data_file_path.set_extension(EXT_GAME_DATA);
        #[cfg(not(target_os = "android"))]
        if false == default_ship_controller_data_file_path.is_file() {
            let default_ship_controller_data_create_info = ShipControllerData::default();
            let mut write_file = File::create(&default_ship_controller_data_file_path).expect("Failed to create file");
            let mut write_contents: String = serde_json::to_string(&default_ship_controller_data_create_info).expect("Failed to serialize.");
            write_contents = write_contents.replace(",\"", ",\n\"");
            write_file.write(write_contents.as_bytes()).expect("Failed to write");
        }

        // load_ship_controller_datas
        let game_data_files: Vec<PathBuf> = self.collect_resources(&game_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let game_data_name = get_unique_resource_name(&self._ship_controller_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let ship_controller_data: ShipControllerData = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._ship_controller_data_map.insert(game_data_name.clone(), newRcRefCell(ship_controller_data));
        }
    }

    fn unload_ship_controller_datas(&mut self) {
        self._ship_controller_data_map.clear();
    }

    pub fn has_ship_controller_data(&self, resource_name: &str) -> bool {
        self._ship_controller_data_map.get(resource_name).is_some()
    }

    pub fn get_ship_controller_data(&self, resource_name: &str) -> &RcRefCell<ShipControllerData> {
        self._ship_controller_data_map.get(resource_name).unwrap()
    }

    // ship data
    fn load_ship_datas(&mut self) {
        let game_data_directory = PathBuf::from(SHIP_DATA_FILE_PATH);

        // create ship data
        let mut default_ship_data_file_path: PathBuf = game_data_directory.clone();
        default_ship_data_file_path.push(&DEFAULT_GAME_DATA_NAME);
        default_ship_data_file_path.set_extension(EXT_GAME_DATA);
        #[cfg(not(target_os = "android"))]
        if false == default_ship_data_file_path.is_file() {
            let default_ship_data_create_info = ShipDataCreateInfo {
                _controller_data_name: DEFAULT_GAME_DATA_NAME.to_string(),
                ..Default::default()
            };
            let mut write_file = File::create(&default_ship_data_file_path).expect("Failed to create file");
            let mut write_contents: String = serde_json::to_string(&default_ship_data_create_info).expect("Failed to serialize.");
            write_contents = write_contents.replace(",\"", ",\n\"");
            write_file.write(write_contents.as_bytes()).expect("Failed to write");
        }

        // load ship data
        let game_data_files: Vec<PathBuf> = self.collect_resources(&game_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let game_data_name = get_unique_resource_name(&self._ship_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let ship_data_create_info: ShipDataCreateInfo = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            let ship_controller_data = self.get_ship_controller_data(&ship_data_create_info._controller_data_name);
            let ship_data = ShipData::create_ship_data(&game_data_name, &ship_data_create_info, ship_controller_data);
            self._ship_data_map.insert(game_data_name.clone(), ship_data);
        }
    }

    fn unload_ship_datas(&mut self) {
        self._ship_data_map.clear();
    }

    pub fn has_ship_data(&self, resource_name: &str) -> bool {
        self._ship_data_map.get(resource_name).is_some()
    }

    pub fn get_ship_data(&self, resource_name: &str) -> &RcRefCell<ShipData> {
        self._ship_data_map.get(resource_name).unwrap()
    }

    // bullet data
    fn load_bullet_datas(&mut self) {
        let game_data_directory = PathBuf::from(BULLET_DATA_FILE_PATH);

        // create bullet data
        let mut default_bullet_data_file_path: PathBuf = game_data_directory.clone();
        default_bullet_data_file_path.push(&DEFAULT_GAME_DATA_NAME);
        default_bullet_data_file_path.set_extension(EXT_GAME_DATA);
        #[cfg(not(target_os = "android"))]
        if false == default_bullet_data_file_path.is_file() {
            let default_bullet_data_create_info = BulletData::default();
            let mut write_file = File::create(&default_bullet_data_file_path).expect("Failed to create file");
            let mut write_contents: String = serde_json::to_string(&default_bullet_data_create_info).expect("Failed to serialize.");
            write_contents = write_contents.replace(",\"", ",\n\"");
            write_file.write(write_contents.as_bytes()).expect("Failed to write");
        }

        // load bullet data
        let game_data_files: Vec<PathBuf> = self.collect_resources(&game_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let game_data_name = get_unique_resource_name(&self._bullet_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let bullet_data: BulletData = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            self._bullet_data_map.insert(game_data_name.clone(), newRcRefCell(bullet_data));
        }
    }

    fn unload_bullet_datas(&mut self) {
        self._bullet_data_map.clear();
    }

    pub fn has_bullet_data(&self, resource_name: &str) -> bool {
        self._bullet_data_map.get(resource_name).is_some()
    }

    pub fn get_bullet_data(&self, resource_name: &str) -> &RcRefCell<BulletData> {
        self._bullet_data_map.get(resource_name).unwrap()
    }

    // weapon data
    fn load_weapon_datas(&mut self) {
        let game_data_directory = PathBuf::from(WEAPON_DATA_FILE_PATH);

        // create weapon data
        let mut default_weapon_data_file_path: PathBuf = game_data_directory.clone();
        default_weapon_data_file_path.push(&DEFAULT_GAME_DATA_NAME);
        default_weapon_data_file_path.set_extension(EXT_GAME_DATA);
        #[cfg(not(target_os = "android"))]
        if false == default_weapon_data_file_path.is_file() {
            let default_weapon_data_create_info = WeaponDataCreateInfo {
                _bullet_data_name: DEFAULT_GAME_DATA_NAME.to_string(),
                ..Default::default()
            };
            let mut write_file = File::create(&default_weapon_data_file_path).expect("Failed to create file");
            let mut write_contents: String = serde_json::to_string(&default_weapon_data_create_info).expect("Failed to serialize.");
            write_contents = write_contents.replace(",\"", ",\n\"");
            write_file.write(write_contents.as_bytes()).expect("Failed to write");
        }

        // load weapon data
        let game_data_files: Vec<PathBuf> = self.collect_resources(&game_data_directory, &[EXT_GAME_DATA]);
        for game_data_file in game_data_files {
            let game_data_name = get_unique_resource_name(&self._weapon_data_map, &game_data_directory, &game_data_file);
            let loaded_contents = system::load(&game_data_file);
            let weapon_data_create_info: WeaponDataCreateInfo = serde_json::from_reader(loaded_contents).expect("Failed to deserialize.");
            let bullet_data = self.get_bullet_data(&weapon_data_create_info._bullet_data_name);
            let weapon_data = WeaponData::create_weapon_data(&game_data_name, &weapon_data_create_info, &bullet_data);
            self._weapon_data_map.insert(game_data_name.clone(), weapon_data);
        }
    }

    fn unload_weapon_datas(&mut self) {
        self._weapon_data_map.clear();
    }

    pub fn has_weapon_data(&self, resource_name: &str) -> bool {
        self._weapon_data_map.get(resource_name).is_some()
    }

    pub fn get_weapon_data(&self, resource_name: &str) -> &RcRefCell<WeaponData> {
        self._weapon_data_map.get(resource_name).unwrap()
    }
}
