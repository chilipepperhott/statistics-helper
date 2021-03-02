mod app;
mod functions;

use app::App;

fn main() {
    eframe::run_native(Box::new(App::new()))
}
