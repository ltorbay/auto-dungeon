use noise::{Fbm, MultiFractal, Perlin, ScaleBias, Terrace, NoiseFn, Curve};
use divide::Divide;

pub struct NoiseGenerator {
    height_source_module: Box<dyn NoiseFn<f64, 2>>,
    humidity_source_module: Box<dyn NoiseFn<f64, 2>>,
}

impl NoiseGenerator {
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
            height_source_module:  scaled_height,
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

    pub fn height(&self, origin: &(i32, i32), x: i32, y: i32) -> f64 {
        let normalized_x = (x + origin.0) as f64 / 2.;
        let normalized_y = (y + origin.1) as f64 / 2.;
        self.height_source_module.get([normalized_x / 128., normalized_y / 128.])
    }

    pub fn humidity(&self, origin: &(i32, i32), x: i32, y: i32) -> f64 {
        let normalized_x = (x + origin.0) as f64 / 2.;
        let normalized_y = (y + origin.1) as f64 / 2.;
        self.humidity_source_module.get([normalized_x / 128., normalized_y / 128.])
    }
}
