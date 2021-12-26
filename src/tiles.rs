use std::collections::HashMap;

use sdl2::rect::{Point, Rect};

const FLAT_SIDE_LENGTH: f32 = 32. / 30.;
const X_TEMPLATE: [f32; 6] = [0., FLAT_SIDE_LENGTH, FLAT_SIDE_LENGTH, 0., -FLAT_SIDE_LENGTH, -FLAT_SIDE_LENGTH];
const Y_TEMPLATE: [f32; 6] = [1., 0.5, -0.5, -1., -0.5, 0.5];

#[derive(Debug)]
pub struct Hexagon {
    pub x: [i16; 6],
    pub y: [i16; 6],
    pub rectangle: Rect,
}

impl Hexagon {
    pub fn new(origin: (i16, i16), coordinates: &Coordinates, pixel_per_hexagon: i16) -> Hexagon {
        let offset = coordinates.as_offset(pixel_per_hexagon);
        let x = X_TEMPLATE.map(|f| (f * pixel_per_hexagon as f32).round() as i16 + origin.0 + offset.0);
        let y = Y_TEMPLATE.map(|f| (f * pixel_per_hexagon as f32).round() as i16 + origin.1 + offset.1);

        let tile_center_offset = (48. - 30.) / 2.;
        let pixel_ratio = pixel_per_hexagon as f32 / 30.;
        let tile_center_offset_pixel = tile_center_offset * pixel_ratio;

        let center = Point::new((origin.0 + offset.0) as i32, (origin.1 + offset.1 + tile_center_offset_pixel.round() as i16) as i32);
        let rectangle = Rect::from_center(center, 32 * 4, 48 * 4);

        Hexagon { x, y, rectangle }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Coordinates {
    pub q: i16,
    pub r: i16,
}

impl Coordinates {
    pub fn from_offset(offset: &(i16, i16), origin: &(i16, i16), pixel_per_hexagon: i16) -> Coordinates {
        let x = offset.0 as f32 - origin.0 as f32;
        let y = offset.1 as f32 - origin.1 as f32;

        let q_f32 = ((2_f32.sqrt() / 3.) * x - y / 3.) / pixel_per_hexagon as f32;
        let r_f32 = (2. / 3.) * y / pixel_per_hexagon as f32;

        let q = q_f32.round() as i16;
        let r = r_f32.round() as i16;

        Coordinates { q, r }
    }

    pub fn as_offset(&self, pixel_per_hexagon: i16) -> (i16, i16) {
        let x_f32 = (pixel_per_hexagon as f32 * FLAT_SIDE_LENGTH).round() * (2. * self.q as f32 + self.r as f32);
        let y_f32 = (28. / 30.) * pixel_per_hexagon as f32 * 1.5 * self.r as f32;

        (x_f32.round() as i16, y_f32.round() as i16)
    }

    pub fn distance_to(&self, to: &Coordinates) -> i16 {
        let vec_distance = [self.q - to.q, self.r - to.r, self.s() - to.s()];
        let vec: Vec<i16> = vec_distance.iter().map(|value| value.abs()).collect();
        let result: i16 = vec.iter().sum();
        result / 2
    }

    fn s(&self) -> i16 {
        -self.q - self.r
    }
}

pub struct Grid {
    pub hexagons: HashMap<Coordinates, Hexagon>,
    pub q_max: i16,
    pub r_max: i16,
}

impl Grid {
    pub fn new(origin: (i16, i16), radius: i16, pixel_per_hexagon: i16) -> Result<Grid, &'static str> {
        let center = Coordinates { q: 0, r: 0 };
        let mut qr_vec = Vec::new();
        for q in -radius..=radius {
            for r in -radius..=radius {
                // TODO load all from json file for maps, with textures
                let target_coordinates = Coordinates { q, r };
                if center.distance_to(&target_coordinates) <= radius {
                    qr_vec.push(target_coordinates);
                }
            }
        }
        let hexagons = qr_vec.iter()
            .map(|coordinate| (*coordinate, Hexagon::new(origin, coordinate, pixel_per_hexagon)))
            .collect();

        Ok(Grid { hexagons, q_max: radius, r_max: radius })
    }
}
