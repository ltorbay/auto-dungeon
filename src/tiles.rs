use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use sdl2::rect::{Point, Rect};

use generator::NoiseGenerator;
use textures::{BiomeType, TerrainType};

const FLAT_SIDE_LENGTH: f32 = 32. / 30.;
const X_TEMPLATE: [f32; 6] = [0., FLAT_SIDE_LENGTH, FLAT_SIDE_LENGTH, 0., -FLAT_SIDE_LENGTH, -FLAT_SIDE_LENGTH];
const Y_TEMPLATE: [f32; 6] = [1., 0.5, -0.5, -1., -0.5, 0.5];

#[derive(Debug)]
pub struct Hexagon {
    pub x: [i32; 6],
    pub y: [i32; 6],
    pub rectangle: Rect,
    pub texture_type: (TerrainType, BiomeType),
    pub height: u8,
}

impl Hexagon {
    pub fn new(origin: (i32, i32), coordinates: &Coordinates, pixel_per_hexagon: i32, texture_type: (TerrainType, BiomeType), height: u8) -> Hexagon {
        let offset = coordinates.as_offset(pixel_per_hexagon);
        let x = X_TEMPLATE.map(|f| (f * pixel_per_hexagon as f32).round() as i32 + origin.0 + offset.0);
        let y = Y_TEMPLATE.map(|f| (f * pixel_per_hexagon as f32).round() as i32 + origin.1 + offset.1);

        let tile_center_offset = (48. - 30.) / 2.;
        let pixel_ratio = pixel_per_hexagon as f32 / 30.;
        let tile_center_offset_pixel = tile_center_offset * pixel_ratio;

        let center = Point::new((origin.0 + offset.0) as i32, (origin.1 + offset.1 + tile_center_offset_pixel.round() as i32) as i32);
        let texture_ratio = (pixel_per_hexagon as f32 * 2. / 30.).round() as u32;
        let rectangle = Rect::from_center(center, 32 * texture_ratio, 48 * texture_ratio);

        Hexagon { x, y, rectangle, texture_type, height }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Coordinates {
    pub q: i32,
    pub r: i32,
}

impl Coordinates {
    pub fn shift(&self, q_offset: i32, r_offset: i32) -> Coordinates {
        Coordinates { q: self.q + q_offset, r: self.r + r_offset }
    }
    
    pub fn from_offset(offset: &(i32, i32), origin: &(i32, i32), pixel_per_hexagon: i32) -> Coordinates {
        let x = offset.0 as f32 - origin.0 as f32;
        let y = offset.1 as f32 - origin.1 as f32;

        let q_f32 = ((2_f32.sqrt() / 3.) * x - y / 3.) / pixel_per_hexagon as f32;
        let r_f32 = (2. / 3.) * y / pixel_per_hexagon as f32;

        let q = q_f32.round() as i32;
        let r = r_f32.round() as i32;

        Coordinates { q, r }
    }

    pub fn as_offset(&self, pixel_per_hexagon: i32) -> (i32, i32) {
        let x_f32 = (pixel_per_hexagon as f32 * FLAT_SIDE_LENGTH).round() * (2. * self.q as f32 + self.r as f32);
        let y_f32 = (28. / 30.) * pixel_per_hexagon as f32 * 1.5 * self.r as f32;

        (x_f32.round() as i32, y_f32.round() as i32)
    }

    pub fn distance_to(&self, to: &Coordinates) -> i32 {
        let vec_distance = [self.q - to.q, self.r - to.r, self.s() - to.s()];
        let vec: Vec<i32> = vec_distance.iter().map(|value| value.abs()).collect();
        let result: i32 = vec.iter().sum();
        result / 2
    }

    pub fn quick_hash(&self) -> u64 {
        let mut s = DefaultHasher::new();
        self.hash(&mut s);
        s.finish()
    }

    fn s(&self) -> i32 {
        -self.q - self.r
    }
}

pub struct Grid {
    pub hexagons: HashMap<Coordinates, Hexagon>,
    pub q_max: i32,
    pub r_max: i32,
}

impl Grid {
    pub fn new(origin: (i32, i32), noise_generator: &NoiseGenerator, radius: i32, pixel_per_hexagon: i32) -> Result<Grid, &'static str> {
        let center = Coordinates { q: 0, r: 0 };
        let mut qr_vec = Vec::new();
        for q in -radius..=radius {
            for r in -radius..=radius {
                let target_coordinates = Coordinates { q, r };
                if center.distance_to(&target_coordinates) <= radius {
                    qr_vec.push(target_coordinates);
                }
            }
        }
        let hexagons = qr_vec.iter()
            .map(|coordinate| {
                let offset = coordinate.as_offset(pixel_per_hexagon);
                let height = noise_generator.height(&origin, offset.0, offset.1);
                let humidity = noise_generator.humidity(&origin, offset.0, offset.1);
                (*coordinate, Hexagon::new(origin, coordinate, pixel_per_hexagon, (TerrainType::Flat, BiomeType::new(height, humidity)), (height + 0.4).floor() as u8))
            })
            .collect();

        Ok(Grid { hexagons, q_max: radius, r_max: radius })
    }
}
