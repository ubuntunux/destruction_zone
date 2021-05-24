use std::collections::HashMap;

//use rust_engine_3d::utilities::system::{ self, newRcRefCell, RcRefCell };

use crate::game_module::weapons::base_bullet::BaseBullet;
use crate::application::project_application::ProjectApplication;
use crate::application::project_audio_manager::AudioLoop;

pub struct WeaponManager {
    pub _id_generator: u64,
    pub _bullets: HashMap<u64, Box<dyn BaseBullet>>,
}

impl WeaponManager {
    pub fn create_weapon_manager() -> Box<WeaponManager> {
        Box::new(WeaponManager {
            _id_generator: 0,
            _bullets: HashMap::new(),
        })
    }

    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }

    pub fn initialize_weapon_manager(&mut self, project_application: &ProjectApplication) {

    }

    pub fn destroy_weapon_manager(&mut self) {

    }

    pub fn add_bullet(&self, project_application: &ProjectApplication) {
        project_application.get_project_audio_manager_mut().create_audio("assaultrifle1", AudioLoop::ONCE);
        // bullet_model = self.resource_manager.get_model("Cube")
        // bullet_object = self.scene_manager.add_object(model=bullet_model, instance_count=BulletActor.max_bullet_count, instance_render_count=0)
        // bullet = BulletActor(self, bullet_object)
        // self.bullets.append(bullet)
        // return bullet
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