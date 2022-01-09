use noise::{Blend, Curve, Fbm, MultiFractal, Multiply, Negate, Perlin, RidgedMulti, ScaleBias, Seedable, Terrace, Turbulence, utils::*};
use noise::utils::PlaneMapBuilder;

use textures::BiomeType;

pub struct NoiseGenerator {
    width: usize,
    height: usize,
    pub height_map: NoiseMap,
    pub humidity_map: NoiseMap,
}

impl NoiseGenerator {
    pub fn new(seed: u32, humidity_scale: f64, humidity_bias: f64) -> NoiseGenerator {
        println!("Generating new noise map with humidity scale {} and bias {}", humidity_scale, humidity_bias);

        let perlin = Perlin::new();
        perlin.set_seed(seed);

        let width = 1024;
        let height = 1024;

        let height_terrace = Terrace::new(&perlin)
            .add_control_point(-1.)
            .add_control_point(-0.2)
            .add_control_point(0.)
            .add_control_point(0.4)
            .add_control_point(0.8)
            .add_control_point(1.2)
            .add_control_point(2.);

        let scaled_height = ScaleBias::new(&height_terrace)
            .set_scale(1.15)
            .set_bias(0.25);

        let height_map = PlaneMapBuilder::new(&scaled_height)
            .set_size(width, height)
            .set_x_bounds(0., 8.)
            .set_y_bounds(0., 8.)
            .build();

        // height_map.write_to_file("height_map.png");

        let base_humidity = Fbm::new()
            .set_seed(seed + 1)
            .set_frequency(0.25)
            .set_persistence(0.5)
            .set_lacunarity(2.208984375)
            .set_octaves(2);

        let inverted_height = Negate::new(&scaled_height);
        let multiply = Multiply::new(&inverted_height, &base_humidity);
        let scaled_humidity = ScaleBias::new(&multiply)
            .set_scale(humidity_scale)
            .set_bias(humidity_bias);

        let humidity_map = PlaneMapBuilder::new(&scaled_humidity)
            .set_size(width, height)
            .set_x_bounds(0., 8.)
            .set_y_bounds(0., 8.)
            .build();
        // humidity_map.write_to_file("humidity_map.png");

        NoiseGenerator { width, height, height_map, humidity_map }
    }

    pub fn height(&self, origin: &(i32, i32), x: i32, y: i32) -> f64 {
        // TODO limit with width and height
        // TODO normalize query bounds
        self.height_map.get_value(((x + origin.0) / 2) as usize, ((y + origin.1) / 2) as usize)
    }

    pub fn humidity(&self, origin: &(i32, i32), x: i32, y: i32) -> f64 {
        self.humidity_map.get_value(((x + origin.0) / 2) as usize, ((y + origin.1) / 2) as usize)
    }
}
