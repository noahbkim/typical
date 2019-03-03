use piston_window::*;
use piston_window::character::*;
use piston_window::types::FontSize;

pub struct Geometry {
    pub width: f64,
    pub height: f64,
}

impl Geometry {
    pub fn new() -> Geometry {
        Geometry {width: 0.0, height: 0.0}
    }

    pub fn compute(&mut self, glyphs: &mut Glyphs, size: FontSize, text: &String) {
        self.height = 0.0;
        self.width = 0.0;
        for character in text.chars() {
            let glyph = glyphs.character(size, character).unwrap();
            self.height = f64::max(self.height, glyph.top());
            self.width += glyph.width();
        }
    }
}
