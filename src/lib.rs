extern crate noise;
extern crate rand;
extern crate sdl2;

use std::{thread, time};
use std::error::Error;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use generator::NoiseGenerator;
use textures::Textures;
use tiles::{Coordinates, Grid, Hexagon};

mod tiles;
mod textures;
mod generator;
mod divide;

const LOGICAL_SCREEN_WIDTH: u32 = 1792;
const LOGICAL_SCREEN_HEIGHT: u32 = 1120;
const ORIGIN: (i32, i32) = ((LOGICAL_SCREEN_WIDTH / 2) as i32, (LOGICAL_SCREEN_HEIGHT / 2) as i32);

const PIXEL_PER_HEXAGON: i32 = 15;
const HEIGHT_SHIFT: i32 = -26 * PIXEL_PER_HEXAGON / 30;
const SHADOW_SHIFT_X: i32 = PIXEL_PER_HEXAGON / 10;
const SHADOW_SHIFT_Y: i32 = -PIXEL_PER_HEXAGON / 6;

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

    const COLOR_BLACK: Color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(COLOR_BLACK);
    canvas.clear();

    const GRID_RADIUS: i32 = 25;

    let mut humidity_scale = 0.97;
    let mut humidity_bias = 0.1;

    let mut noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
    let mut center_coordinates = Coordinates { q: 0, r: 0 };
    let mut grid = Grid::new(ORIGIN, center_coordinates, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;

    draw_grid(center_coordinates, &mut canvas, &grid, &mut textures, GRID_RADIUS);
    canvas.present();

    let mut pristine = true;
    let mut events = sdl_context.event_pump()?;
    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } => break 'main,

                Event::KeyDown { keycode: Option::Some(Keycode::Left), .. } => {
                    center_coordinates = center_coordinates.shift(2, 0);
                    grid = Grid::new(ORIGIN, center_coordinates, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Right), .. } => {
                    center_coordinates = center_coordinates.shift(-2, 0);
                    grid = Grid::new(ORIGIN, center_coordinates, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Up), .. } => {
                    center_coordinates = center_coordinates.shift(-1, 2);
                    grid = Grid::new(ORIGIN, center_coordinates, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Down), .. } => {
                    center_coordinates = center_coordinates.shift(1, -2);
                    grid = Grid::new(ORIGIN, center_coordinates, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::P), .. } => {
                    humidity_bias += 0.1;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    grid = Grid::new(ORIGIN, center_coordinates, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::M), .. } => {
                    humidity_bias -= 0.1;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    grid = Grid::new(ORIGIN, center_coordinates, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::O), .. } => {
                    humidity_scale += 0.01;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    grid = Grid::new(ORIGIN, center_coordinates, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::L), .. } => {
                    humidity_scale -= 0.01;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    grid = Grid::new(ORIGIN, center_coordinates, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                _ => {}
            }
        }
        if !pristine {
            canvas.set_draw_color(COLOR_BLACK);
            canvas.clear();
            draw_grid(center_coordinates, &mut canvas, &grid, &mut textures, GRID_RADIUS);
            canvas.present();
            pristine = true;
        }
        thread::sleep(time::Duration::from_millis(1024 / 32));
    }

    Ok(())
}

pub fn draw_grid(center: Coordinates, canvas: &mut Canvas<Window>, grid: &Grid, textures: &mut Textures, radius: i32) {
    for elevation in 0..=4 {
        if elevation > 0 {
            grid.hexagons
                .iter()
                .filter(|(_, hexagon)| hexagon.height == elevation)
                .for_each(|(_, hexagon)| canvas.filled_polygon(&hexagon.x.map(|val| (val + SHADOW_SHIFT_X as i32) as i16),
                                                               &hexagon.y.map(|val| (val + SHADOW_SHIFT_Y + HEIGHT_SHIFT * elevation as i32) as i16),
                                                               Color::RGBA(0, 0, 0, 40))
                    .expect("Could not create shadow polygon"));
        }

        for r in (center.r - radius)..=(center.r + radius) {
            for minus_q in (-center.q - radius)..=(-center.q + radius) {
                match Option::Some(Coordinates { q: -minus_q, r })
                    .filter(|coordinates| center.distance_to(coordinates) <= radius)
                    .map(|coordinates| grid.hexagons.get(&coordinates)
                        .map(|hexagon| (coordinates, hexagon)))
                    .flatten()
                    .filter(|(_, hexagon)| hexagon.height >= elevation) {
                    None => {}
                    Some((mut coordinates, Hexagon { height: 0, rectangle, texture_type, .. })) => {
                        canvas.copy(textures.random_texture(texture_type, &mut coordinates), None, *rectangle)
                            .expect("Could not create texture");
                    }
                    Some((mut coordinates, hexagon)) => {
                        let mut new_rectangle = hexagon.rectangle;
                        new_rectangle.offset(0, elevation as i32 * HEIGHT_SHIFT);
                        canvas.copy(textures.random_texture(&hexagon.texture_type, &mut coordinates), None, new_rectangle)
                            .expect("Could not create texture");
                    }
                }
            }
        }
    }
}
