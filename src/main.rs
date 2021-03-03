mod app;

use app::App;

fn main() {
    eframe::run_native(Box::new(App::new()))
}
