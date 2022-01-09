extern crate noise;
extern crate rand;
extern crate sdl2;

use std::{thread, time};

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use generator::NoiseGenerator;
use textures::{BiomeType, TerrainType, Textures};
use tiles::{Coordinates, Grid, Hexagon};

mod tiles;
mod textures;
mod generator;

const PIXEL_PER_HEXAGON: i32 = 15;
const HEIGHT_SHIFT: i32 = -26 * PIXEL_PER_HEXAGON / 30;
const SHADOW_SHIFT_X: i32 = PIXEL_PER_HEXAGON / 10;
const SHADOW_SHIFT_Y: i32 = - PIXEL_PER_HEXAGON / 6;


// TODO use only i32 and f32 -> normalize units
fn main() {
    draw().expect("Failed drawing")
}

fn draw() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

    let (screen_width, screen_height) = video_subsys.display_bounds(0)?.size();
    let origin = ((screen_width / 2) as i32, (screen_height / 2) as i32);

    let window = video_subsys
        .window(
            "Auto dungeon",
            screen_width,
            screen_height,
        )
        .fullscreen_desktop()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut textures = Textures::new(&texture_creator);

    const COLOR_BLACK: Color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(COLOR_BLACK);
    canvas.clear();

    const GRID_RADIUS: i32 = 25;

    let mut humidity_scale = 2.4;
    let mut humidity_bias = 0.2;

    let mut noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
    let mut grid = Grid::new(origin, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;

    draw_grid(&mut canvas, &grid, &mut textures, GRID_RADIUS);

    let mut texture_selector = (TerrainType::Hill, BiomeType::Desert);
    let texture_ratio = (PIXEL_PER_HEXAGON as f32 * 2. / 30.).round() as u32;
    let brush_holder_rectangle = Rect::new((screen_width / 16) as i32, (screen_height / 16) as i32, 32 * texture_ratio, 48 * texture_ratio);

    canvas.copy(textures.random_texture(&texture_selector, &mut Coordinates { q: 100, r: 100 }), None, brush_holder_rectangle)
        .expect("Could not create texture");
    canvas.present();

    let mut pristine = true;
    let mut events = sdl_context.event_pump()?;
    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } => break 'main,

                Event::KeyDown { keycode: Option::Some(Keycode::Left), .. } => {
                    texture_selector.0 = texture_selector.0.previous();
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Right), .. } => {
                    texture_selector.0 = texture_selector.0.next();
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Up), .. } => {
                    texture_selector.1 = texture_selector.1.previous();
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Down), .. } => {
                    texture_selector.1 = texture_selector.1.next();
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::P), .. } => {
                    humidity_bias += 0.1;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    grid = Grid::new(origin, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::M), .. } => {
                    humidity_bias -= 0.1;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    grid = Grid::new(origin, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::O), .. } => {
                    humidity_scale += 0.1;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    grid = Grid::new(origin, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::L), .. } => {
                    humidity_scale -= 0.1;
                    noise_generator = NoiseGenerator::new(0, humidity_scale, humidity_bias);
                    grid = Grid::new(origin, &noise_generator, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
                    pristine = false;
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let coordinates = Coordinates::from_offset(&(x, y), &origin, PIXEL_PER_HEXAGON);
                    match grid.hexagons.get_mut(&coordinates) {
                        Some(hexagon) => {
                            hexagon.texture_type = texture_selector.clone();
                            pristine = false;
                        }
                        None => println!("Area does not match any known hexagon x {} y {} calculated {:?}", x, y, coordinates),
                    }
                }

                _ => {}
            }
        }
        if !pristine {
            canvas.set_draw_color(COLOR_BLACK);
            canvas.clear();
            
            draw_grid(&mut canvas, &grid, &mut textures, GRID_RADIUS);
            canvas.copy(textures.random_texture(&texture_selector, &mut Coordinates { q: 100, r: 100 }), None, brush_holder_rectangle)
                .expect("Could not create texture");

            canvas.present();
            pristine = true;
        }
        thread::sleep(time::Duration::from_millis(1024 / 32));
    }

    Ok(())
}

fn draw_grid(canvas: &mut Canvas<Window>, grid: &Grid, textures: &mut Textures, radius: i32) {
    // TODO take center as params, and shift q,r bounds accordingly
    let center = Coordinates { q: 0, r: 0 };
    for elevation in 0..=3 {
        grid.hexagons
            .iter()
            .filter(|(_, hexagon)| elevation > 0 && hexagon.height == elevation)
            .for_each(|(_, hexagon)| canvas.filled_polygon(&hexagon.x.map(|val| (val + SHADOW_SHIFT_X * elevation as i32) as i16),
                                                           &hexagon.y.map(|val| (val + (SHADOW_SHIFT_Y + HEIGHT_SHIFT) * elevation as i32) as i16),
                                                           Color::RGBA(0, 0, 0, 40))
                .expect("Could not create shadow polygon"));
        
        for r in -radius..=radius {
            for minus_q in -radius..=radius {
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
