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

// left clie to add, dblclick to copy and close
// custom charcode
// emoji
// kaomoji
// hover charcode info

impl eframe::App for CharMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            match self.mode {
                Mode::Emoji => {

                    //         ui.label((9786..20000).map(|c| char::from_u32(c).unwrap()).collect::<String>());
                    let font_id = egui::FontId::proportional(45.0);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::Vec2::splat(2.0);

                            for chr in (0x1f600..0x1f620).map(|c| char::from_u32(c).unwrap()) {

                                let button = egui::Button::new(
                                    egui::RichText::new(chr.to_string()).font(font_id.clone()),
                                )
                                    .frame(false);

                                let tooltip_ui = |ui: &mut egui::Ui| {
                                    ui.label(
                                        egui::RichText::new(chr.to_string()).font(font_id.clone()),
                                    );
                                    ui.label(format!("{}\nU+{:X}\n\nClick to copy", "name", chr as u32));
                                };

                                if ui.add(button).on_hover_ui(tooltip_ui).clicked() {
                                    clipboard::set(self.input.to_owned());
                                }
                            }
                        });
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
