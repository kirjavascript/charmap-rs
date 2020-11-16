// https://lib.rs/crates/x11-clipboard
// clipboard_simple
//
// use orbtk::prelude::*;
// emoji / kaomoji / textgen
// keymap to chars
// emoji/unicode stuff

#![feature(once_cell)]
use gtk::prelude::*;
use gio::prelude::*;

mod emoji;

use gtk::{
    Application,
    Window,
    WindowType,
    WindowPosition,
    Notebook,
    Entry,
};

fn main() {
    let application = Application::new(
        Some("com.kirjava.charmap-rs"),
        gio::ApplicationFlags::empty(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(ui);
    application.run(&[]);

    if application.get_is_remote() {
        eprintln!("already running");
    }
}

fn ui (app: &Application) {
    let window = Window::new(WindowType::Toplevel);
    app.add_window(&window);
    window.set_title("charmap-rs");
    window.set_default_size(640, 480);
    window.set_position(WindowPosition::CenterAlways);
    window.set_type_hint(gdk::WindowTypeHint::Dialog);

    let wrapper = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let output = Entry::new();
    output.set_has_frame(false);

    let notebook = Notebook::new();
    notebook.set_vexpand(true);

    emoji::emoji(&notebook, &output);

    wrapper.add(&notebook);
    wrapper.add(&output);
    window.add(&wrapper);
    window.show_all();
}
