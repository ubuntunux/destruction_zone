use std::collections::HashMap;

use rust_engine_3d::renderer::effect::EffectCreateInfo;
use rust_engine_3d::utilities::system::RcRefCell;

use crate::application::project_application::ProjectApplication;
use crate::application::project_audio_manager::AudioLoop;
use crate::game_module::weapons::bullet::Bullet;


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

    pub fn update_weapon_manager(&mut self, project_application: &ProjectApplication, delta_time: f32) {
        let height_map_data = project_application.get_project_scene_manager().get_height_map_data();

        let mut dead_bullets: Vec<u64> = Vec::new();
        for (id, bullet) in self._bullets_array.iter() {
            let bullet = &mut bullet.borrow_mut();
            bullet.update_bullet(delta_time, height_map_data);
            if false == bullet._is_alive {
                let transform = bullet.get_transform_object();
                let effect_create_info = EffectCreateInfo {
                    _effect_position: transform.get_position().clone_owned(),
                    _effect_rotation: transform.get_rotation().clone_owned(),
                    _effect_data_name: "effects/bullet_destroy".to_string(),
                    ..Default::default()
                };
                project_application.get_project_scene_manager_mut().add_effect("bullet_destroy", &effect_create_info);
                static BULLET_AUDIOS: [&str; 3] = ["Bullet_Metal_01", "Bullet_Metal_02", "Bullet_Metal_03"];
                let index: usize = rand::random::<usize>() % BULLET_AUDIOS.len();
                project_application.get_project_audio_manager_mut().create_audio(BULLET_AUDIOS[index], AudioLoop::ONCE);
                project_application.get_project_scene_manager_mut().remove_static_render_object(&bullet._bullet_render_object.borrow()._render_object_name);
                dead_bullets.push(*id);
            }
        }

        for id in dead_bullets.iter() {
            self._bullets_array.remove(&id);
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