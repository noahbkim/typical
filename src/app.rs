extern crate piston_window;

use piston_window::*;
use piston_window::character::*;

use crate::settings::Settings;
use crate::geometry::Geometry;
use crate::metrics::Metrics;

pub struct App {
    window: PistonWindow,
    glyphs: Glyphs,
    settings: Settings,

    // Mechanics
    metrics: Metrics,
    geometry: Geometry,

    // Game
    text: String,
    cursor: usize,
    wrong: bool,
}

impl App {
    pub fn new(settings: Settings) -> App {
        let window: PistonWindow = WindowSettings::new("Typical", [settings.width, settings.height]).build().unwrap();
        let glyphs: Glyphs = Glyphs::new(&settings.font, window.factory.clone(), TextureSettings::new()).unwrap();
        let mut app: App = App {
            window, glyphs, settings,
            metrics: Metrics::new(), geometry: Geometry::new(),
            text: String::new(), cursor: 4, wrong: true
        };
        app.next();
        app
    }

    pub fn run(&mut self) {
        self.window.set_lazy(true);
        let settings: &Settings = &self.settings;
        let geometry: &Geometry = &self.geometry;
        let text: &mut String = &mut self.text;
        let glyphs: &mut Glyphs = &mut self.glyphs;
        let cursor: &mut usize = &mut self.cursor;
        let wrong: &mut bool = &mut self.wrong;

        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |context, graphics| {
                clear(settings.background, graphics);

                let origin = context.transform.trans(
                    (settings.width as f64 - geometry.width) / 2.0,
                    (settings.height as f64 + geometry.height) / 2.0);
                let mut x: f64 = 0.0;

                let mut image = Image::new_color(settings.completed);
                for (i, character) in text.chars().enumerate() {
                    let glyph = glyphs.character(settings.size, character).unwrap();

                    if i == *cursor {
                        let color = if *wrong { settings.wrong } else { settings.active };
                        ellipse(color, [glyph.width() / 2.0 - 2.0, 7.0, 4.0, 4.0], origin.trans(x, 0.0), graphics);
                        image = Image::new_color(color);
                    }

                    image.draw(glyph.texture, &context.draw_state, origin.trans(x, -glyph.top()), graphics);
                    x += glyph.width();

                    if i == *cursor {
                        image = Image::new_color(settings.active);
                    }
                }

                match event.text_args() {
                    Some(text) => {
                        println!("{}", text)
                    },
                    None => {}
                }
            });
        }
    }

    pub fn next(&mut self) {
        self.text.clear();
        self.text.push_str("Hello, world!");
        self.geometry.compute(&mut self.glyphs, self.settings.size, &self.text);
    }

}
