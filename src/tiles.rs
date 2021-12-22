use std::num;

const SMALL_SIDE_LENGTH: f32 = 0.866;
const X_TEMPLATE: [f32; 6] = [1., 0.5, -0.5, -1., -0.5, 0.5];
const Y_TEMPLATE: [f32; 6] = [0., SMALL_SIDE_LENGTH, SMALL_SIDE_LENGTH, 0., -SMALL_SIDE_LENGTH, -SMALL_SIDE_LENGTH];

#[derive(Debug)]
pub struct Hexagon {
    pub x: [i16; 6],
    pub y: [i16; 6],
}

impl Hexagon {
    pub fn new(origin: (i16, i16), coordinates: Coordinates, size: i16) -> Result<Hexagon, &'static str> {
        let offset = coordinates.as_offset(size);
        let x = X_TEMPLATE.map(|f| (f * size as f32).round() as i16 + origin.0 + offset.0);
        let y = Y_TEMPLATE.map(|f| (f * size as f32).round() as i16 + origin.1 + offset.1);
        Ok(Hexagon { x, y })
    }
}

pub struct Coordinates {
    pub q: i16,
    pub r: i16,
}

impl Coordinates {
    pub fn as_offset(&self, size: i16) -> (i16, i16) {
        ((size as f32 * 1.5 * self.q as f32).round() as i16,
         (size as f32 * (self.q as f32 * SMALL_SIDE_LENGTH + self.r as f32 * 2. * SMALL_SIDE_LENGTH)).round() as i16)
    }
}