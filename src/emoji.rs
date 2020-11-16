// https://github.com/amio/emoji.json

use gtk::prelude::*;
use tinyjson::JsonValue;
use std::lazy::SyncLazy;

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
};

pub fn emoji(notebook: &Notebook, output: &Entry) {
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let list = gtk::Box::new(gtk::Orientation::Vertical, 10);

    let entry = Entry::new();
    entry.set_placeholder_text(Some("search emoji.."));

    container.add(&entry);
    container.add(&list);

    render("", &list);

    let list = list.clone();
    entry.connect_property_text_notify(move |entry| {
        // clear list
        let text = entry.get_buffer().get_text();
        render(&text, &list);
    });


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

        if emojis.len() > 10 {
            break;
        }
    }

    emojis
}

fn render(text: &str, list: &gtk::Box) {
    list.foreach(|x| list.remove(x));
    let emojis = search(&text);

    for emoji in emojis {
        let glyph = Label::new(Some(&emoji.glyph));
        let desc = Label::new(Some(&emoji.desc));
        let row = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        row.set_halign(gtk::Align::Start);
        row.add(&glyph);
        row.add(&desc);
        list.add(&row);
    }
    list.show_all();
}
