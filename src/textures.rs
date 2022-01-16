use std::collections::HashMap;

use rand::prelude::{IteratorRandom, StdRng};
use rand::SeedableRng;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use tiles::Coordinates;

const TEXTURES_BASE_DIR: &str = "assets/tiles/grid/hexset_grid_";

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum TerrainType {
    Flat,
    Hill,
    Mont,
    OFlat,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum BiomeType {
    Snow,
    WDeep,
    WShallow,
    // TODO make swamp texture greener
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
            (he, _hu) if he < -0.3 => BiomeType::WDeep,
            (he, _hu) if he < 0. => BiomeType::WShallow,
            (he, _hu) if he < 0.05 => BiomeType::Desert,
            (he, _hu) if he > 2.5 => BiomeType::Snow,
            (he, hu) if hu < -0.8 || he > 1.8 => BiomeType::Stone,
            (_he, hu) if hu < -0.5 => BiomeType::Desert,
            (_he, hu) if hu < 0. => BiomeType::Warm,
            (_he, hu) if hu < 0.4 => BiomeType::Temperate,
            (_he, hu) if hu < 0.8 => BiomeType::Boreal,
            _ => BiomeType::Swamp
        }
    }
}

pub struct Textures<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    textures_locations: HashMap<TerrainType, Vec<&'a str>>,
    biomes_locations: HashMap<BiomeType, &'a str>,
    textures_cache: HashMap<(TerrainType, BiomeType), Vec<Texture<'a>>>,
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

        Textures { texture_creator, textures_locations, textures_cache: Default::default(), biomes_locations }
    }

    pub fn random_texture(&mut self, texture_type: &(TerrainType, BiomeType), coordinates: &mut Coordinates) -> &Texture<'a> {
        if !self.textures_cache.contains_key(texture_type) {
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

            self.textures_cache.insert(texture_type.clone(), loaded_textures);
        }
        self.textures_cache.get(texture_type).expect("Unable to fetch texture")
            .iter()
            .choose(&mut StdRng::seed_from_u64(coordinates.quick_hash()))
            .expect("No texture associated with terrain")
    }
}