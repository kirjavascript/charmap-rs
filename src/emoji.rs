// emoji.json taken from: https://github.com/amio/emoji.json

use gtk::prelude::*;
use tinyjson::JsonValue;
use std::lazy::SyncLazy;
use super::{clone, add_text};

static EMOJI_TEXT: &str = include_str!("../res/emoji.json");

static EMOJI: SyncLazy<Vec<JsonValue>> = SyncLazy::new(|| {
    let emoji_json: JsonValue = EMOJI_TEXT.parse().unwrap();
    let emoji: &Vec<_> = emoji_json.get().unwrap();
    emoji.clone()
});

use gtk::{
    Notebook,
    Label,
    Entry,
    Button,
    EventBox,
};

pub fn emoji(notebook: &Notebook, output: &Entry) {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let list = gtk::Box::new(gtk::Orientation::Vertical, 0);

    let adjust: Option<&gtk::Adjustment> = None;
    let scroll = gtk::ScrolledWindow::new(adjust, adjust);
    scroll.set_vexpand(true);

    let entry = Entry::new();
    entry.set_placeholder_text(Some("search emoji.."));

    container.add(&entry);
    scroll.add(&list);
    container.add(&scroll);

    render("".to_string(), &list, output, false);

    entry.connect_property_text_notify(clone!(list, output => move |entry| {
        let text = entry.get_buffer().get_text();
        render(text, &list, &output, false);
    }));

    let label = Label::new(Some("emoji"));
    notebook.append_page(&container, Some(&label));

}

#[derive(Debug)]
struct Emoji {
    glyph: String,
    desc: String,
}

fn search(text: &str) -> Vec<Emoji> {
    let mut emojis = Vec::new();

    for emoji in EMOJI.iter() {
        let glyph: &String = emoji["char"].get().unwrap();
        let desc: &String = emoji["name"].get().unwrap();
        if desc.contains(text) {
            emojis.push(Emoji {
                glyph: glyph.clone(),
                desc: desc.clone(),
            });
        }
    }

    emojis
}

fn render(text: String, list: &gtk::Box, output: &Entry, showmore: bool) {
    list.foreach(|x| list.remove(x));
    let emojis = search(&text);

    for (i, emoji) in emojis.iter().enumerate() {
        let glyph = Label::new(Some(&emoji.glyph));
        let desc = Label::new(Some(&emoji.desc));
        let clickable = EventBox::new();
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        row.set_margin_top(5);
        row.set_margin_bottom(5);
        row.set_halign(gtk::Align::Start);
        row.set_hexpand(true);
        row.add(&glyph);
        row.add(&desc);
        clickable.add(&row);
        list.add(&clickable);
        let cloned_string = emoji.glyph.clone();
        clickable.connect_button_release_event(clone!(output => move |_,_| {
            add_text(&cloned_string, &output);
            Inhibit(false)
        }));
        if !showmore && i > 20 {
            let label = &format!("show {} more", emojis.len() - 20);
            let clicky = Button::with_label(label);
            list.add(&clicky);
            clicky.connect_clicked(clone!(list, output => move |_| {
                render(text.clone(), &list, &output, true);
            }));
            break;
        }
    }
    list.show_all();
}
