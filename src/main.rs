mod phrase;
mod app;

use app::App;

fn main() {
    let mut app: App = App::new();
    app.run();
}
