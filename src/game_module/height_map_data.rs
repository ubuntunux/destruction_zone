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
        assert!(2 <= self._lod_count, "lod_count must be greater than 2.");

        self._width.push(width);
        self._height.push(height);
        let mut lod_height_map_data: Vec<f32> = Vec::new();
        for y in 0..height {
            for x in 0..width {
                lod_height_map_data.push(height_map_data[(y * width + x) as usize * 4] as f32 / 255.0 * max_height);
            }
        }
        self._min_height_map_data.push(lod_height_map_data);
        self.generate_hiz_min();
    }

    pub fn generate_hiz_min(&mut self) {
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

    pub fn get_height_bilinear(&self, pos: &Vector3<f32>, lod: usize) -> f32 {
        let lod = lod.min(self._lod_count as usize - 1);
        let width = self._width[lod];
        let height = self._height[lod];
        let texcoord_x: f32 = (pos.x - &self._bounding_box._min.x) / self._bounding_box._size.x;
        let texcoord_y: f32 = (pos.z - &self._bounding_box._min.z) / self._bounding_box._size.z;
        let pixel_pos_x: f32 = 0f32.max(1f32.min(texcoord_x)) * (width - 1) as f32;
        let pixel_pos_y: f32 = 0f32.max(1f32.min(texcoord_y)) * (height - 1) as f32;
        let pixel_pos_x_frac: f32 = pixel_pos_x.fract();
        let pixel_pos_y_frac: f32 = pixel_pos_y.fract();
        let pixel_pos_x_min: i32 = pixel_pos_x as i32;
        let pixel_pos_y_min: i32 = pixel_pos_y as i32 * width;
        let pixel_pos_x_max: i32 = pixel_pos_x.ceil() as i32;
        let pixel_pos_y_max: i32 = pixel_pos_y.ceil() as i32 * width;
        let pixel_index_00: usize = (pixel_pos_y_min + pixel_pos_x_min) as usize;
        let pixel_index_01: usize = (pixel_pos_y_min + pixel_pos_x_max) as usize;
        let pixel_index_10: usize = (pixel_pos_y_max + pixel_pos_x_min) as usize;
        let pixel_index_11: usize = (pixel_pos_y_max + pixel_pos_x_max) as usize;
        let height_map_data = &self._min_height_map_data[lod];
        let height_data_0 = lerp(height_map_data[pixel_index_00], height_map_data[pixel_index_01], pixel_pos_x_frac);
        let height_data_1 = lerp(height_map_data[pixel_index_10], height_map_data[pixel_index_11], pixel_pos_x_frac);
        let height = self._bounding_box._min.y + lerp(height_data_0, height_data_1, pixel_pos_y_frac);
        self._sea_height.max(height as f32)
    }

    pub fn get_height_point(&self, pos: &Vector3<f32>, lod: usize) -> f32 {
        let lod = lod.min(self._lod_count as usize - 1);
        let width = self._width[lod];
        let height = self._height[lod];
        let texcoord_x: f32 = (pos.x - &self._bounding_box._min.x) / self._bounding_box._size.x;
        let texcoord_y: f32 = (pos.z - &self._bounding_box._min.z) / self._bounding_box._size.z;
        let pixel_pos_x: i32 = (0f32.max(1f32.min(texcoord_x)) * (width - 1) as f32) as i32;
        let pixel_pos_y: i32 = (0f32.max(1f32.min(texcoord_y)) * (height - 1) as f32) as i32;
        let pixel_index: usize = (pixel_pos_x + pixel_pos_y * width) as usize;
        let height = self._bounding_box._min.y + self._min_height_map_data[lod][pixel_index];
        self._sea_height.max(height as f32)
    }

    pub fn get_collision_point(&self, start_pos: &Vector3<f32>, dir: &Vector3<f32>, limit_dist: f32, collision_point: &mut Vector3<f32>) -> bool {
        log::info!("========== get_collision_point ===================");
        log::info!("    start_pos: {:?}, dir: {:?}", start_pos, dir);

        let max_lod: i32 = 5;//self._lod_count - 2;
        let mut lod: i32 = if limit_dist < 0f32 {
            max_lod
        } else {
            let max_dist: f32 = (dir.x.abs().max(dir.z.abs()) * limit_dist).ceil();
            let max_size: f32 = self._bounding_box._size.x.max(self._bounding_box._size.z);
            max_lod.min((max_size / max_dist).ceil().log2().ceil() as i32)
        };

        let mut pos: Vector3<f32> = start_pos.clone_owned();
        let goal_pos: Vector3<f32> = start_pos + dir * limit_dist;
        let step: f32 = 1f32;

        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut texcoord_x: f32 = 0f32;
        let mut texcoord_y: f32 = 0f32;
        let mut pixel_pos_x: i32 = 0;
        let mut pixel_pos_y: i32 = 0;
        let mut pixel_index: usize = 0;
        let mut changed_lod = true;
        let mut collided = false;
        while 0 <= lod {
            if changed_lod {
                log::info!("    changed_lod: {:?}", lod);
                width = self._width[lod as usize];
                height = self._height[lod as usize];
                texcoord_x = (pos.x - &self._bounding_box._min.x) / self._bounding_box._size.x;
                texcoord_y = (pos.z - &self._bounding_box._min.z) / self._bounding_box._size.z;
                pixel_pos_x = (0f32.max(1f32.min(texcoord_x)) * (width - 1) as f32) as i32;
                pixel_pos_y = (0f32.max(1f32.min(texcoord_y)) * (height - 1) as f32) as i32;
                pixel_index = (pixel_pos_x + pixel_pos_y * width) as usize;
                changed_lod = false;

                log::info!("    changed_lod: pos: {:?}, width: {:?}, height: {:?}, texcoord_x: {:?}, texcoord_y: {:?}, pixel_pos_x: {:?}, pixel_pos_y: {:?}", pos, width, height, texcoord_x, texcoord_y, pixel_pos_x, pixel_pos_y);
            }

            let height_value = self._sea_height.max( self._bounding_box._min.y + self._min_height_map_data[lod as usize][pixel_index] as f32 );
            if pos.y <= height_value {
                collided = true;
                collision_point.clone_from(&pos);
                collision_point.y = height_value;

                log::info!("    collide!!! collision_point: {:?}", collision_point);

                lod -= 1;
                changed_lod = true;
                continue;
            }

            // next step
            let pos_x = self._bounding_box._min.x + (self._bounding_box._size.x / width as f32) * (pixel_pos_x + if 0f32 < dir.x { 1 } else { 0 }) as f32;
            let pos_z = self._bounding_box._min.z + (self._bounding_box._size.z / height as f32) * (pixel_pos_y + if 0f32 < dir.z { 1 } else { 0 }) as f32;
            let dx: f32 = (pos_x - start_pos.x).abs();
            let dz: f32 = (pos_z - start_pos.z).abs();
            if (dir.z / dir.x * dx).abs() <= dz {
                // horizontal step
                pos = start_pos + dir / dir.x.abs() * dx;
                pixel_pos_x += if 0f32 < dir.x { 1 } else { -1 };
            } else {
                // vertical step
                pos = start_pos + dir / dir.z.abs() * dz;
                pixel_pos_y += if 0f32 < dir.z { 1 } else { -1 };
            }

            log::info!("    Step: pos: {:?}, width: {:?}, height: {:?}, texcoord_x: {:?}, texcoord_y: {:?}, pixel_pos_x: {:?}, pixel_pos_y: {:?}", pos, width, height, texcoord_x, texcoord_y, pixel_pos_x, pixel_pos_y);

            if pixel_pos_x < 0 || pixel_pos_y < 0 || width <= pixel_pos_x || height <= pixel_pos_y {
                log::info!("    out of range: {:?}", collided);
                return collided;
            }

            pixel_index = (pixel_pos_x + pixel_pos_y * width) as usize;
        }

        log::info!("    end: {:?}", collided);
        collided
    }
}