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

#[derive(Debug, PartialEq)]
enum Mode {
    Emoji,
    Kaomoji,
    Misc,
}

impl Default for CharMap {
    fn default() -> Self {
        Self {
            input: "".to_string(),
            mode: Mode::Emoji,
        }
    }
}

// custom charcode
// box chars

impl eframe::App for CharMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("mode select").show(ctx, |ui| {
            ui.horizontal(|ui| {
                use Mode::*;
                for mode in [Emoji, Kaomoji, Misc] {
                    let text = format!("{:?}", mode);
                    let color = if self.mode == mode {
                        egui::Color32::from_rgb(1, 93, 130)
                    } else {
                        egui::Color32::DARK_GRAY
                    };

                    if ui.add(egui::Button::new(text).fill(color)).clicked() {
                        self.mode = mode;
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.mode {
                Mode::Emoji => {
                    let font_id = egui::FontId::proportional(45.0);
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::Vec2::splat(2.0);

                            for i in 0x1f600..0x1f620 {
                                let chr = char::from_u32(i).unwrap();

                                let button = egui::Button::new(
                                    egui::RichText::new(chr.to_string()).font(font_id.clone()),
                                )
                                    .frame(false);

                                let tooltip_ui = |ui: &mut egui::Ui| {
                                    ui.label(format!("U+{:X}", chr as u32));
                                };

                                let response = ui.add(button).on_hover_ui(tooltip_ui);

                                if response.clicked() {
                                    clipboard::set(chr.to_string());
                                }
                                if response.double_clicked() {
                                    _frame.close();
                                }

                                if response.secondary_clicked() {
                                    self.input.push_str(&chr.to_string());
                                }

                            }
                        });
                    });

                },
                _ => {},
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
