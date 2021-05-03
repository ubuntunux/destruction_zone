use nalgebra::{ Vector3 };

use rust_engine_3d::utilities::math::lerp;
use rust_engine_3d::utilities::bounding_box::BoundingBox;

#[derive(Clone)]
pub struct HeightMapData {
    _bounding_box: BoundingBox,
    _width: i32,
    _height: i32,
    _height_map_data: Vec<u8>,
}

impl Default for HeightMapData {
    fn default() -> HeightMapData {
        HeightMapData {
            _bounding_box: BoundingBox::default(),
            _width: 1,
            _height: 1,
            _height_map_data: Vec::new(),
        }
    }
}

impl HeightMapData {
    pub fn initialize_height_map_data(&mut self, bounding_box: &BoundingBox, width: u32, height: u32, height_map_data: Vec<u8>) {
        self._bounding_box = bounding_box.clone();
        self._width = width as i32;
        self._height = height as i32;
        self._height_map_data = height_map_data;
    }

    pub fn get_height(&self, pos: &Vector3<f32>) -> f32 {
        if false == self._height_map_data.is_empty() {
            let width_minus_one = (self._width - 1) as f32;
            let height_minus_one = (self._height - 1) as f32;
            let pixel_pos_x = (pos.x - &self._bounding_box._min.x) / self._bounding_box._size.x * width_minus_one;
            let pixel_pos_y = (pos.z - &self._bounding_box._min.z) / self._bounding_box._size.z * height_minus_one;
            let pixel_pos_x = pixel_pos_x.min(width_minus_one).max(0.0);
            let pixel_pos_y = pixel_pos_y.min(height_minus_one).max(0.0);
            let pixel_pos_x_min = pixel_pos_x as i32;
            let pixel_pos_y_min = pixel_pos_y as i32 * self._width;
            let pixel_pos_x_max = pixel_pos_x.ceil() as i32;
            let pixel_pos_y_max = pixel_pos_y.ceil() as i32 * self._width;
            let pixel_pos_x_frac = pixel_pos_x.fract();
            let pixel_pos_y_frac = pixel_pos_y.fract();
            let tex_coord_00 = (pixel_pos_y_min + pixel_pos_x_min) as usize * 4;
            let tex_coord_01 = (pixel_pos_y_min + pixel_pos_x_max) as usize * 4;
            let tex_coord_10 = (pixel_pos_y_max + pixel_pos_x_min) as usize * 4;
            let tex_coord_11 = (pixel_pos_y_max + pixel_pos_x_max) as usize * 4;
            let height_data_0 = lerp(self._height_map_data[tex_coord_00] as f32, self._height_map_data[tex_coord_01] as f32, pixel_pos_x_frac) / 255.0;
            let height_data_1 = lerp(self._height_map_data[tex_coord_10] as f32, self._height_map_data[tex_coord_11] as f32, pixel_pos_x_frac) / 255.0;
            return lerp(height_data_0, height_data_1, pixel_pos_y_frac) as f32 * self._bounding_box._size.y;
        }
        0.0
    }
}