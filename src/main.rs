mod settings;
mod phrase;
mod geometry;
mod dictionary;
mod app;

use settings::Settings;
use app::App;

fn main() {
    App::new(Settings {
        width: 1000,
        height: 50,
        active: [0.0, 0.0, 0.0, 1.0],
        wrong: [1.0, 0.0, 0.0, 1.0],
        completed: [0.5, 0.5, 0.5, 1.0],
        background: [1.0; 4],
        font: String::from("assets/ubuntu.ttf"),
        size: 28,
    }).run();
}
