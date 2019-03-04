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
        let window: PistonWindow = WindowSettings::new("Typical", [settings.width, settings.height]).exit_on_esc(true).build().unwrap();
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
                    if self.cursor != self.text.len() && text.len() > 0 && text.chars().next().unwrap() == self.text.chars().nth(self.cursor).unwrap() {
                        self.wrong = false;
                        self.cursor += 1;
                    } else {
                        self.wrong = true;
                        self.metrics.mistakes += 1;
                    }
                },
                None => {}
            }
            if event.press_args() == Some(Button::Keyboard(Key::Return)) && self.cursor == self.text.len() {
                self.next();
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
        let settings: &Settings = &self.settings;
        let geometry: &Geometry = &self.geometry;
        let text: &String = &self.text;
        let glyphs: &mut Glyphs = &mut self.glyphs;
        let cursor: &usize = &self.cursor;
        let wrong: &bool = &self.wrong;

        self.window.draw_2d(event, |context, graphics| {
            clear(settings.background, graphics);

            let origin = context.transform.trans(
                (settings.width as f64 - geometry.width) / 2.0,
                (settings.height as f64 + geometry.height) / 2.0 - 3.0);
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
        });
    }
}
