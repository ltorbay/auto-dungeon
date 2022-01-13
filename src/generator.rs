use noise::{Fbm, MultiFractal, Multiply, Negate, Perlin, ScaleBias, Terrace, Turbulence, utils::*, Value, NoiseFn};

pub struct NoiseGenerator {
    width: usize,
    height: usize,
    height_source_module: Box<dyn NoiseFn<f64, 2>>,
    humidity_source_module: Box<dyn NoiseFn<f64, 2>>,
}

impl NoiseGenerator {
    pub fn new(seed: u32, humidity_scale: f64, humidity_bias: f64) -> NoiseGenerator {
        println!("Generating new noise map with humidity scale {} and bias {}", humidity_scale, humidity_bias);

        let scaled_height = NoiseGenerator::get_height_noise_function(seed);

        // let height_map = PlaneMapBuilder::new(&scaled_height)
        //     .set_size(width, height)
        //     .set_x_bounds(0., 8.)
        //     .set_y_bounds(0., 8.)
        //     .build();

        // height_map.write_to_file("height_map.png");

        let base_humidity = Fbm::<Perlin>::new(seed + 1)
            .set_frequency(0.25)
            .set_persistence(0.5)
            .set_lacunarity(2.208984375)
            .set_octaves(2);

        let inverted_height = Negate::new(NoiseGenerator::get_height_noise_function(seed));
        let multiply = Multiply::new(inverted_height, base_humidity);
        let scaled_humidity = ScaleBias::new(multiply)
            .set_scale(humidity_scale)
            .set_bias(humidity_bias);

        // let humidity_map = PlaneMapBuilder::new(&scaled_humidity)
        //     .set_size(width, height)
        //     .set_x_bounds(0., 8.)
        //     .set_y_bounds(0., 8.)
        //     .build();
        // humidity_map.write_to_file("humidity_map.png");


        NoiseGenerator {
            width : 1024,
            height : 1024,
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
            .set_scale(1.15)
            .set_bias(0.25);
        Box::new(scaled_height)
    }

    pub fn height(&self, origin: &(i32, i32), x: i32, y: i32) -> f64 {
        // TODO limit with width and height
        // TODO normalize query bounds
        // self.height_map.get_value(((x + origin.0) / 2) as usize, ((y + origin.1) / 2) as usize)

        let normalized_x = (x + origin.0) as f64 / 2.;
        let normalized_y = (y + origin.1) as f64 / 2.;
        self.height_source_module.get([normalized_x / 128., normalized_y / 128.])
    }

    pub fn humidity(&self, origin: &(i32, i32), x: i32, y: i32) -> f64 {
        // TODO
        // self.humidity_map.get_value(((x + origin.0) / 2) as usize, ((y + origin.1) / 2) as usize)

        let normalized_x = (x + origin.0) as f64 / 2.;
        let normalized_y = (y + origin.1) as f64 / 2.;
        self.humidity_source_module.get([normalized_x / 128., normalized_y / 128.])
    }
}

