use error::Result;
use sdl2::pixels::Color;
use sdl2::surface::Surface;
use sdl2::ttf::{Font, Sdl2TtfContext};
use std::collections::HashMap;

pub struct TextCache<'a, 'b, 'c> {
    cache: HashMap<(String, Color), (Surface<'a>, bool)>,
    font: Font<'b, 'c>,
}

impl<'a, 'b, 'c> TextCache<'a, 'b, 'c> {
    pub fn new(
        context: &'b Sdl2TtfContext,
        font: &str,
        size: u16,
    ) -> Result<TextCache<'a, 'b, 'static>> {
        let font = context.load_font(font, size)?;

        Ok(TextCache {
            cache: HashMap::new(),
            font,
        })
    }

    pub fn get(&mut self, text: &str, color: Color) -> Result<&Surface<'a>> {
        let font = &mut self.font;
        let entry = self.cache
            .entry((text.to_owned(), color))
            .or_insert_with(|| {
                (
                    font.render(text)
                        .solid(color)
                        .expect("Could not render font but we cannot return an error here"),
                    false,
                )
            });
        entry.1 = true;
        Ok(&entry.0)
    }

    pub fn cleanup(&mut self) {
        let mut keys_to_remove = Vec::new();
        for (key, mut value) in &mut self.cache {
            if value.1 {
                value.1 = false
            } else {
                keys_to_remove.push(key.clone());
            }
        }
        for key in keys_to_remove {
            self.cache.remove(&key);
        }
    }
}
