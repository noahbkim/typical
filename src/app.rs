extern crate piston_window;

use std::time::Instant;
use piston_window::*;
use piston_window::character::CharacterCache;

use crate::settings::Settings;


pub struct Metrics {
    mistakes: u32,
    start: Instant,
}

pub struct Drawing {
    width: f64,
    height: f64,
}

pub struct App {
    window: PistonWindow,
    glyphs: Glyphs,
    settings: Settings,

    // Mechanics
    metrics: Metrics,
    drawing: Drawing,

    // Game
    text: String,
    cursor: u32,
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
            metrics: Metrics { mistakes: 0, start: Instant::now() },
            drawing: Drawing { width: 0.0, height: 0.0 },
            text: String::new(),
            cursor: 0,
            wrong: false
        };
        app.next();
        app
    }

    pub fn run(&mut self) {
        self.window.set_lazy(true);
        let settings: &Settings = &self.settings;
        let drawing: &Drawing = &self.drawing;
        let text: &mut String = &mut self.text;
        let glyphs: &mut Glyphs = &mut self.glyphs;

        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |context, graphics| {
                clear(settings.background, graphics);

                let transform = context.transform.trans(
                    (settings.width as f64 - drawing.width) / 2.0,
                    (settings.height as f64 + drawing.height) / 2.0);

                text::Text::new_color(settings.foreground, 24).draw(
                    text,
                    glyphs,
                    &context.draw_state,
                    transform,
                    graphics
                ).unwrap();

            });
        }
    }

    pub fn next(&mut self) {
        self.text.clear();
        self.text.push_str("Hello, world!");
        self.compute();
    }

    fn compute(&mut self) {
        self.drawing.height = if self.text.len() > 0 {
             self.glyphs.character(self.settings.size, self.text.chars().next().unwrap()).unwrap().top()
        } else {
            0.0
        };

        self.drawing.width = 0.0;
        for character in self.text.chars() {
            let glyph = self.glyphs.character(self.settings.size, character).unwrap();
            self.drawing.width += glyph.width();
        }
    }
}
