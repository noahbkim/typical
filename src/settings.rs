use piston_window::types::FontSize;

pub struct Settings {
    pub width: u32,
    pub height: u32,
    pub foreground: [f32; 4],
    pub background: [f32; 4],
    pub font: String,
    pub size: FontSize,
}
