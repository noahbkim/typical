use piston_window::types::FontSize;

pub struct Settings {
    pub width: u32,
    pub height: u32,
    pub active: [f32; 4],
    pub wrong: [f32; 4],
    pub completed: [f32; 4],
    pub background: [f32; 4],
    pub font: String,
    pub size: FontSize,
}

impl Settings {
    pub fn load(path: String) {

    }
}
