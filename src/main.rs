mod clipboard;
use eframe::egui;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "charmap-rs",
        options,
        Box::new(|_cc| Box::new(CharMap::default())),
    );
}

struct CharMap {
    input: String,
    mode: Mode,
}

enum Mode {
    Emoji,
}

impl Default for CharMap {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            mode: Mode::Emoji,
        }
    }
}

// left clie to add, right click to copy and close

impl eframe::App for CharMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("😀 😃 😄 😁 😆 😅 😂 🤣 🥲 🥹 ☺️ 😊");
            ui.label("ayy");
            ui.code("😀 😃 😄 😁 😆 😅 😂 🤣 🥲 🥹 ☺️ 😊");

            match self.mode {
                Mode::Emoji => {


                },
            }
            ui.text_edit_singleline(&mut self.input);

            ui.horizontal(|ui| {
                if ui.button("copy").clicked() {
                    clipboard::set(self.input.to_owned());
                }
                if ui.button("copy & quit").clicked() {
                    clipboard::set(self.input.to_owned());
                    _frame.close();
                }
            });
        });
    }
}
