use std::collections::HashMap;

use crate::game_module::weapons::bullet::{Bullet, BulletType, BULLET_TYPES, BulletData};
use crate::application::project_application::ProjectApplication;
use crate::application::project_audio_manager::AudioLoop;
use crate::game_module::weapons::weapon::{WeaponData, WeaponType, WEAPON_TYPES};

pub struct WeaponManager {
    pub _id_generator: u64,
    pub _bullet_data_map: HashMap<BulletType, Box<BulletData>>,
    pub _weapon_data_map: HashMap<WeaponType, Box<WeaponData>>,
    pub _bullets_array: HashMap<u64, *const Vec<Box<Bullet>>>,
}

impl WeaponManager {
    pub fn create_weapon_manager() -> Box<WeaponManager> {
        Box::new(WeaponManager {
            _id_generator: 0,
            _bullet_data_map: HashMap::new(),
            _weapon_data_map: HashMap::new(),
            _bullets_array: HashMap::new(),
        })
    }

    pub fn initialize_weapon_manager(&mut self, project_application: &ProjectApplication) {
        let _project_resources = project_application.get_project_resources();

        // bullets
        let default_bullet_data = BulletData {
            _bullet_type: BulletType::Beam,
            _shield_damage: 1.0,
            _hull_damage: 1.0,
            _bullet_speed: 1.0,
            _bullet_range: 10.0,
            _bullet_life_time: 10.0,
        };
        for bullet_type in BULLET_TYPES.iter() {
            self._bullet_data_map.insert(*bullet_type, Box::new(default_bullet_data.clone()));
        }

        // weapons
        let default_weapon_data = WeaponData {
            _weapon_type: WeaponType::BeamEmitter,
            _rate_of_fire: 1.0,
            _bullet_amount: 1,
            _bullet_data: std::ptr::null()
        };
        for weapon_type in WEAPON_TYPES.iter() {
            self._weapon_data_map.insert(*weapon_type, Box::new(default_weapon_data.clone()));
        }
    }

    pub fn destroy_weapon_manager(&mut self) {
        self._bullets_array.clear();
    }

    pub fn generate_id(&mut self) -> u64 {
        let id = self._id_generator;
        self._id_generator += 1;
        id
    }

    pub fn get_bullet_data(&self, bullet_type: BulletType) -> &Box<BulletData> {
        self._bullet_data_map.get(&bullet_type).unwrap()
    }

    pub fn get_weapon_data(&self, weapon_type: WeaponType) -> &Box<WeaponData> {
        self._weapon_data_map.get(&weapon_type).unwrap()
    }


    pub fn regist_bullets(&mut self, bullets: *const Vec<Box<Bullet>>) -> u64 {
        let id = self.generate_id();
        self._bullets_array.insert(id, bullets);
        id
    }

    pub fn unregist_bullets(&mut self, id: u64) {
        self._bullets_array.remove(&id);
    }

    // 이 함수는 사용 안할거임
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