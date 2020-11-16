use gtk::prelude::*;
// use tinyjson::JsonValue;
// use std::lazy::SyncLazy;
use super::{clone, add_text};

// static EMOJI_TEXT: &str = include_str!("../res/emoji.json");

// static EMOJI: SyncLazy<Vec<JsonValue>> = SyncLazy::new(|| {
//     let emoji_json: JsonValue = EMOJI_TEXT.parse().unwrap();
//     let emoji: &Vec<_> = emoji_json.get().unwrap();
//     emoji.clone()
// });

use gtk::{
    Notebook,
    Label,
    Entry,
    Button,
    EventBox,
    ComboBoxText,
};

pub fn textgen(notebook: &Notebook, output: &Entry) {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let entry = Entry::new();

    container.add(&entry);

    // buttons with the text on
    //
    // GLOBAL

    entry.connect_property_text_notify(move |entry| {
    });

    let label = Label::new(Some("textgen"));
    notebook.append_page(&container, Some(&label));

}
