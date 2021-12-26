extern crate sdl2;

use std::{thread, time};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use tiles::{Coordinates, Grid};
use textures::{Textures, TerrainType};

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
    let grid = Grid::new(origin, GRID_RADIUS, PIXEL_PER_HEXAGON)?;
    
    draw_grid(&mut canvas, &grid, textures.random_texture(TerrainType::Grass), GRID_RADIUS);
    canvas.present();

    let mut events = sdl_context.event_pump()?;
    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown { keycode: Option::Some(Keycode::Escape), .. } => break 'main,

                Event::MouseButtonDown { x, y, .. } => {
                    let coordinates = Coordinates::from_offset(&(x as i16, y as i16), &origin, PIXEL_PER_HEXAGON);
                    match grid.hexagons.get(&coordinates) {
                        Some(hexagon) => {
                            canvas.set_draw_color(COLOR_BLACK);
                            canvas.clear();
                            draw_grid(&mut canvas, &grid, textures.random_texture(TerrainType::Grass), GRID_RADIUS);
                            println!("Creating {:?} for {:?}", hexagon, coordinates);
                            
                            // canvas.polygon(&hexagon.x, &hexagon.y, pixels::Color::RGB(255, 0, 0))
                            //     .expect("Could not create polygon");
                            
                            canvas.copy(textures.random_texture(TerrainType::Hill), None, hexagon.rectangle)?;
                            canvas.present();
                        }
                        None => println!("Area does not match any known hexagon x {} y {} calculated {:?}", x, y, coordinates),
                    }
                }

                _ => {}
            }
        }
        thread::sleep(time::Duration::from_millis(16));
    }

    Ok(())
}

// TODO update texture to texture_creator + TerrainType stored in hexagon
fn draw_grid(canvas: &mut Canvas<Window>, grid: &Grid, texture: &Texture, radius: i16) {
    let center = Coordinates { q: 0, r: 0 };
    for minus_q in -radius..=radius {
        for r in -radius..=radius {
            let coordinates = Coordinates { q: - minus_q, r };
            if center.distance_to(&coordinates) <= radius {
                let hexagon = grid.hexagons.get(&coordinates)
                    .unwrap_or_else(|| panic!("Missing hexagon for drawing {:?}", coordinates));
                canvas.copy(texture, None, hexagon.rectangle)
                    .expect("Could not create texture");
            }
        }
    }
}
