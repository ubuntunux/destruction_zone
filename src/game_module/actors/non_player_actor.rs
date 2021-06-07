use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::game_module::ship::ship_controller::{ ShipController };
use crate::game_module::actors::actor_data::{ ActorData, ActorTrait };
use crate::game_module::height_map_data::HeightMapData;
use crate::game_module::ship::ship::{ShipInstance, ShipData};

pub struct NonPlayerActor {
    pub _id: u64,
    pub _actor_data: ActorData,
    pub _ship: ShipInstance,
}

impl ActorTrait for NonPlayerActor {
    fn initialize_actor(&mut self) {
    }

    fn get_actor_id(&self) -> u64 {
        self._id
    }

    fn is_player_actor(&self) -> bool {
        false
    }

    fn get_actor_data(&self) -> &ActorData {
        &self._actor_data
    }

    fn get_actor_data_mut(&mut self) -> &mut ActorData {
        &mut self._actor_data
    }

    fn get_ship(&self) -> &ShipInstance {
        &self._ship
    }

    fn get_ship_mut(&mut self) -> &mut ShipInstance {
        &mut self._ship
    }

    fn get_controller(&self) -> &ShipController {
        &self._ship._controller
    }

    fn get_controller_mut(&mut self) -> &mut ShipController {
        &mut self._ship._controller
    }

    fn get_transform(&self) -> &TransformObjectData {
        self._ship.get_transform()
    }

    fn get_transform_mut(&self) -> &mut TransformObjectData {
        self._ship.get_transform_mut()
    }

    fn fire(&mut self) {
        unimplemented!()
    }

    fn update_actor(&mut self, delta_time: f32, height_map_data: &HeightMapData) {
        let transform = unsafe { &mut *(self._ship._transform_object as *mut TransformObjectData) };

        // update actor controller
        let ship_controller = &mut self._ship._controller;
        ship_controller.update_controller(delta_time, transform, height_map_data);

        // update transform
        transform.rotation_pitch(ship_controller.get_velocity_pitch() * delta_time);
        transform.rotation_yaw(ship_controller.get_velocity_yaw() * delta_time);
        transform.set_roll(ship_controller.get_roll());
        transform.set_position(ship_controller.get_position());
    }
}

impl NonPlayerActor {
    pub fn create_actor(
        id: u64,
        ship_data: &RcRefCell<ShipData>,
        render_object: &RcRefCell<RenderObjectData>
    ) -> Box<NonPlayerActor> {
        Box::new(NonPlayerActor {
            _id: id,
            _actor_data: ActorData {},
            _ship: ShipInstance::create_ship_instance(ship_data, render_object),
        })
    }
}