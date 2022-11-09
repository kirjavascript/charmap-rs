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
            mode: Mode::Misc,
        }
    }
}

// text convert
// custom charcode / char explorer / read_chars
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
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::Vec2::splat(2.0);

                            for i in 0x1f600..0x1f620 {
                                let chr = char::from_u32(i).unwrap().to_string();
                                self.render_char(ui, &chr, &chr);
                            }
                        });
                    });

                },
                Mode::Misc => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            let list = [
                                ("ZWJ", 0x200b),
                                ("BELL", 0x7),
                            ];
                            for (name, index) in list {
                                self.render_char(ui, name, &char::from_u32(index).unwrap().to_string());
                            }

                            for chr in [
                                "🎉", "✴", "✯", "★", "🗲", "✿", "✧", "♥", "❤", "❥", "～", "～́̀", "ツ",
                            ] {
                                self.render_char(ui, &chr, &chr);
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
                if ui.button("quit").clicked() {
                    _frame.close();
                }
                ui.add(egui::TextEdit::singleline(&mut self.input).desired_width(999.0));
            });
        });
    }

}


impl CharMap {
    fn render_char(&mut self, ui: &mut egui::Ui, name: &str, text: &str) {

        let button = egui::Button::new(
            egui::RichText::new(name.to_string()).font(egui::FontId::proportional(45.0)),
        )
            .frame(false);

        let response = ui.add(button).on_hover_ui(|ui| {
            ui.label(format!("U+{:X}", text.chars().next().unwrap() as u32));
        });

        if response.clicked() {
            clipboard::set(text.to_string());
        }
        if response.secondary_clicked() {
            self.input.push_str(&text.to_string());
        }
    }

}
