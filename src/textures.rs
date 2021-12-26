use std::collections::HashMap;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;


const TEXTURES_BASE_DIR: &str = "assets/tiles/grid/";

#[derive(Hash, PartialEq, Eq, Clone)]
pub enum TerrainType {
    Grass,
    Hill,
}

pub struct Textures<'a> {
    texture_creator: &'a TextureCreator<WindowContext>,
    textures_locations: HashMap<TerrainType, Vec<&'a str>>,
    textures: HashMap<TerrainType, Vec<Texture<'a>>>,
}

impl<'a> Textures<'a> {
    pub fn new(texture_creator: &'a sdl2::render::TextureCreator<sdl2::video::WindowContext>) -> Textures<'a> {
        let textures_locations = HashMap::from(
            [(TerrainType::Grass, Vec::from(["hexset_grid_temperate_flat_01.png", "hexset_grid_temperate_flat_02.png", "hexset_grid_temperate_flat_03.png"])),
            (TerrainType::Hill, Vec::from(["hexset_grid_temperate_hill_01.png", "hexset_grid_temperate_hill_02.png", "hexset_grid_temperate_hill_03.png"]))]
        );
        Textures { texture_creator, textures_locations, textures: Default::default() }
    }

    pub fn random_texture(&mut self, terrain_type: TerrainType) -> &Texture<'a> {
        if !self.textures.contains_key(&terrain_type) {
            let locations = self.textures_locations.get(&terrain_type).expect("Missing texture type in textures locations");
            let loaded_textures = locations.iter().map(|location| {
                let mut full_texture_path: String = TEXTURES_BASE_DIR.to_owned();
                full_texture_path.push_str(*location);
                let texture: Texture<'a> = self.texture_creator.load_texture(full_texture_path)
                    .expect("Could not load texture");
                texture
            })
                .collect();

            self.textures.insert(terrain_type.clone(), loaded_textures);
        }
        // TODO implement random texture getter from vec
        self.textures.get(&terrain_type).expect("Unable to fetch texture")
            .get(0)
            .expect("No texture associated with terrain")
    }
}