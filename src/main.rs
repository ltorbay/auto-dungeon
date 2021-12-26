extern crate sdl2;

use std::{thread, time};

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::image::{InitFlag, LoadTexture};

use tiles::{Coordinates, Grid};

mod tiles;

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
    let grass = texture_creator.load_texture("assets/tiles/grid/hexset_grid_temperate_flat_01.png")?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    const PIXEL_PER_HEXAGON: i16 = 60;
    let grid = Grid::new(origin, 4, PIXEL_PER_HEXAGON)?;
    
    let base_color = pixels::Color::RGB(255, 255, 0);
    draw_grid(&mut canvas, &grid, &grass, base_color);
    canvas.present();

    let new_color = pixels::Color::RGB(255, 0, 0);
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
                            canvas.set_draw_color(Color::RGB(0, 0, 0));
                            canvas.clear();
                            draw_grid(&mut canvas, &grid, &grass, base_color);
                            println!("Creating {:?} for {:?}", hexagon, coordinates);
                            canvas.polygon(&hexagon.x, &hexagon.y, new_color)
                                .expect("Could not create polygon");
                            
                            canvas.copy(&grass, None, hexagon.rectangle)?;
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

fn draw_grid(canvas: &mut Canvas<Window>, grid: &Grid, texture: &Texture, base_color: Color) {
    grid.hexagons.iter().for_each(|tuple| {
        canvas.copy(texture, None, tuple.1.rectangle)
            .expect("Could not create texture");
        // canvas.polygon(&tuple.1.x, &tuple.1.y, base_color)
        //     .expect("Could not create polygon");
    });
}

// fn new_hexagon(canvas: &mut Canvas<Window>, base_color: Color, r: i16, q: i16) -> Result<(), String> {
//     let hexagon = Hexagon::new(ORIGIN, &Coordinates { r, q }, 50);
//     canvas.polygon(&hexagon.x, &hexagon.y, base_color)?;
//     canvas.present();
//     println!("Created {:?}", hexagon);
// 
//     Ok(())
// }
