use nalgebra::{ Vector3 };

use rust_engine_3d::utilities::math::lerp;
use rust_engine_3d::utilities::bounding_box::BoundingBox;

#[derive(Clone)]
pub struct HeightMapData {
    _sea_height: f32,
    _bounding_box: BoundingBox,
    _lod_count: i32,
    _width: Vec<i32>,
    _height: Vec<i32>,
    _min_height_map_data: Vec<Vec<f32>>,
}

impl Default for HeightMapData {
    fn default() -> HeightMapData {
        HeightMapData {
            _sea_height: 0.0,
            _bounding_box: BoundingBox::default(),
            _lod_count: 0,
            _width: Vec::new(),
            _height: Vec::new(),
            _min_height_map_data: Vec::new(),
        }
    }
}

impl HeightMapData {
    pub fn initialize_height_map_data(&mut self, bounding_box: &BoundingBox, width: i32, height: i32, height_map_data: Vec<u8>, sea_height: f32) {
        self._sea_height = sea_height;
        self._bounding_box = bounding_box.clone();
        let max_height = bounding_box._size.y;
        let lod_count_x = (width as f32).log2() as i32 + 1;
        let lod_count_y = (height as f32).log2() as i32 + 1;
        self._lod_count = lod_count_x.min(lod_count_y);
        self._width.push(width);
        self._height.push(height);
        let mut lod_height_map_data: Vec<f32> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                lod_height_map_data.push(height_map_data[(y * width + x) as usize * 4] as f32 / 255.0 * max_height);
            }
        }
        self._min_height_map_data.push(lod_height_map_data);
        self.generate_lod();
    }

    pub fn generate_lod(&mut self) {
        for _ in 1..self._lod_count {
            let width = *self._width.last().unwrap() as i32;
            let height = *self._height.last().unwrap() as i32;
            self._width.push(width / 2);
            self._height.push(height / 2);
            let mut lod_height_map_data: Vec<f32> = Vec::new();
            let last_height_map_data = &self._min_height_map_data.last().unwrap();
            for y in (0..height).step_by(2) {
                for x in (0..width).step_by(2) {
                    let tex_coord_0 = (y * width + x) as usize;
                    let tex_coord_1 = ((y + 1) * width + x) as usize;
                    let height_00 = last_height_map_data[tex_coord_0];
                    let height_01 = last_height_map_data[tex_coord_0 + 1];
                    let height_10 = last_height_map_data[tex_coord_1];
                    let height_11 = last_height_map_data[tex_coord_1 + 1];
                    let min_height = height_00.min(height_01.min(height_10.min(height_11)));
                    lod_height_map_data.push(min_height);
                }
            }
            self._min_height_map_data.push(lod_height_map_data);
        }
    }

    pub fn get_height(&self, pos: &Vector3<f32>, lod: usize) -> f32 {
        let lod = lod.min(self._lod_count as usize - 1);
        let width: usize = self._width[lod] as usize;
        let height: usize = self._height[lod] as usize;
        let texcoord_x: f32 = (pos.x - &self._bounding_box._min.x) / self._bounding_box._size.x;
        let texcoord_y: f32 = (pos.z - &self._bounding_box._min.z) / self._bounding_box._size.z;
        let pixel_pos_x: f32 = 0f32.max(1f32.min(texcoord_x)) * (width - 1) as f32;
        let pixel_pos_y: f32 = 0f32.max(1f32.min(texcoord_y)) * (height - 1) as f32;
        let pixel_pos_x_frac: f32 = pixel_pos_x.fract();
        let pixel_pos_y_frac: f32 = pixel_pos_y.fract();
        let pixel_pos_x_min: usize = pixel_pos_x as usize;
        let pixel_pos_y_min: usize = pixel_pos_y as usize * width;
        let pixel_pos_x_max: usize = pixel_pos_x.ceil() as usize;
        let pixel_pos_y_max: usize = pixel_pos_y.ceil() as usize * width;
        let pixel_index_00: usize = pixel_pos_y_min + pixel_pos_x_min;
        let pixel_index_01: usize = pixel_pos_y_min + pixel_pos_x_max;
        let pixel_index_10: usize = pixel_pos_y_max + pixel_pos_x_min;
        let pixel_index_11: usize = pixel_pos_y_max + pixel_pos_x_max;
        let height_map_data = &self._min_height_map_data[lod];
        let height_data_0 = lerp(height_map_data[pixel_index_00], height_map_data[pixel_index_01], pixel_pos_x_frac);
        let height_data_1 = lerp(height_map_data[pixel_index_10], height_map_data[pixel_index_11], pixel_pos_x_frac);
        let height = self._bounding_box._min.y + lerp(height_data_0, height_data_1, pixel_pos_y_frac);
        self._sea_height.max(height)
    }
}