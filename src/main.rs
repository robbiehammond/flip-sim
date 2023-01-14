extern crate sdl2;
mod grid;
mod Grid;

use std::time::Duration;

use Grid::action_grid::phys_system;
use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::{Point, Rect};
use grid::action_grid::{PLAYGROUND_WIDTH, PLAYGROUND_HEIGHT, NUM_HEIGHT_CELLS, NUM_WIDTH_CELLS};
fn main() -> Result<(), String> {
    pub const SHOW_GRIDLIENS: bool = true;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("rust-sdl2 resource-manager demo", PLAYGROUND_WIDTH, PLAYGROUND_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .map_err(|e| e.to_string())?;
    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_target(PixelFormatEnum::RGBA8888, PLAYGROUND_WIDTH, PLAYGROUND_HEIGHT)
        .map_err(|e| e.to_string())?;

    let mut angle = 0.0;

    let mut g = phys_system::new();

    'mainloop: loop {
        g.update();
        for event in sdl_context.event_pump()?.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::Quit { .. } => break 'mainloop,
                _ => {}
            }
        }

        canvas
            .with_texture_canvas(&mut texture, |texture_canvas| {
                texture_canvas.clear();
                texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                for p in g.particles() {
                    texture_canvas.filled_circle(p.pos.0 as i16, p.pos.1 as i16, 20, Color::GREEN);

                }
            })
            .map_err(|e| e.to_string())?;

        if (SHOW_GRIDLIENS) {
            canvas
                .with_texture_canvas(&mut texture, |texture_canvas| {
                    texture_canvas.set_draw_color(Color::RGBA(255, 0, 0, 255));
                    for i in (0..NUM_HEIGHT_CELLS) {
                        for j in (0..NUM_WIDTH_CELLS) {
                            texture_canvas.filled_circle((i * NUM_WIDTH_CELLS) as i16,( j * NUM_HEIGHT_CELLS) as i16, 2, Color::BLUE);
                        }
                    }
                })
                .map_err(|e| e.to_string())?;
        }
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 255));
        let dst = Some(Rect::new(0, 0, PLAYGROUND_WIDTH , PLAYGROUND_HEIGHT));
        canvas.clear();
        canvas.copy_ex(
            &texture,
            None,
            dst,
            angle,
            Some(Point::new(PLAYGROUND_WIDTH as i32 / 2, PLAYGROUND_HEIGHT as i32 / 2)),
            false,
            false,
        )?;
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

    }

    Ok(())
}