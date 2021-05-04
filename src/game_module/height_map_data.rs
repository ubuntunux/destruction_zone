use nalgebra::{ Vector3 };

use rust_engine_3d::utilities::math::lerp;
use rust_engine_3d::utilities::bounding_box::BoundingBox;

#[derive(Clone)]
pub struct HeightMapData {
    _bounding_box: BoundingBox,
    _lod_count: i32,
    _width: Vec<i32>,
    _height: Vec<i32>,
    _max_height_map_data: Vec<Vec<f32>>,
}

impl Default for HeightMapData {
    fn default() -> HeightMapData {
        HeightMapData {
            _bounding_box: BoundingBox::default(),
            _lod_count: 0,
            _width: Vec::new(),
            _height: Vec::new(),
            _max_height_map_data: Vec::new(),
        }
    }
}

impl HeightMapData {
    pub fn initialize_height_map_data(&mut self, bounding_box: &BoundingBox, width: i32, height: i32, height_map_data: Vec<u8>) {
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
        self._max_height_map_data.push(lod_height_map_data);
        self.generate_lod();
    }

    pub fn generate_lod(&mut self) {
        // let max_height = bounding_box._size.y;
        // let lod_count_x = (width as f32).log2() as i32 + 1;
        // let lod_count_y = (height as f32).log2() as i32 + 1;
        // self._lod_count = lod_count_x.min(lod_count_y);
        // for lod in 0..self._lod_count {
        //     let lod_width = width as i32 >> lod;
        //     let lod_height = height as i32 >> lod;
        //     self._width.push(lod_width);
        //     self._height.push(lod_height);
        //     let mut lod_height_map_data: Vec<f32> = Vec::new();
        //     for y in 0..lod_height {
        //         for x in 0..lod_width {
        //             let height = height_map_data[(y * width + x) * 4] as f32 / 255.0 * max_height;
        //             lod_height_map_data.push(height);
        //         }
        //     }
        //     self._max_height_map_data.push(lod_height_map_data);
        // }

    }

    pub fn get_height(&self, pos: &Vector3<f32>, lod: usize) -> f32 {
        let width_minus_one = (self._width[lod] - 1) as f32;
        let height_minus_one = (self._height[lod] - 1) as f32;
        let pixel_pos_x = (pos.x - &self._bounding_box._min.x) / self._bounding_box._size.x * width_minus_one;
        let pixel_pos_y = (pos.z - &self._bounding_box._min.z) / self._bounding_box._size.z * height_minus_one;
        let pixel_pos_x = pixel_pos_x.min(width_minus_one).max(0.0);
        let pixel_pos_y = pixel_pos_y.min(height_minus_one).max(0.0);
        let pixel_pos_x_min = pixel_pos_x as i32;
        let pixel_pos_y_min = pixel_pos_y as i32 * self._width[lod];
        let pixel_pos_x_max = pixel_pos_x.ceil() as i32;
        let pixel_pos_y_max = pixel_pos_y.ceil() as i32 * self._width[lod];
        let pixel_pos_x_frac = pixel_pos_x.fract();
        let pixel_pos_y_frac = pixel_pos_y.fract();
        let tex_coord_00 = (pixel_pos_y_min + pixel_pos_x_min) as usize;
        let tex_coord_01 = (pixel_pos_y_min + pixel_pos_x_max) as usize;
        let tex_coord_10 = (pixel_pos_y_max + pixel_pos_x_min) as usize;
        let tex_coord_11 = (pixel_pos_y_max + pixel_pos_x_max) as usize;
        let height_data_0 = lerp(self._max_height_map_data[lod][tex_coord_00], self._max_height_map_data[lod][tex_coord_01], pixel_pos_x_frac);
        let height_data_1 = lerp(self._max_height_map_data[lod][tex_coord_10], self._max_height_map_data[lod][tex_coord_11], pixel_pos_x_frac);
        return lerp(height_data_0, height_data_1, pixel_pos_y_frac);
    }
}