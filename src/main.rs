use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

// use snek;
mod snek;
use snek::SnekGame;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    video_subsystem.gl_attr().set_context_profile(sdl2::video::GLProfile::GLES);
    video_subsystem.gl_attr().set_context_major_version(2);
    video_subsystem.gl_attr().set_context_minor_version(0);

    let window = video_subsystem
        .window("Snek!", 1024, 768)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())?;
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;
    // FIXME: dimensions on board should be constant and scale up to canvas resolution.
    // FIXME: for now, they're fixed and we assume we're at 1024x768.
    let mut board = SnekGame::new(32, 24);

    'running: loop {
        const FRAMES_PER_SEC: u32 = 60;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.clear();
        board.step();
        board.draw(&mut canvas);
        canvas.present();
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMES_PER_SEC));
        ::std::thread::sleep(Duration::new(1, 0)); // 1 second delay for debug
    }

    Ok(())
}
