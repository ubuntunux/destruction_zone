#[derive(Clone, Debug)]
pub enum ShipDataType {
    Scout,
}

#[derive(Clone, Debug)]
pub struct ShipData {
    pub _ship_name: String,
    pub _ship_type: ShipDataType,
    pub _model_data_name: String,
    pub _hull_armor: f32,
    pub _shield_armor: f32,
    pub _max_hull: f32,
    pub _max_shields: f32,
}

#[derive(Clone, Debug)]
pub struct ShipInstance {
    pub _ship_data: ShipData,
    pub _hull: f32,
    pub _shields: f32,
}

impl ShipData {
    pub fn create_ship_data(ship_data_type: ShipDataType) -> ShipData {
        match ship_data_type {
            ShipDataType::Scout => ShipData {
                _ship_name: "".to_string(),
                _ship_type: ShipDataType::Scout,
                _model_data_name: "".to_string(),
                _hull_armor: 0.0,
                _shield_armor: 0.0,
                _max_hull: 100.0,
                _max_shields: 10.0,
            }
        }
    }
}

impl ShipInstance {
    pub fn create_ship_instance(ship_data_type: ShipDataType) -> ShipInstance {
        let ship_data = ShipData::create_ship_data(ship_data_type);
        ShipInstance {
            _ship_data: ship_data.clone(),
            _hull: ship_data._max_hull,
            _shields: ship_data._max_shields,
        }
    }

    pub fn get_hull_point(&self) -> f32 {
        self._hull
    }

    pub fn get_max_hull_point(&self) -> f32 {
        self._ship_data._max_hull
    }

    pub fn get_shield_point(&self) -> f32 {
        self._shields
    }

    pub fn get_max_shield_point(&self) -> f32 {
        self._ship_data._max_shields
    }
}