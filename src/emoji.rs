// https://github.com/amio/emoji.json

use gtk::prelude::*;
use tinyjson::JsonValue;

static EMOJI: &str = include_str!("../res/emoji.json");

use gtk::{
    Notebook,
    Label,
    ListBox,
    ListBoxRow,
    Entry,
};

pub fn emoji(notebook: &Notebook, output: &Entry) {
    let emoji_json: JsonValue = EMOJI.parse().unwrap();
    let emoji: &Vec<_> = emoji_json.get().unwrap();

    for item in emoji {
        let c: &String = item["char"].get().unwrap();
        println!("{}", c);

    }

    let listbox = ListBox::new();
    {
        let row = ListBoxRow::new();
        let label = Label::new(Some("egg"));
        let label2 = Label::new(Some("🥚"));
        let wrapper = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        wrapper.add(&label);
        wrapper.add(&label2);
        row.add(&wrapper);
        listbox.add(&row);
    }

    let label = Label::new(Some("emoji"));
    notebook.append_page(&listbox, Some(&label));
}
