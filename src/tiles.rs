use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

use ::{FLAT_SIDE_LENGTH, PIXEL_PER_HEXAGON};
use generator::NoiseGenerator;
use renderer::Printer;
use textures::{BiomeType, TerrainType, Textures};

#[derive(Debug)]
pub struct Hexagon {
    pub texture_type: (TerrainType, BiomeType),
    pub height: u8,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Coordinates {
    pub q: i32,
    pub r: i32,
}

impl Coordinates {
    const NEIGHBORS_PERMUTATIONS: [(i8, i8); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

    // TODO make Area struct
    pub fn build_hexagonal_area(center: Coordinates, radius: i32) -> Vec<Coordinates> {
        let mut qr_vec = Vec::new();
        for q in (center.q - radius)..=(center.q + radius) {
            for r in (center.r - radius)..=(center.r + radius) {
                let target_coordinates = Coordinates { q, r };
                if center.distance_to(&target_coordinates) <= radius {
                    qr_vec.push(target_coordinates);
                }
            }
        }
        qr_vec
    }

    pub fn shift(&self, q_offset: i32, r_offset: i32) -> Coordinates {
        Coordinates { q: self.q + q_offset, r: self.r + r_offset }
    }

    pub fn neighbors(&self) -> [Coordinates; 6] {
        Coordinates::NEIGHBORS_PERMUTATIONS
            .iter()
            .map(|(q_permutation, r_permutation)| self.shift(*q_permutation as i32, *r_permutation as i32))
            .collect::<Vec<Coordinates>>()
            .try_into()
            .unwrap()
    }

    pub fn as_offset(&self, center: &Coordinates) -> (i32, i32) {
        let normalized_q = (self.q - center.q) as f32;
        let normalized_r = (self.r - center.r) as f32;

        let x_f32 = (PIXEL_PER_HEXAGON as f32 * FLAT_SIDE_LENGTH).round() * (2. * normalized_q + normalized_r);
        let y_f32 = (28. / 30.) * PIXEL_PER_HEXAGON as f32 * 1.5 * normalized_r as f32;

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
}

impl Grid {
    pub fn new(noise_generator: &NoiseGenerator, area: &[Coordinates]) -> Result<Grid, &'static str> {
        // TODO edge detection, so tiles are aware of their neighbors
        // -> make peaks at the top
        // -> make deserts for beach on low altitudes only if close to water or sand
        // TODO generate hex based random elements according to biome (cactuses, trees...)
        let hexagons = area.iter()
            .map(|coordinates| {
                let height = noise_generator.height(coordinates);
                let humidity = noise_generator.humidity(coordinates);
                (*coordinates, Hexagon { texture_type: (TerrainType::Flat, BiomeType::new(height, humidity)), height: (height + 0.4).floor() as u8 })
            })
            .collect();

        Ok(Grid { hexagons })
    }

    pub fn at(&mut self, noise_generator: &NoiseGenerator, area: &[Coordinates]) {
        // TODO move origin and related transformations into noise_generator ?
        let hexagons = area.iter()
            .map(|coordinates| (*coordinates, self.hexagons.remove(coordinates).unwrap_or_else(|| {
                let height = noise_generator.height(coordinates);
                let humidity = noise_generator.humidity(coordinates);
                Hexagon { texture_type: (TerrainType::Flat, BiomeType::new(height, humidity)), height: (height + 0.4).floor() as u8 }
            })))
            .collect();
        self.hexagons = hexagons;
    }

    pub fn draw(&self, printer: &mut Printer, center: Coordinates, textures: &mut Textures, radius: i32) {
        printer.clear();
        for elevation in 0..=4 {
            if elevation > 0 {
                self.hexagons
                    .iter()
                    .filter(|(_, hexagon)| hexagon.height == elevation)
                    .for_each(|(coordinates, _)| {
                        printer.print_shadow(&center, coordinates, elevation);
                    });
            }

            // TODO printing only the upper layer looks nice, but we can have holes if the tile below is too low !
            for r in (center.r - radius)..=(center.r + radius) {
                for minus_q in (-center.q - radius)..=(-center.q + radius) {
                    match Option::Some(Coordinates { q: -minus_q, r })
                        .filter(|coordinates| center.distance_to(coordinates) <= radius)
                        .map(|coordinates| self.hexagons.get(&coordinates)
                            .map(|hexagon| (coordinates, hexagon)))
                        .flatten()
                        .filter(|(_, hexagon)| hexagon.height >= elevation) {
                        None => {}
                        Some((mut coordinates, hexagon)) => {
                            let texture = textures.random_texture(&hexagon.texture_type, &mut coordinates);
                            printer.print_texture(&center, &coordinates, texture, elevation);
                        }
                    }
                }
            }
        }
        printer.present();
    }
}
