use std::collections::HashMap;

use rand::prelude::{IteratorRandom, StdRng};
use rand::SeedableRng;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use tiles::Coordinates;
use generator::NoiseGenerator;

const TEXTURES_BASE_DIR: &str = "assets/tiles/grid/hexset_grid_";

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum TerrainType {
    Flat,
    Hill,
    Mont,
    OFlat,
}

impl TerrainType {
    pub fn next(&self) -> TerrainType {
        match self {
            TerrainType::Flat => TerrainType::Hill,
            TerrainType::Hill => TerrainType::Mont,
            TerrainType::Mont => TerrainType::OFlat,
            TerrainType::OFlat => TerrainType::Flat,
        }
    }

    pub fn previous(&self) -> TerrainType {
        match self {
            TerrainType::Flat => TerrainType::OFlat,
            TerrainType::Hill => TerrainType::Flat,
            TerrainType::Mont => TerrainType::Hill,
            TerrainType::OFlat => TerrainType::Mont,
        }
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum BiomeType {
    Snow,
    WDeep,
    WShallow,
    Swamp,
    Boreal,
    Temperate,
    Warm,
    Desert,
    Stone,
}

impl BiomeType {
    pub fn new(height: f64, humidity: f64) -> BiomeType {
        match (height, humidity) {
            (he, _hu) if he < -0.4 => BiomeType::WDeep,
            (he, _hu) if he < 0. => BiomeType::WShallow,
            (he, _hu) if he < 0.05 => BiomeType::Desert,
            (he, _hu) if he > 2. => BiomeType::Snow,
            (he, hu) if hu < -0.8 || he > 1.8 => BiomeType::Stone,
            (_he, hu) if hu < -0.5 => BiomeType::Desert,
            (_he, hu) if hu < 0. => BiomeType::Warm,
            (_he, hu) if hu < 0.4 => BiomeType::Temperate,
            (_he, hu) if hu < 0.9 => BiomeType::Boreal,
            _ => BiomeType::Swamp
        }
    }

    pub fn next(&self) -> BiomeType {
        match self {
            BiomeType::Snow => BiomeType::WDeep,
            BiomeType::WDeep => BiomeType::WShallow,
            BiomeType::WShallow => BiomeType::Swamp,
            BiomeType::Swamp => BiomeType::Boreal,
            BiomeType::Boreal => BiomeType::Temperate,
            BiomeType::Temperate => BiomeType::Warm,
            BiomeType::Warm => BiomeType::Desert,
            BiomeType::Desert => BiomeType::Stone,
            BiomeType::Stone => BiomeType::Snow,
        }
    }

    pub fn previous(&self) -> BiomeType {
        match self {
            BiomeType::Snow => BiomeType::Stone,
            BiomeType::WDeep => BiomeType::Snow,
            BiomeType::WShallow => BiomeType::WDeep,
            BiomeType::Swamp => BiomeType::WShallow,
            BiomeType::Boreal => BiomeType::Swamp,
            BiomeType::Temperate => BiomeType::Boreal,
            BiomeType::Warm => BiomeType::Temperate,
            BiomeType::Desert => BiomeType::Warm,
            BiomeType::Stone => BiomeType::Desert,
        }
    }
}

pub struct Textures<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    textures_locations: HashMap<TerrainType, Vec<&'a str>>,
    biomes_locations: HashMap<BiomeType, &'a str>,
    textures: HashMap<(TerrainType, BiomeType), Vec<Texture<'a>>>,
}

impl<'a> Textures<'a> {
    pub fn new(texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Textures<'a> {
        let textures_locations = HashMap::from(
            [(TerrainType::Flat, Vec::from(["flat_01.png", "flat_02.png", "flat_03.png"])),
                (TerrainType::Hill, Vec::from(["hill_01.png", "hill_02.png", "hill_03.png"])),
                (TerrainType::Mont, Vec::from(["mont_01.png", "mont_02.png", "mont_03.png"])),
                (TerrainType::OFlat, Vec::from(["O_flat_01.png", "O_flat_02.png", "O_flat_03.png"]))]
        );
        let biomes_locations = HashMap::from(
            [(BiomeType::Boreal, "boreal_"),
                (BiomeType::Desert, "desert_"),
                (BiomeType::Snow, "snow_"),
                (BiomeType::Stone, "stone1_"),
                (BiomeType::Swamp, "swamp_"),
                (BiomeType::Temperate, "temperate_"),
                (BiomeType::Warm, "warm_"),
                (BiomeType::WDeep, "wdeep_"),
                (BiomeType::WShallow, "wshallow_"), ]
        );

        Textures { texture_creator, textures_locations, textures: Default::default(), biomes_locations }
    }

    pub fn random_texture(&mut self, texture_type: &(TerrainType, BiomeType), coordinates: &mut Coordinates) -> &Texture<'a> {
        if !self.textures.contains_key(texture_type) {
            let locations = self.textures_locations.get(&texture_type.0).expect("Missing texture type in textures locations");
            let loaded_textures = locations.iter().map(|location| {
                let mut full_texture_path: String = TEXTURES_BASE_DIR.to_owned();
                full_texture_path.push_str(self.biomes_locations.get(&texture_type.1).expect("Missing biome type in biome locations"));
                full_texture_path.push_str(*location);
                let texture: Texture<'a> = self.texture_creator.load_texture(full_texture_path)
                    .expect("Could not load texture");
                texture
            })
                .collect();

            self.textures.insert(texture_type.clone(), loaded_textures);
        }
        self.textures.get(texture_type).expect("Unable to fetch texture")
            .iter()
            .choose(&mut StdRng::seed_from_u64(coordinates.quick_hash()))
            .expect("No texture associated with terrain")
    }
}