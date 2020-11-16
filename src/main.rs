// https://lib.rs/crates/x11-clipboard
// clipboard_simple
//
// use orbtk::prelude::*;
// emoji / kaomoji / textgen
// keymap to chars
// emoji/unicode stuff
//
// copy to clipboard and exit

#![feature(once_cell)]
use gtk::prelude::*;
use gio::prelude::*;

mod emoji;
mod clone;

use gtk::{
    Application,
    Window,
    WindowType,
    WindowPosition,
    Notebook,
    Entry,
    Button,
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
    let output_wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);

    let output = Entry::new();
    output.set_has_frame(false);
    output.set_hexpand(true);

    let copy = Button::with_label("clipboard & exit");
    copy.connect_button_press_event(clone!(output => move |_,_| {
        let clipboard = gtk::Clipboard::get(&gdk::SELECTION_CLIPBOARD);
        clipboard.set_text(&output.get_text());
        Inhibit(false)
    }));

    let clear = Button::with_label("clear");
    clear.connect_button_press_event(clone!(output => move |_,_| {
        output.set_text("");
        Inhibit(false)
    }));

    let notebook = Notebook::new();
    notebook.set_vexpand(true);

    emoji::emoji(&notebook, &output);

    wrapper.add(&notebook);
    wrapper.add(&output_wrapper);
    output_wrapper.add(&output);
    output_wrapper.add(&clear);
    output_wrapper.add(&copy);
    window.add(&wrapper);
    window.show_all();

}


pub fn add_text(text: &str, entry: &gtk::Entry) {
    entry.set_text(&format!("{}{}", entry.get_text(), text));
}
