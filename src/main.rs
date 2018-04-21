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

mod assets;
mod cookie;
mod error;
mod state;
mod turret;
mod textcache;

use error::{Error, Result, ResultExt};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn run() -> Result<()> {
    let sdl_context = sdl2::init()
        .map_err(Error::from)
        .chain_err(|| "Could not init SDL2")?;
    let _image_context = sdl2::image::init(sdl2::image::INIT_PNG)
        .map_err(Error::from)
        .chain_err(|| "Could not init sdl2 image")?;
    let ttf_context = sdl2::ttf::init().chain_err(|| "Could not init TTF")?;

    let video_subsystem = sdl_context
        .video()
        .map_err(Error::from)
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
        .map_err(Error::from)
        .chain_err(|| "Could not create SDL event pump")?;

    let texture_creator = canvas.texture_creator();
    let assets = assets::Assets::new(&texture_creator)
        .chain_err(|| "Could not load assets")?;

    let _rand = rand::thread_rng();
    let mut fps_manager = sdl2::gfx::framerate::FPSManager::new();
    fps_manager.set_framerate(60).map_err(Error::from).chain_err(|| "Could not set framerate")?;
    let mut state = state::GameState::default();
    let mut textcache = textcache::TextCache::new(&ttf_context, "assets/arial.ttf", 18)
        .chain_err(|| "Could not create a text cache")?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::MouseButtonDown { x, y, .. } => state.click((x, y)),
                _ => {}
            }
        }
        state.update();

        {
            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            state.render(&mut canvas, &assets, &mut textcache)
                .chain_err(||"Could not render state")?;
            canvas.present();
        }

        fps_manager.delay();
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
