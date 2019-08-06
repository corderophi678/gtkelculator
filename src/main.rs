mod ui;

use crate::ui::{app::App, style::STYLE};

use gtk::*;
use std::process;

fn main() {
    if gtk::init().is_err() {
        eprintln!("Failed to initialize GTK Application");
        process::exit(1);
    }

    let app = App::new();
    let provider = gtk::CssProvider::new();
    provider
        .load_from_data(STYLE.as_bytes())
        .expect("Failed to load CSS");
    gtk::StyleContext::add_provider_for_screen(
        &gdk::Screen::get_default().expect("Error initializing gtk css provider"),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    app.window.show_all();
    gtk::main();
}
