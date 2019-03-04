extern crate piston_window;

use piston_window::*;
use piston_window::character::*;

use crate::settings::Settings;
use crate::geometry::Geometry;
use crate::metrics::Metrics;
use crate::words::Words;

pub struct App {
    window: PistonWindow,
    glyphs: Glyphs,
    settings: Settings,

    // Mechanics
    words: Words,
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
            window,
            glyphs,
            settings,
            words: Words::new(),
            metrics: Metrics::new(),
            geometry: Geometry::new(),
            text: String::new(),
            cursor: 0,
            wrong: false
        };
        app.next();
        app
    }

    pub fn run(&mut self) {
        self.window.set_lazy(true);
        while let Some(event) = self.window.next() {
            match event.text_args() {
                Some(text) => {
                    if text.len() > 0 && text.chars().next().unwrap() == self.text.chars().nth(self.cursor).unwrap() {
                        self.wrong = false;
                        self.cursor += 1;
                        if self.cursor == self.text.len() {
                            self.next();
                        }
                    } else {
                        self.wrong = true;
                        self.metrics.mistakes += 1;
                    }
                },
                None => {}
            }
            self.draw(&event);
        }
    }

    pub fn next(&mut self) {
        self.text.clear();
        self.text.push_str("Hello, world!");
        self.cursor = 0;
        self.wrong = false;
        self.geometry.compute(&mut self.glyphs, self.settings.size, &self.text);
    }

    pub fn draw(&mut self, event: &Event) {
        self.draw_words(event);
    }

    pub fn draw_words(&mut self, event: &Event) {
        let size: Size = self.window.size();
        let settings: &Settings = &self.settings;
        let geometry: &Geometry = &self.geometry;
        let text: &String = &self.text;
        let glyphs: &mut Glyphs = &mut self.glyphs;
        let cursor: &usize = &self.cursor;
        let wrong: &bool = &self.wrong;

        self.window.draw_2d(event, |context, graphics| {
            clear(settings.background, graphics);

            let origin = context.transform.trans(
                (size.width as f64 - geometry.width) / 2.0,
                (size.height as f64 + geometry.height) / 2.0 - 3.0);
            let mut x: f64 = 0.0;

            let mut image = Image::new_color(settings.completed);
            for (i, character) in text.chars().enumerate() {
                let glyph = glyphs.character(settings.size, character).unwrap();

                if i == *cursor {
                    let color = if *wrong { settings.wrong } else { settings.active };
                    ellipse(color, [(glyph.width() / 2.0).round() - 2.0, 7.0, 4.0, 4.0], origin.trans(x.round(), 0.0), graphics);
                    image = Image::new_color(color);
                }

                image.draw(glyph.texture, &context.draw_state, origin.trans(x.round(), -glyph.top().round()), graphics);
                x += glyph.width();

                if i == *cursor {
                    image = Image::new_color(settings.active);
                }
            }
        });
    }
}
