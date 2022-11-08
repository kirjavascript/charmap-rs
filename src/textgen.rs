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

static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789()";
static AESTHETIC: &str = "ＡＢＣＤＥＦＧＨＩＪＫＬＭＮＯＰＱＲＳＴＵＶＷＸＹＺａｂｃｄｅｆｇｈｉｊｋｌｍｎｏｐｑｒｓｔｕｖｗｘｙｚ０１２３４５６７８９";

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
    let ae_button = Button::with_label("aesthetic");

    container.add(&entry);
    container.add(&ae_button);

    // use hashmap for lookups?

    // buttons with the text on
    //
    // GLOBAL

    entry.connect_property_text_notify(clone!(ae_button => move |entry| {
        ae_button.set_label(&entry.get_text());
    }));

    let label = Label::new(Some("textgen"));
    notebook.append_page(&container, Some(&label));

}
