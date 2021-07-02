use nalgebra::Vector3;
use serde::{ Serialize, Deserialize };

use rust_engine_3d::renderer::render_object::RenderObjectData;
use rust_engine_3d::renderer::transform_object::TransformObjectData;
use rust_engine_3d::utilities::system::{RcRefCell, newRcRefCell};

use crate::game_module::actors::actor_data::ActorTrait;
use crate::game_module::height_map_data::HeightMapData;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash, Debug, Copy)]
pub enum BulletType {
    Beam,
    Gatling,
    Laser,
    Plasma,
    Shotgun
}

pub const BULLET_TYPES: [BulletType; 5] = [
    BulletType::Beam,
    BulletType::Gatling,
    BulletType::Laser,
    BulletType::Plasma,
    BulletType::Shotgun
];

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct BulletData {
    pub _bullet_type: BulletType,
    pub _shield_damage: f32,
    pub _hull_damage: f32,
    pub _bullet_speed: f32,
    pub _bullet_range: f32,
    pub _bullet_life_time: f32,
    pub _bullet_destroy_effects: Vec<String>,
    pub _bullet_destroy_sound_bank: String,
    pub _model_data_name: String
}

impl Default for BulletData {
    fn default() -> BulletData {
        BulletData {
            _bullet_type: BulletType::Beam,
            _shield_damage: 1.0,
            _hull_damage: 1.0,
            _bullet_speed: 100.0,
            _bullet_range: 100.0,
            _bullet_life_time: 10.0,
            _bullet_destroy_effects: Vec::new(),
            _bullet_destroy_sound_bank: "".to_string(),
            _model_data_name: "".to_string(),
        }
    }
}

pub struct Bullet {
    pub _bullet_data: *const BulletData,
    pub _owner_actor: *const dyn ActorTrait,
    pub _is_alive: bool,
    pub _is_collided: bool,
    pub _elapsed_time: f32,
    pub _transform: *const TransformObjectData,
    pub _initial_position: Vector3<f32>,
    pub _bullet_render_object: RcRefCell<RenderObjectData>,
}


// Implementation
impl Bullet {
    pub fn create_bullet(bullet_data: *const BulletData, owner_actor: *const dyn ActorTrait, bullet_render_object: &RcRefCell<RenderObjectData>) -> RcRefCell<Bullet> {
        newRcRefCell(Bullet {
            _owner_actor: owner_actor,
            _transform: &bullet_render_object.borrow()._transform_object,
            _initial_position: bullet_render_object.borrow()._transform_object.get_position().clone_owned(),
            _bullet_data: bullet_data,
            _elapsed_time: 0.0,
            _is_alive: true,
            _is_collided: false,
            _bullet_render_object: bullet_render_object.clone(),
        })
    }
    pub fn get_owner_actor(&self) -> &dyn ActorTrait {
        unsafe { &*self._owner_actor }
    }
    pub fn get_owner_actor_mut(&self) -> &mut dyn ActorTrait { unsafe { &mut *(self._owner_actor as *mut dyn ActorTrait) } }
    pub fn get_bullet_type(&self) -> BulletType { self.get_bullet_data()._bullet_type }
    pub fn get_bullet_data(&self) -> &BulletData { unsafe { &*self._bullet_data } }
    pub fn get_transform_object(&self) -> &TransformObjectData { unsafe { &*self._transform } }
    pub fn get_transform_object_mut(&self) -> &mut TransformObjectData { unsafe { &mut *(self._transform as *mut TransformObjectData) } }
    pub fn update_bullet(&mut self, delta_time: f32, height_map_data: &HeightMapData) -> bool {
        if self._is_alive {
            let bullet_data = unsafe { &*self._bullet_data };

            let transform = unsafe { &mut *(self._transform as *mut TransformObjectData) };
            transform.move_front(bullet_data._bullet_speed * delta_time);

            self._elapsed_time += delta_time;

            let current_position = transform.get_position();
            let move_distance = (current_position - &self._initial_position).norm();
            if bullet_data._bullet_life_time < self._elapsed_time || bullet_data._bullet_range < move_distance {
                self._is_alive = false;
                return false;
            }

            let floating_height = height_map_data.get_height(current_position, 0);
            if current_position.y < floating_height {
                self._is_alive = false;
                self._is_collided = true;
                return false;
            }
        }
        self._is_alive
    }
}



/*
class BulletActor:
    fire_offset = 0.5
    fire_term = 0.1
    max_distance = 1000.0
    bullet_speed = 1000.0
    damage = 1
    max_bullet_count = max(10, int(math.ceil((bullet_speed / max_distance) / fire_term)))

    def __init__(self, bullet_manager, bullet_object):
        self.bullet_manager = bullet_manager
        self.game_client = bullet_manager.game_client
        self.sound_manager = bullet_manager.sound_manager
        self.game_effect_manager = self.game_client.game_effect_manager
        self.actor = None
        self.bullet_object = bullet_object
        self.destroy_position = None

        assert(1 < self.max_bullet_count and self.bullet_object.is_instancing())

        self.bullet_transforms = []
        for i in range(self.bullet_object.instance_count):
            self.bullet_transforms.append(TransformObject())
        self.bullet_count = 0
        self.elapsed_time = 0.0
        self.current_fire_term = 0.0

    def set_actor(self, actor):
        self.actor = actor

    def destroy(self, scene_manager):
        scene_manager.delete_object(self.bullet_object.name)

    def get_pos(self):
        return self.bullet_object.transform.get_pos()

    def get_transform(self):
        return self.bullet_object.transform

    def destroy_bullet(self, index, create_effect=False):
        if index < self.bullet_count:
            if create_effect:
                if self.destroy_position is not None:
                    destroy_position = self.destroy_position
                else:
                    destroy_position = self.bullet_transforms[index].get_pos()
                self.game_effect_manager.create_damage_particle(destroy_position)
                self.sound_manager.play_sound(random.choice(SOUND_BULLET_HITS), volume=0.2, position=destroy_position)

            last_index = self.bullet_count - 1
            if 0 < last_index:
                self.bullet_transforms[index], self.bullet_transforms[last_index] = self.bullet_transforms[last_index], self.bullet_transforms[index]
            self.bullet_count = last_index
            self.bullet_object.set_instance_render_count(self.bullet_count)

    def check_collide(self, actor):
        bound_box = actor.actor_object.bound_box
        bound_box_pos = bound_box.bound_center
        radius = bound_box.radius * 0.5

        for i in range(self.bullet_count):
            collide = False
            bullet_pos0 = self.bullet_transforms[i].get_prev_pos()
            bullet_pos1 = self.bullet_transforms[i].get_pos()
            to_actor0 = bound_box_pos - bullet_pos0
            to_actor1 = bound_box_pos - bullet_pos1
            if length(to_actor0) <= radius or length(to_actor1) <= radius:
                collide = True
            elif np.dot(to_actor0, to_actor1) <= 0.0:
                bullet_dir = normalize(bullet_pos1 - bullet_pos0)
                bullet_move_length = np.dot(to_actor0, bullet_dir)
                bullet_move_path = bullet_dir * bullet_move_length
                d = length(to_actor0 - bullet_move_path)
                if d <= radius:
                    self.destroy_position = bound_box_pos + to_actor0 - bullet_move_path
                    collide = True
            if collide:
                self.destroy_bullet(i, create_effect=True)
                return True
        return False

    def fire(self, fire_pos, fire_direction, camera_transform, target_actor_distance):
        if self.bullet_count < self.max_bullet_count and self.current_fire_term <= 0.0:
            bullet_transform = self.bullet_transforms[self.bullet_count]
            self.bullet_count += 1

            bullet_position = fire_pos + fire_direction * self.fire_offset

            # fire sound
            self.sound_manager.play_sound(SOUND_FIRE, position=bullet_position)

            if 0.0 < target_actor_distance:
                target_position = camera_transform.get_pos() - camera_transform.front * target_actor_distance
            else:
                target_position = bullet_position + bullet_position - fire_pos
            matrix = Matrix4()
            lookat(matrix, bullet_position, target_position, WORLD_UP)

            bullet_transform.rotationMatrix[0][:3] = matrix[0][:3]
            bullet_transform.rotationMatrix[1][:3] = matrix[1][:3]
            bullet_transform.rotationMatrix[2][:3] = matrix[2][:3]
            bullet_transform.matrix_to_vectors()
            bullet_transform.set_prev_pos(fire_pos)
            bullet_transform.set_pos(bullet_position)
            bullet_transform.update_transform()
            self.bullet_object.set_instance_render_count(self.bullet_count)
            self.current_fire_term = self.fire_term

    def update_bullet(self, debug_line_manager, delta_time):
        actor_pos = self.actor.actor_object.get_pos()
        self.bullet_object.transform.set_pos(actor_pos)

        bullet_index = 0
        for i in range(self.bullet_count):
            bullet_transform = self.bullet_transforms[bullet_index]
            current_pos = bullet_transform.get_pos()
            next_pos = current_pos + bullet_transform.front * self.bullet_speed * delta_time
            collide = self.game_client.check_collide(current_pos, next_pos)
            if length(current_pos - actor_pos) < self.max_distance and not collide:
                bullet_transform.set_pos(next_pos)
                bullet_transform.update_transform()

                # bullet_pos0 = bullet_transform.get_prev_pos()
                # bullet_pos1 = bullet_transform.get_pos()
                # debug_line_manager.draw_debug_line_3d(bullet_pos0, bullet_pos1, Float4(1.0, 1.0, 0.0, 1.0), 5.0, is_infinite=True)

                self.bullet_object.instance_matrix[i][...] = bullet_transform.matrix
                matrix_translate(self.bullet_object.instance_matrix[i], *(-actor_pos))
                bullet_index += 1
            else:
                self.destroy_bullet(bullet_index, create_effect=collide)
        if 0.0 < self.current_fire_term:
            self.current_fire_term -= delta_time

*/