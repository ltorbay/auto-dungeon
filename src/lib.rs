extern crate noise;
extern crate rand;
extern crate sdl2;

use std::{thread, time};
use std::error::Error;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use generator::NoiseGenerator;
use renderer::Printer;
use textures::Textures;
use tiles::{Coordinates, Grid};

mod tiles;
mod textures;
mod generator;
mod divide;
mod renderer;

const LOGICAL_SCREEN_WIDTH: u32 = 1792;
const LOGICAL_SCREEN_HEIGHT: u32 = 1120;
const ORIGIN: (i32, i32) = ((LOGICAL_SCREEN_WIDTH / 2) as i32, (LOGICAL_SCREEN_HEIGHT / 2) as i32);

// TODO constants class
pub const PIXEL_PER_HEXAGON: u32 = 15;
pub const FLAT_SIDE_LENGTH: f32 = 32. / 30.;

pub fn run(full_screen: bool, width: u32, height: u32) -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let (display_width, display_height) = video_subsys.display_bounds(0)?.size();
    println!("display {}x{}", display_width, display_height);

    let mut canvas;
    {
        let mut builder = video_subsys.window("Auto dungeon", width, height);
        if full_screen { builder.fullscreen_desktop(); } else { builder.borderless(); }

        canvas = builder.opengl()
            .build()?
            .into_canvas()
            .build()?;
    }

    // TODO Apparently scales the whole screen each time which is highly ineffective, see:
    // https://stackoverflow.com/questions/11043969/how-to-scale-to-resolution-in-sdl
    canvas.set_logical_size(LOGICAL_SCREEN_WIDTH, LOGICAL_SCREEN_HEIGHT)
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut textures = Textures::new(&texture_creator);

    const GRID_RADIUS: i32 = 25;

    let mut humidity_scale = 0.97;
    let mut humidity_bias = 0.1;

    let mut noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
    let mut center_coordinates = Coordinates { q: 0, r: 0 };
    let mut area = Coordinates::build_hexagonal_area(center_coordinates, GRID_RADIUS);
    let mut grid = Grid::new(&noise_generator, &area)?;

    let mut printer = Printer::new(&mut canvas, ORIGIN, PIXEL_PER_HEXAGON);
    grid.draw(&mut printer, center_coordinates, &mut textures, GRID_RADIUS);

    let mut pristine = true;
    let mut events = sdl_context.event_pump()?;
    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } => break 'main,

                // TODO more generic way of handling inputs
                Event::KeyDown { keycode: Option::Some(Keycode::Left), .. } => {
                    center_coordinates = center_coordinates.shift(-2, 0);
                    area = Coordinates::build_hexagonal_area(center_coordinates, GRID_RADIUS);
                    grid.at(&noise_generator, &area);
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Right), .. } => {
                    center_coordinates = center_coordinates.shift(2, 0);
                    area = Coordinates::build_hexagonal_area(center_coordinates, GRID_RADIUS);
                    grid.at(&noise_generator, &area);
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Up), .. } => {
                    center_coordinates = center_coordinates.shift(1, -2);
                    area = Coordinates::build_hexagonal_area(center_coordinates, GRID_RADIUS);
                    grid.at(&noise_generator, &area);
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Down), .. } => {
                    center_coordinates = center_coordinates.shift(-1, 2);
                    area = Coordinates::build_hexagonal_area(center_coordinates, GRID_RADIUS);
                    grid.at(&noise_generator, &area);
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::P), .. } => {
                    humidity_bias += 0.1;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    area = Coordinates::build_hexagonal_area(center_coordinates, GRID_RADIUS);
                    grid = Grid::new(&noise_generator, &area)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::M), .. } => {
                    humidity_bias -= 0.1;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    area = Coordinates::build_hexagonal_area(center_coordinates, GRID_RADIUS);
                    grid = Grid::new(&noise_generator, &area)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::O), .. } => {
                    humidity_scale += 0.01;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    area = Coordinates::build_hexagonal_area(center_coordinates, GRID_RADIUS);
                    grid = Grid::new(&noise_generator, &area)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::L), .. } => {
                    humidity_scale -= 0.01;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    area = Coordinates::build_hexagonal_area(center_coordinates, GRID_RADIUS);
                    grid = Grid::new(&noise_generator, &area)?;
                    pristine = false;
                }
                _ => {}
            }
        }
        if !pristine {
            grid.draw(&mut printer, center_coordinates, &mut textures, GRID_RADIUS);
            pristine = true;
        }
        thread::sleep(time::Duration::from_millis(1024 / 32));
    }

    Ok(())
}