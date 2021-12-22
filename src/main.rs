extern crate sdl2;

use std::{thread, time};

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;

use tiles::Coordinates;
use tiles::Hexagon;

mod tiles;

const SCREEN_WIDTH: i16 = 800;
const SCREEN_HEIGHT: i16 = 600;
const ORIGIN: (i16, i16) = (SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

fn main() {
    draw().expect("Failed drawing")
}

fn draw() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys
        .window(
            "Auto dungeon",
            SCREEN_WIDTH as u32,
            SCREEN_HEIGHT as u32,
        )
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .build()
        .map_err(|e| e.to_string())?;

    let base_color = pixels::Color::RGB(255, 255, 0);

    let mut r = 0;
    let mut q = 0;

    new_hexagon(&mut canvas, base_color, r, q)?;

    let mut events = sdl_context.event_pump()?;
    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,

                Event::KeyDown { keycode: Option::Some(Keycode::Up), .. } => {
                    r -= 1;
                    new_hexagon(&mut canvas, base_color, r, q)?;
                }

                Event::KeyDown { keycode: Option::Some(Keycode::Down), .. } => {
                    r += 1;
                    new_hexagon(&mut canvas, base_color, r, q)?;
                }

                Event::KeyDown { keycode: Option::Some(Keycode::Left), .. } => {
                    q -= 1;
                    new_hexagon(&mut canvas, base_color, r, q)?;
                }

                Event::KeyDown { keycode: Option::Some(Keycode::Right), .. } => {
                    q += 1;
                    new_hexagon(&mut canvas, base_color, r, q)?;
                }

                _ => {}
            }
        }
        thread::sleep(time::Duration::from_millis(16));
    }

    Ok(())
}

fn new_hexagon(canvas: &mut Canvas<Window>, base_color: Color, r: i16, q: i16) -> Result<(), String> {
    let hexagon = Hexagon::new(ORIGIN, Coordinates { r, q }, 50)?;
    canvas.polygon(&hexagon.x, &hexagon.y, base_color)?;
    canvas.present();
    println!("Created {:?}", hexagon);
    
    Ok(())
}
