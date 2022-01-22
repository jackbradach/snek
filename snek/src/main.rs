use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

// use snek;
// mod snek;
use snek::snek::{SnekDirection, SnekGame};

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
    let mut game = SnekGame::new(32, 24);
    let mut ticks = 0;
    'running: loop {
        const FRAMES_PER_SEC: u32 = 10;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => { game.set_snekdir(SnekDirection::North); },

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => { game.set_snekdir(SnekDirection::East); },

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => { game.set_snekdir(SnekDirection::West); },

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => { game.set_snekdir(SnekDirection::South); },

                _ => {}
            }
        }

        
        canvas.clear();
        game.step();
        game.draw(&mut canvas);
        canvas.present();

        println!("{}", game);

        if game.game_over {
            println!("Game Over!");
            // break 'running;
        }
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMES_PER_SEC));
        // ::std::thread::sleep(Duration::new(0, 500_000_000)); // 1 second delay for debug
        ticks += 1;
    }

    Ok(())
}
