pub enum ArmorDataType {
    Default,
}

#[derive(Clone, Debug, Copy)]
pub struct ArmorData {
    pub _physical_armor: f32,
    pub _shield_armor: f32,
    pub _max_hit_point: f32,
    pub _max_shields: f32,
}

#[derive(Clone, Debug, Copy)]
pub struct ArmorInstance {
    pub _armor_data: ArmorData,
    pub _hit_point: f32,
    pub _shields: f32,
}

impl ArmorData {
    pub fn create_armor_data(armor_data_type: ArmorDataType) -> ArmorData {
        match armor_data_type {
            ArmorDataType::Default => ArmorData {
                _physical_armor: 0.0,
                _shield_armor: 0.0,
                _max_hit_point: 100.0,
                _max_shields: 10.0,
            }
        }
    }
}

impl ArmorInstance {
    pub fn create_armor_instance(armor_data_type: ArmorDataType) -> ArmorInstance {
        let armor_data = ArmorData::create_armor_data(armor_data_type);
        ArmorInstance {
            _armor_data: armor_data.clone(),
            _hit_point: armor_data._max_hit_point,
            _shields: armor_data._max_shields,
        }
    }
}