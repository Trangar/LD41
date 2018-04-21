use assets::Assets;
use cookie::CookieEnemy;
use error::{Error, Result, ResultExt};
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::surface::Surface;
use std::rc::Rc;
use textcache::TextCache;
use turret::*;

pub struct GameState {
    pub turrets: Vec<Rc<Turret>>,
    pub turret_projectiles: Vec<TurretProjectile>,
    pub enemies: Vec<CookieEnemy>,
    pub autoclickers: u32,
    pub currency: f64,
    pub cookie_angle: f64,
    pub cookie_size: f64,
}

impl Default for GameState {
    fn default() -> GameState {
        GameState {
            turrets: Vec::new(),
            turret_projectiles: Vec::new(),
            enemies: Vec::new(),
            autoclickers: 0,
            currency: 1000f64,
            cookie_angle: 0f64,
            cookie_size: 1f64,
        }
    }
}

impl GameState {
    pub fn update(&mut self) {
        self.currency += self.incr_per_sec() / 60f64;
        self.cookie_angle += 0.1f64;
        if self.cookie_size < 1f64 {
            self.cookie_size += 0.001f64;
        }
    }

    fn incr_per_sec(&self) -> f64 {
        self.autoclickers as f64
    }

    fn autoclicker_price(&self) -> f64 {
        self.autoclickers as f64 * 10f64 + 100f64
    }

    pub fn click<TPoint: Into<Point>>(&mut self, position: TPoint) {
        let point = position.into();
        if (point - Point::new(200, 300)).magnitude_squared() < 17000f64 * self.cookie_size {
            self.currency += 1f64;
            self.cookie_size -= 0.01f64;
        }
        if self.autoclicker_price() <= self.currency {
            let diff = point - Point::new(200, 525);
            if diff.x().abs() < 100 && diff.y().abs() < 50 {
                self.currency -= self.autoclicker_price();
                self.autoclickers += 1;
            }
        }
    }

    pub fn render(
        &self,
        canvas: &mut WindowCanvas,
        assets: &Assets,
        textcache: &mut TextCache,
    ) -> Result<()> {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas
            .draw_line((400, 0), (400, 600))
            .map_err(Error::from)
            .chain_err(|| "Could not draw line")?;
        {
            let income_text = textcache
                .get(
                    &format!("{} (+{}/s)", self.currency.floor(), self.incr_per_sec()),
                    Color::RGB(255, 255, 255),
                )
                .chain_err(|| "Could not render income text")?;
            canvas
                .draw_centered(&income_text, (200, 100))
                .chain_err(|| "Could not draw income text")?;
        }

        let mut rect: Rect = (
            0,
            0,
            (400f64 * self.cookie_size) as u32,
            (400f64 * self.cookie_size) as u32,
        ).into();
        rect.center_on((200, 300));

        canvas.copy_ex(
            &assets.cookie,
            None,
            Some(rect),
            self.cookie_angle,
            None,
            false,
            false,
        )?;

        canvas.set_draw_color(if self.autoclicker_price() <= self.currency {Color::RGB(255, 255, 255) } else { Color::RGB(125, 125, 125) });
        canvas.fill_rect(Some((100, 500, 200, 50).into()))?;
        {
            let build_autoclicker_text = textcache
                .get(&format!("Build autoclicker ({})", self.autoclicker_price()), Color::RGB(0, 0, 0))
                .chain_err(|| "Could not render build_autoclicker text")?;
            canvas
                .draw_centered(&build_autoclicker_text, (200, 525))
                .chain_err(|| "Could not draw build_autoclicker text")?;
        }
       
        Ok(())
    }
}

pub trait PointExt {
    fn magnitude_squared(&self) -> f64;
}

impl PointExt for Point {
    fn magnitude_squared(&self) -> f64 {
        self.x() as f64 * self.x() as f64 + self.y() as f64 * self.y() as f64
    }
}

pub trait CanvasExt {
    fn draw_centered<TIntoPoint: Into<Point>>(
        &mut self,
        surface: &Surface,
        rect: TIntoPoint,
    ) -> Result<()>;
}

impl CanvasExt for WindowCanvas {
    fn draw_centered<TIntoPoint: Into<Point>>(
        &mut self,
        surface: &Surface,
        point: TIntoPoint,
    ) -> Result<()> {
        let texture_creator = self.texture_creator();
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .chain_err(|| "Could not turn surface into texture (CanvasExt)")?;
        let point = point.into();
        self.copy(
            &texture,
            None,
            Some(Rect::new(
                point.x() - (surface.width() as i32 / 2),
                point.y() - (surface.height() as i32 / 2),
                surface.width(),
                surface.height(),
            )),
        )?;
        Ok(())
    }
}
