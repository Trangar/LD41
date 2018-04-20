#![cfg_attr(
    feature = "cargo-clippy",
    warn(
        decimal_literal_representation, enum_glob_use, fallible_impl_from, if_not_else,
        match_same_arms, mut_mut, needless_borrow, option_unwrap_used, range_plus_one,
        result_unwrap_used
    )
)]

extern crate cgmath;
extern crate collision;
extern crate rand;
extern crate sdl2;
extern crate time;
#[macro_use]
extern crate error_chain;

mod error;

use error::{Error, Result, ResultExt};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

const FRAME_INTERVAL: u64 = 1_000_000_000 / 60;

fn run() -> Result<()> {
    let sdl_context = sdl2::init()
        .map_err(|e| Error::from(e))
        .chain_err(|| "Could not init SDL2")?;
    let video_subsystem = sdl_context
        .video()
        .map_err(|e| Error::from(e))
        .chain_err(|| "Could not init SDL video subsystem")?;
    let window = video_subsystem
        .window("SDL Game", 800, 600)
        .position_centered()
        .build()
        .chain_err(|| "Could not create window")?;
    let mut canvas = window
        .into_canvas()
        .software()
        .build()
        .chain_err(|| "Could not create window canvas")?;
    let mut event_pump = sdl_context
        .event_pump()
        .map_err(|e| Error::from(e))
        .chain_err(|| "Could not create SDL event pump")?;
    let ttf_context = sdl2::ttf::init().chain_err(|| "Could not init TTF")?;

    let texture_creator = canvas.texture_creator();
    let mut rand = rand::thread_rng();
    let mut i = 0;
    let mut frame = 0;
    let mut last_frame_time = time::precise_time_s();

    'running: loop {
        let target = FRAME_INTERVAL + time::precise_time_ns();

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
        last_frame_time = time::precise_time_s();
        i += 1;
        let end = time::precise_time_ns();
        frame += 1;
        if end < target {
            std::thread::sleep(Duration::from_millis((target - end) / 1_000_000));
        }
        if last_frame_time + 1f64 < time::precise_time_s() {
            last_frame_time = time::precise_time_s();
            canvas
                .window_mut()
                .set_title(&format!("FPS: {}", frame))
                .chain_err(|| "Could not update window title")?;
            frame = 0;
        }
    }
    Ok(())
}

fn main() {
    std::env::set_var("RUST_BACKTRACE", "full");
    if let Err(e) = run() {
        for item in e.iter() {
            println!("{}", item);
            if let Some(cause) = item.cause() {
                if cause.description() != item.description() {
                    println!("  caused by: {}", cause);
                }
            }
        }

        if let Some(backtrace) = e.backtrace() {
            let frames = backtrace.frames();
            for frame in frames.iter() {
                for symbol in frame.symbols().iter() {
                    if let (Some(file), Some(lineno)) = (symbol.filename(), symbol.lineno()) {
                        if file.display().to_string().starts_with("D:\\") {
                            println!("{}:{}", file.display().to_string(), lineno);
                        }
                    }
                }
            }
        }
    }
}
