extern crate image as im;
extern crate piston_window;

use piston_window::*;
use std::sync::{Arc, Mutex};

mod canvas;
mod engine;
mod geometry;
mod scene;
mod object;

use engine::Engine;

const BACKGROUND_C: [f32; 4] = [0.5, 0.5, 0.5, 1.0];
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 800;

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 800;

fn main() {
    println!("+-----------------------------------+");
    println!("| RAY_MARCHER                       |");
    println!("| Version: 0.0.1                    |");
    println!("| Author:  gerrit.proessl@gmail.com |");
    println!("+-----------------------------------+");

    let opengl = OpenGL::V3_2;
    let mut window: PistonWindow = WindowSettings::new("RAY_MARCHER", (WINDOW_WIDTH, WINDOW_HEIGHT))
        .exit_on_esc(true)
        .opengl(opengl)
        .build()
        .unwrap();

    let mut c = im::ImageBuffer::new(CANVAS_WIDTH, CANVAS_HEIGHT);
    let mut texture: G2dTexture = Texture::from_image(
        &mut window.factory,
        &c,
        &TextureSettings::new()
    ).unwrap();
    
    let c = Arc::new(Mutex::new(c));

    let engine = Engine::new(Arc::clone(&c));

    let mut mouse_x: u32 = 0;
    let mut mouse_y: u32 = 0;
    while let Some(e) = window.next() {
        if let Some(_r_a) = e.render_args() {
            texture.update(&mut window.encoder, &(*c.lock().unwrap())).unwrap();
            window.draw_2d(&e, |ctx, gfx| {
                clear(BACKGROUND_C, gfx);
                image(&texture, ctx.transform, gfx);
            });
        }
        if let Some(button) = e.press_args() {
            match button {
                Button::Keyboard(_button) => (),
                Button::Mouse(_button) => {
                    println!("Mouse[{}:{}]", mouse_x, mouse_y);
                }
                _ => (),
            }
        }
        if let Some(pos) = e.mouse_cursor_args() {
            mouse_x = pos[0] as u32;
            mouse_y = pos[1] as u32;
        }
        if let Some(_u_a) = e.update_args() {
            engine.update();
        }
    }

}
