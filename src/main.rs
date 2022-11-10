mod clipboard;
mod convert;
use eframe::egui;
use convert::Convert;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "charmap-rs",
        options,
        Box::new(|cc| {

            let mut fonts = egui::FontDefinitions::default();

            fonts.font_data.insert(
                "unifont".to_owned(),
                egui::FontData::from_static(include_bytes!(
                    "../ttf/unifont-15.0.01.ttf"
                )),
            );

            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "unifont".to_owned());

            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "unifont".to_owned());

            cc.egui_ctx.set_fonts(fonts);

            Box::new(CharMap::default())
        }),
    );
}


struct CharMap {
    mode: Mode,
    last_copied: String,
    convert_type: Convert,
    convert_input: String,
}

impl Default for CharMap {
    fn default() -> Self {
        Self {
            mode: Mode::Kaomoji,
            last_copied: "".to_string(),
            convert_type: Convert::Aesthetic,
            convert_input: "".to_string(),
        }
    }
}


#[derive(Debug, PartialEq)]
enum Mode {
    Emoji,
    Kaomoji,
    Convert,
    Misc,
}

// custom charcode / char explorer / read_chars
// CJK
// https://en.wikipedia.org/wiki/Mathematical_operators_and_symbols_in_Unicode

impl eframe::App for CharMap {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        if ctx.input().pointer.secondary_down() {
            _frame.close();
        }

        egui::TopBottomPanel::top("mode select").show(ctx, |ui| {
            ui.horizontal(|ui| {
                use Mode::*;
                for mode in [Kaomoji, Emoji, Convert, Misc] {
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
        egui::TopBottomPanel::bottom("copy / input").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.last_copied.len() > 0 {
                    ui.add(egui::Label::new(egui::RichText::new(self.last_copied.to_string()).font(egui::FontId::monospace(18.0))));
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

                        // let text_style = egui::TextStyle::Body;

                        // let row_height = ui.text_style_height(&text_style);
                        // // let row_height = ui.spacing().interact_size.y; // if you are adding buttons instead of labels.
                        // let items = 1_000_000;
                        // let row_qty = 15;
                        // egui::ScrollArea::vertical().auto_shrink([false, false]).show_rows(ui, row_height, items / row_qty, |ui, row_range| {
                        //     for row in row_range {
                        //         let index = row * row_qty;
                        //         ui.horizontal(|ui| {
                        //             for i in index..index+row_qty {
                        //                 let text = format!("{}", char::from_u32(i as _).unwrap_or('?').to_string());
                        //                 ui.label(text);

                        //             }
                        //         });
                        //     }
                        // });
                    });

                },
                Mode::Kaomoji => {
                    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.spacing_mut().item_spacing = egui::Vec2::splat(20.0);
                            for kao in [
                                "(◕◡◕✿)",
                                "(◡‿◡✿)",
                                "(◠‿◠✿)",
                                "(◔◡◔✿)",
                                "(~˘▾˘)~",
                                "¯\\_(ツ)_/¯",
                                "(⌐■_■)",
                                "ಠ_ಠ",
                                "ʘ︵ʘ",
                                "(╯°□°) ╯︵ ┻━┻",
                                "ヘ (°□。) ヘ",
                                "( ͡° ͜ʖ ͡°)",
                                "( ˘ ³˘)♥",
                                "(っ⌒‿⌒)っ~",
                            ] {

                                let button = egui::Button::new(
                                    egui::RichText::new(kao.to_string()).font(egui::FontId::monospace(20.0)),
                                )
                                    .frame(false);

                                if ui.add(button).clicked() {
                                    self.copy(kao);
                                }
                            }
                        });
                    });
                },
                Mode::Convert => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let converted = self.convert_type.convert(&self.convert_input);
                        let text = egui::RichText::new(converted.clone()).size(39.0);
                        if ui.add(egui::Button::new(text)).clicked() {
                            self.copy(&converted);
                        }
                        ui.horizontal(|ui| {

                            egui::ComboBox::from_label("")
                                .selected_text(format!("{:?}", self.convert_type))
                                .show_ui(ui, |ui| {
                                    use Convert::*;
                                    for ctype in [Aesthetic, Super, Flip] {
                                        let text = format!("{:?}", &ctype);
                                        ui.selectable_value(&mut self.convert_type, ctype, text);
                                    }
                                });

                            ui.add(egui::TextEdit::singleline(&mut self.convert_input).desired_width(999.0));
                        });
                    });
                },
                Mode::Misc => {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            let list = [
                                ("ZWJ", 0x200b),
                                ("BEL", 0x7),
                            ];
                            for (name, index) in list {
                                self.render_char(ui, name, &char::from_u32(index).unwrap().to_string());
                            }

                            for chr in [
                                "λ", "🎉", "✴", "✯", "★", "🗲", "✿", "✧", "💜", "♥", "❤", "❥", "～", "ツ", "√", "♪", "♫", "♬", "🌴", "﷽",
                            ] {
                                self.render_char(ui, &chr, &chr);
                            }
                        });
                    });
                },
            }
        });
    }

}


impl CharMap {
    fn render_char(&mut self, ui: &mut egui::Ui, name: &str, text: &str) {
        let button = egui::Button::new(
            egui::RichText::new(name.to_string()).font(egui::FontId::monospace(45.0)),
        )
            .frame(false);

        let response = ui.add(button).on_hover_ui(|ui| {
            ui.label(format!("U+{:X}", text.chars().next().unwrap() as u32));
        });

        if response.clicked() {
            self.copy(text);
        }
    }

    fn copy(&mut self, text: &str) {
        clipboard::set(text.to_string());
        self.last_copied = text.to_string();
    }

}
