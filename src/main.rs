extern crate rand;
extern crate sdl2;

use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use textures::{BiomeType, TerrainType, Textures};
use tiles::{Coordinates, Grid};

mod tiles;
mod textures;

// TODO use only i32 and f32 -> normalize units
fn main() {
    draw().expect("Failed drawing")
}

fn draw() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;

    let (screen_width, screen_height) = video_subsys.display_bounds(0)?.size();
    let origin = ((screen_width / 2) as i16, (screen_height / 2) as i16);

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

    const PIXEL_PER_HEXAGON: i16 = 60;
    const GRID_RADIUS: i16 = 4;

    let mut grid = Grid::new(origin, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
    draw_grid(&mut canvas, &grid, &mut textures, GRID_RADIUS);

    let mut texture_type = (TerrainType::Hill, BiomeType::Desert);

    let texture_ratio = (PIXEL_PER_HEXAGON as f32 * 2. / 30.).round() as u32;
    let brush_holder_rectangle = Rect::new((screen_width / 16) as i32, (screen_height / 16) as i32, 32 * texture_ratio, 48 * texture_ratio);

    canvas.copy(textures.random_texture(&texture_type, &mut Coordinates { q: 100, r: 100 }), None, brush_holder_rectangle)
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
                    texture_type.0 = texture_type.0.previous();
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Right), .. } => {
                    texture_type.0 = texture_type.0.next();
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Up), .. } => {
                    texture_type.1 = texture_type.1.previous();
                    pristine = false;
                }
                Event::KeyDown { keycode: Option::Some(Keycode::Down), .. } => {
                    texture_type.1 = texture_type.1.next();
                    pristine = false;
                }

                Event::MouseButtonDown { x, y, .. } => {
                    let coordinates = Coordinates::from_offset(&(x as i16, y as i16), &origin, PIXEL_PER_HEXAGON);
                    match grid.hexagons.get_mut(&coordinates) {
                        Some(hexagon) => {
                            hexagon.texture_type = texture_type.clone();
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
            canvas.copy(textures.random_texture(&texture_type, &mut Coordinates { q: 100, r: 100 }), None, brush_holder_rectangle)
                .expect("Could not create texture");

            canvas.present();
            pristine = true;
        }
        thread::sleep(time::Duration::from_millis(16));
    }

    Ok(())
}

fn draw_grid(canvas: &mut Canvas<Window>, grid: &Grid, textures: &mut Textures, radius: i16) {
    let center = Coordinates { q: 0, r: 0 };
    for minus_q in -radius..=radius {
        for r in -radius..=radius {
            let mut coordinates = Coordinates { q: -minus_q, r };
            if center.distance_to(&coordinates) <= radius {
                let hexagon = grid.hexagons.get(&coordinates)
                    .unwrap_or_else(|| panic!("Missing hexagon for drawing {:?}", &coordinates));
                canvas.copy(textures.random_texture(&hexagon.texture_type, &mut coordinates), None, hexagon.rectangle)
                    .expect("Could not create texture");
            }
        }
    }
}
