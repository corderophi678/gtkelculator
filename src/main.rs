mod ui;

use crate::ui::app::App;

use gtk::*;
use std::process;

fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK Application");
        process::exit(1);
    }

    let app = App::new();

    app.window.show_all();
    gtk::main();
}
