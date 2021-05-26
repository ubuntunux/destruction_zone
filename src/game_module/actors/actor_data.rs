use rust_engine_3d::renderer::camera::CameraObjectData;
use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::actor_controller::actor_controller::{ ControllerDataType, ActorController };
use crate::game_module::actor_manager::calc_floating_height;
use crate::game_module::armor::armor::{ ArmorInstance, ArmorDataType };
use crate::game_module::weapons::weapon::{ WeaponTrait, BeamEmitter };
use crate::game_module::height_map_data::HeightMapData;

pub struct ActorData {
    pub _render_object: RcRefCell<RenderObjectData>,
    pub _transform_object: *mut TransformObjectData,
    pub _controller: ActorController,
    pub _armor: ArmorInstance,
    pub _weapons: Vec<Box<dyn WeaponTrait>>,
}

pub trait ActorTrait {
    fn initialize_actor(&mut self);
    fn get_actor_id(&self) -> u64;
    fn is_player_actor(&self) -> bool;
    fn get_actor_data(&self) -> &ActorData;
    fn get_actor_data_mut(&mut self) -> &mut ActorData;
    fn get_armor(&self) -> &ArmorInstance;
    fn get_armor_mut(&mut self) -> &mut ArmorInstance;
    fn get_controller(&self) -> &ActorController;
    fn get_controller_mut(&mut self) -> &mut ActorController;
    fn get_transform(&self) -> &TransformObjectData;
    fn get_transform_mut(&self) -> &mut TransformObjectData;
    fn update_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData);
    fn update_player_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData, main_camera: &mut CameraObjectData);
}

// Implementation

impl ActorData {
    pub fn create_actor_data(
        controller_type: ControllerDataType,
        armor_type: ArmorDataType,
        render_object: &RcRefCell<RenderObjectData>
    ) -> ActorData {
        let transform_object = (&mut render_object.borrow_mut()._transform_object as *mut TransformObjectData).clone();
        let floating_height = calc_floating_height(&render_object.borrow());
        ActorData {
            _render_object: render_object.clone(),
            _transform_object: transform_object,
            _controller: ActorController::create_actor_controller(controller_type, floating_height),
            _armor: ArmorInstance::create_armor_instance(armor_type),
            _weapons: Vec::new(),
        }
    }

    pub fn initialize_actor_data(&mut self, owner_actor: *const dyn ActorTrait) {
        let weapon = BeamEmitter::create_beam_emitter(
            owner_actor,
            &self._render_object,
            &TransformObjectData::new_transform_object_data()
        );
        self._weapons.push(weapon);
    }

    pub fn get_armor(&self) -> &ArmorInstance {
        &self._armor
    }

    pub fn get_armor_mut(&mut self) -> &mut ArmorInstance {
        &mut self._armor
    }

    pub fn get_controller(&self) -> &ActorController {
        &self._controller
    }

    pub fn get_controller_mut(&mut self) -> &mut ActorController {
        &mut self._controller
    }

    pub fn get_transform(&self) -> &TransformObjectData {
        unsafe { &(*self._transform_object) }
    }

    pub fn get_transform_mut(&self) -> &mut TransformObjectData {
        unsafe { &mut *(self._transform_object as *mut TransformObjectData) }
    }
}