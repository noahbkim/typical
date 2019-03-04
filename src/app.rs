extern crate piston_window;

use piston_window::*;
use piston_window::character::*;

use crate::settings::Settings;
use crate::phrase::Phrase;
use crate::dictionary::Dictionary;
use crate::geometry::Geometry;

pub struct App {
    window: PistonWindow,
    glyphs: Glyphs,
    settings: Settings,

    // Mechanics
    phrase: Phrase,
    dictionary: Dictionary,
    geometry: Geometry,
}

impl App {
    pub fn new(settings: Settings) -> App {
        let window: PistonWindow = WindowSettings::new("Typical", [settings.width, settings.height]).build().unwrap();
        let glyphs: Glyphs = Glyphs::new(&settings.font, window.factory.clone(), TextureSettings::new()).unwrap();
        App {
            window,
            glyphs,
            settings,
            phrase: Phrase::new(),
            dictionary: Dictionary::new(),
            geometry: Geometry::new(),
        }
    }

    pub fn run(&mut self) {
        self.window.set_lazy(true);
        while let Some(event) = self.window.next() {
            if let Some(text) = event.text_args() {
                self.phrase.text(text);
            }
            if self.phrase.done() {
                self.next();
            }
            self.draw(&event);
        }
    }

    pub fn next(&mut self) {
        self.phrase.next(String::from("Hello, world!"));
        self.geometry.compute(&mut self.glyphs, self.settings.size, &self.phrase.text);
    }

    pub fn draw(&mut self, event: &Event) {
        self.draw_words(event);
    }

    pub fn draw_words(&mut self, event: &Event) {
        let glyphs: &mut Glyphs = &mut self.glyphs;
        let settings: &Settings = &self.settings;
        let geometry: &Geometry = &self.geometry;
        let size: Size = self.window.size();
        let text: &String = &self.phrase.text;
        let cursor: &usize = &self.phrase.cursor;
        let wrong: &bool = &self.phrase.wrong;

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
                    rectangle(color, [0.0, 6.0, glyph.width().round(), 3.0], origin.trans(x.round(), 0.0), graphics);
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
