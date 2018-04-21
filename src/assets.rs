use error::{Error, Result, ResultExt};
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::render::Texture;

pub struct Assets<'a> {
    pub cookie: Texture<'a>,
}

impl<'a> Assets<'a> {
    pub fn new<T>(creator: &TextureCreator<T>) -> Result<Assets> {
        Ok(Assets {
            cookie: creator.load_texture("assets/cookie.png")
                .map_err(Error::from)
                .chain_err(|| "Could not load assets/cookie.png")?,
        })
    }
}
