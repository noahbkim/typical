extern crate piston_window;

use piston_window::*;
use crate::phrase::Phrase;

pub struct App {
    window: PistonWindow,
    phrase: Option<Phrase>
}

impl App {
    pub fn new() -> App {
        let window: PistonWindow = WindowSettings::new("Typical", [600, 100]).build().unwrap();
        App { window, phrase: None }
    }
    pub fn run(&mut self) {
        self.window.set_lazy(true);
        let mut glyphs: Glyphs = Glyphs::new(
            "assets/ubuntu.ttf",
            self.window.factory.clone(),
            TextureSettings::new()
        ).unwrap();
        while let Some(event) = self.window.next() {
            self.window.draw_2d(&event, |context, graphics| {
                clear([0.0, 0.0, 0.0, 1.0], graphics);
                let transform = context.transform.trans(10.0, 100.0);
                text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32).draw(
                    "Hello world!",
                    &mut glyphs,
                    &context.draw_state,
                    transform,
                    graphics
                ).unwrap();
            });
        }
    }
}
