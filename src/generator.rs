use noise::{Curve, Fbm, MultiFractal, NoiseFn, Perlin, ScaleBias, Terrace};

use divide::Divide;
use tiles::Coordinates;

pub struct NoiseGenerator {
    height_source_module: Box<dyn NoiseFn<f64, 2>>,
    humidity_source_module: Box<dyn NoiseFn<f64, 2>>,
}

impl NoiseGenerator {
    const MAP_CENTER: Coordinates = Coordinates { q: 0, r: 0 };

    pub fn new(seed: u32, humidity_scale: f64, humidity_bias: f64) -> NoiseGenerator {
        println!("Generating new noise map with humidity scale {} and bias {}", humidity_scale, humidity_bias);

        let scaled_height = NoiseGenerator::get_height_noise_function(seed);

        let base_humidity = Fbm::<Perlin>::new(seed + 1)
            .set_frequency(0.10)
            .set_persistence(0.5)
            .set_lacunarity(2.208984375)
            .set_octaves(2);

        let biased_height = ScaleBias::new(NoiseGenerator::get_height_noise_function(seed))
            .set_bias(1.)
            .set_scale(3.);
        let multiply = Divide::new(base_humidity, biased_height);
        let scaled_humidity = ScaleBias::new(multiply)
            .set_scale(humidity_scale)
            .set_bias(humidity_bias);

        NoiseGenerator {
            height_source_module: scaled_height,
            humidity_source_module: Box::new(scaled_humidity),
        }
    }

    fn get_height_noise_function(seed: u32) -> Box<dyn NoiseFn<f64, 2>> {
        let perlin: Perlin = Perlin::new(seed);

        let height_terrace = Terrace::<f64, Perlin, 2>::new(perlin)
            .add_control_point(-1.)
            .add_control_point(-0.2)
            .add_control_point(0.)
            .add_control_point(0.4)
            .add_control_point(0.8)
            .add_control_point(1.2)
            .add_control_point(2.);

        let scaled_height = ScaleBias::new(height_terrace)
            .set_scale(2.5)
            .set_bias(0.5);
        let curved_height = Curve::new(scaled_height)
            .add_control_point(-3., -3.)
            .add_control_point(-2.5, -2.)
            .add_control_point(-2., -0.5)
            .add_control_point(-1., -0.1)
            .add_control_point(0., 0.2)
            .add_control_point(1., 0.4)
            .add_control_point(2., 1.5)
            .add_control_point(2.5, 3.)
            .add_control_point(3., 5.);
        // TODO rivers
        Box::new(curved_height)
    }

    pub fn height(&self, coordinates: &Coordinates) -> f64 {
        let (x,y) = coordinates.as_offset(&NoiseGenerator::MAP_CENTER);
        self.height_source_module.get([-x as f64 / 512., -y as f64 / 512.])
    }

    pub fn humidity(&self, coordinates: &Coordinates) -> f64 {
        let (x,y) = coordinates.as_offset(&NoiseGenerator::MAP_CENTER);
        self.humidity_source_module.get([-x as f64 / 512., -y as f64 / 512.])
    }
}

