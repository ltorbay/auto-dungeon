use sdl2::rect::{Point, Rect};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::gfx::primitives::DrawRenderer;

use tiles::Coordinates;
use sdl2::pixels::Color;
use ::{PIXEL_PER_HEXAGON, FLAT_SIDE_LENGTH};

pub struct Printer<'a> {
    canvas: &'a mut Canvas<Window>,

    origin: (i32, i32),
    texture_ratio: u32,
    tile_center_offset_pixel: f32,

    shadow_x_template: [i32; 6],
    shadow_y_template: [i32; 6],
}

impl<'a> Printer<'a> {
    const X_TEMPLATE: [f32; 6] = [0., FLAT_SIDE_LENGTH, FLAT_SIDE_LENGTH, 0., -FLAT_SIDE_LENGTH, -FLAT_SIDE_LENGTH];
    const Y_TEMPLATE: [f32; 6] = [1., 0.5, -0.5, -1., -0.5, 0.5];

    const COLOR_BLACK: Color = Color::RGB(0, 0, 0);
    const COLOR_SHADOW: Color = Color::RGBA(0, 0, 0, 40);

    const PRINT_RECTANGLE_TEXTURE_WIDTH: u32 = 32;
    const PRINT_RECTANGLE_TEXTURE_HEIGHT: u32 = 48;

    const HEIGHT_SHIFT: i32 = -26 * PIXEL_PER_HEXAGON as i32 / 30;
    const SHADOW_SHIFT_X: i32 = PIXEL_PER_HEXAGON as i32 / 10;
    const SHADOW_SHIFT_Y: i32 = -(PIXEL_PER_HEXAGON as i32) / 6;

    pub fn new(canvas: &'a mut Canvas<Window>, origin: (i32, i32), pixel_per_hexagon: u32) -> Printer<'a> {
        let texture_ratio = (pixel_per_hexagon as f32 * 2. / 30.).round() as u32;
        let shadow_x_template = Printer::X_TEMPLATE.map(|f| (f * pixel_per_hexagon as f32).round() as i32);
        let shadow_y_template = Printer::Y_TEMPLATE.map(|f| (f * pixel_per_hexagon as f32).round() as i32);
        let tile_center_offset = (48. - 30.) / 2.;
        let pixel_ratio = pixel_per_hexagon as f32 / 30.;
        let tile_center_offset_pixel = tile_center_offset * pixel_ratio;

        Printer { canvas, origin, texture_ratio, tile_center_offset_pixel, shadow_x_template, shadow_y_template }
    }
    
    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Printer::COLOR_BLACK);
        self.canvas.clear();
    }
    
    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn print_texture(&mut self, screen_center: &Coordinates, coordinates: &Coordinates, texture: &Texture, elevation: u8) {
        let (x_offset, y_offset) = coordinates.as_offset(screen_center);
        let center = Point::new((self.origin.0 + x_offset) as i32, self.origin.1 + y_offset + self.tile_center_offset_pixel.round() as i32 + elevation as i32 * Printer::HEIGHT_SHIFT);
        let texture_destination = Rect::from_center(center,
                                                    Printer::PRINT_RECTANGLE_TEXTURE_WIDTH * self.texture_ratio,
                                                    Printer::PRINT_RECTANGLE_TEXTURE_HEIGHT * self.texture_ratio);
        self.canvas.copy(texture, None, texture_destination)
            .expect("Could not create texture");
    }

    pub fn print_shadow(&mut self, screen_center: &Coordinates, coordinates: &Coordinates, elevation: u8) {
        let (x_offset, y_offset) = coordinates.as_offset(screen_center);
        let x_template_shift = self.origin.0 + x_offset + Printer::SHADOW_SHIFT_X as i32;
        let y_template_shift = self.origin.1 + y_offset + Printer::SHADOW_SHIFT_Y as i32 + elevation as i32 * Printer::HEIGHT_SHIFT;
        
        self.canvas.filled_polygon(&self.shadow_x_template.map(|val| (val + x_template_shift) as i16),
                                   &self.shadow_y_template.map(|val| (val + y_template_shift) as i16),
                                   Printer::COLOR_SHADOW)
            .expect("Could not create shadow polygon")
    }
}