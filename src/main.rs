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
// custom charcode
// hover charcode info

impl eframe::App for CharMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

                ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(999.0));

            match self.mode {
                Mode::Emoji => {

                    egui::ScrollArea::vertical()
                        .show_viewport(ui, |ui, viewport| {
                            ui.label((9786..20000).map(|c| char::from_u32(c).unwrap()).collect::<String>());
                        });


                },
            }

        });
        egui::TopBottomPanel::bottom("copy / input").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("copy").clicked() {
                    clipboard::set(self.input.to_owned());
                }
                if ui.button("copy & quit").clicked() {
                    clipboard::set(self.input.to_owned());
                    _frame.close();
                }
                ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(999.0));
            });
        });
    }
}
