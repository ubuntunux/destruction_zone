use std::collections::HashMap;

use nalgebra::Vector3;

use rust_engine_3d::application::audio_manager::AudioLoop;
use rust_engine_3d::effect::effect_data::EffectCreateInfo;
use rust_engine_3d::utilities::bounding_box::BoundingBox;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::application::project_application::ProjectApplication;
use crate::game_module::weapons::bullet::Bullet;
use crate::game_module::actor_manager::{ActorManager, ActorMap};

pub struct WeaponManager {
    pub _id_generator: u64,
    pub _bullets_array: HashMap<u64, RcRefCell<Bullet>>,
}

impl WeaponManager {
    pub fn create_weapon_manager() -> Box<WeaponManager> {
        Box::new(WeaponManager {
            _id_generator: 0,
            _bullets_array: HashMap::new(),
        })
    }

    pub fn initialize_weapon_manager(&mut self, _project_application: &ProjectApplication) {
    }

    pub fn destroy_weapon_manager(&mut self) {
        self._bullets_array.clear();
    }

    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }

    pub fn regist_bullets(&mut self, bullet: &RcRefCell<Bullet>) -> u64 {
        let id = self.generate_id();
        self._bullets_array.insert(id, bullet.clone());
        id
    }

    pub fn unregist_bullets(&mut self, id: u64) {
        self._bullets_array.remove(&id);
    }

    pub fn update_weapon_manager(&mut self, project_application: &ProjectApplication, actor_manager: &mut ActorManager, delta_time: f32) {
        let height_map_data = project_application.get_project_scene_manager().get_height_map_data();

        let mut dead_bullets: Vec<u64> = Vec::new();
        for (id, bullet) in self._bullets_array.iter() {
            let bullet = &mut bullet.borrow_mut();
            bullet.update_bullet(delta_time, height_map_data);

            if bullet._is_alive {
                let is_player_actor = bullet.get_owner_actor().is_player_actor();
                let actors_map_ptr: *const ActorMap = &actor_manager._actors;
                let actors_map: &mut ActorMap = unsafe { &mut *(actors_map_ptr as *mut ActorMap) };
                for actor in actors_map.values_mut() {
                    if is_player_actor != actor.is_player_actor() {
                        let bullet_transform = bullet.get_transform_object();
                        let intersect = {
                            let actor_bound_box: &BoundingBox = &actor.get_ship()._render_object.borrow()._bound_box;
                            let to_actor: Vector3<f32> = &actor_bound_box._center - bullet_transform.get_position();
                            to_actor.norm() <= actor_bound_box._radius
                        };

                        if intersect {
                            actor_manager.remove_actor(project_application.get_project_scene_manager_mut(), actor.as_mut());
                            bullet._is_alive = false;
                            bullet._is_collided = true;
                            break;
                        }
                    }
                }
            }

            if false == bullet._is_alive {
                if bullet._is_collided {
                    let bullet_transform = bullet.get_transform_object();

                    let bullet_destroy_effect_count = bullet.get_bullet_data()._bullet_destroy_effects.len();
                    if 0 < bullet_destroy_effect_count {
                        let effect_index: usize = if 1 < bullet_destroy_effect_count { rand::random::<usize>() % bullet_destroy_effect_count } else { 0 };
                        let effect_create_info = EffectCreateInfo {
                            _effect_position: bullet_transform.get_position().clone_owned(),
                            _effect_rotation: bullet_transform.get_rotation().clone_owned(),
                            _effect_data_name: bullet.get_bullet_data()._bullet_destroy_effects[effect_index].clone(),
                            ..Default::default()
                        };
                        project_application.get_project_scene_manager_mut().add_effect(&effect_create_info._effect_data_name, &effect_create_info);
                    }

                    if false == bullet.get_bullet_data()._bullet_destroy_sound_bank.is_empty() {
                        project_application.get_audio_manager_mut().create_audio_bank(&bullet.get_bullet_data()._bullet_destroy_sound_bank, AudioLoop::ONCE);
                    }
                }
                project_application.get_project_scene_manager_mut().remove_static_render_object(&bullet._bullet_render_object.borrow()._render_object_name);
                dead_bullets.push(*id);
            }
        }

        for id in dead_bullets.iter() {
            self.unregist_bullets(*id);
        }
    }


    /*
    pub fn destroy(self) {
        for bullet in self.bullets:
            bullet.destroy(self.scene_manager)
    }

    pub fn set_damage(self, bullet, target_actor) {
        target_actor.set_damage(bullet.damage)
    }

    pub fn update_bullets(self, delta_time, actors) {
        player_actor = self.actor_manager.player_actor
        for bullet in self.bullets:
            if bullet.actor is player_actor:
                for actor in actors:
                    if bullet.check_collide(actor):
                        self.set_damage(bullet, actor)
            else:
                if bullet.check_collide(player_actor):
                    self.set_damage(bullet, player_actor)
            bullet.update_bullet(self.core_manager.debug_line_manager, delta_time)
    }
*/
}