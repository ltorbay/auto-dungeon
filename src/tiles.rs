use std::collections::HashMap;

const SMALL_SIDE_LENGTH: f32 = 0.866;
const X_TEMPLATE: [f32; 6] = [1., 0.5, -0.5, -1., -0.5, 0.5];
const Y_TEMPLATE: [f32; 6] = [0., SMALL_SIDE_LENGTH, SMALL_SIDE_LENGTH, 0., -SMALL_SIDE_LENGTH, -SMALL_SIDE_LENGTH];

#[derive(Debug)]
pub struct Hexagon {
    pub x: [i16; 6],
    pub y: [i16; 6],

    // TODO Add texture
}

impl Hexagon {
    pub fn new(origin: (i16, i16), coordinates: &Coordinates, pixel_per_hexagon: i16) -> Hexagon {
        let offset = coordinates.as_offset(pixel_per_hexagon);
        let x = X_TEMPLATE.map(|f| (f * pixel_per_hexagon as f32).round() as i16 + origin.0 + offset.0);
        let y = Y_TEMPLATE.map(|f| (f * pixel_per_hexagon as f32).round() as i16 + origin.1 + offset.1);
        Hexagon { x, y }
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
        
        let q_f32 = (2./3.) * x / pixel_per_hexagon as f32;
        let r_f32 = ((-1./3.) * x + (3_f32.sqrt()/3.) * y ) / pixel_per_hexagon as f32;
        
        let q = q_f32.round() as i16;
        let r = r_f32.round() as i16;
        
        Coordinates { q, r }
    }

    pub fn as_offset(&self, pixel_per_hexagon: i16) -> (i16, i16) {
        ((pixel_per_hexagon as f32 * 1.5 * self.q as f32).round() as i16,
         ((pixel_per_hexagon as f32 * SMALL_SIDE_LENGTH).round() * (self.q as f32 + self.r as f32 * 2.)) as i16)
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
