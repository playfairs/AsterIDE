use crate::theme::CherryBlossomTheme;

#[derive(Clone, Default, Debug)]
pub struct SearchState {
    pub query: String,
    pub replace: String,
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub use_regex: bool,
    pub results: Vec<SearchResult>,
    pub current_result: usize,
}

#[derive(Clone, Debug)]
pub struct SearchResult {
    pub file_path: String,
    pub line: usize,
    pub start_col: usize,
    pub end_col: usize,
    pub line_content: String,
}

pub const CONTEXT_LINES: usize = 5;

impl SearchState {
    pub fn new() -> Self {
        Self::default()
    }

    fn build_regex(&self) -> Option<regex::Regex> {
        if self.query.is_empty() {
            return None;
        }

        let mut pattern = if self.use_regex {
            self.query.clone()
        } else {
            regex::escape(&self.query)
        };

        if self.whole_word {
            pattern = format!("\\b{}\\b", pattern);
        }

        let mut builder = regex::RegexBuilder::new(&pattern);
        builder.case_insensitive(!self.case_sensitive);

        builder.build().ok()
    }

    pub fn find_in_file(&mut self, file_path: &str, lines: &[String]) {
        let regex = match self.build_regex() {
            Some(r) => r,
            None => return,
        };

        for (line_idx, line) in lines.iter().enumerate() {
            for mat in regex.find_iter(line) {
                self.results.push(SearchResult {
                    file_path: file_path.to_string(),
                    line: line_idx + 1,
                    start_col: mat.start() + 1,
                    end_col: mat.end() + 1,
                    line_content: line.clone(),
                });
            }
        }
    }

    pub fn replace_all_in_text(&self, text: &str) -> String {
        let regex = match self.build_regex() {
            Some(r) => r,
            None => return text.to_string(),
        };

        regex.replace_all(text, &self.replace).to_string()
    }
}

pub fn show_search_tab(ui: &mut egui::Ui, state: &mut SearchState, min_chars: usize) {
    ui.vertical(|ui| {
        ui.add_space(10.0);

        ui.horizontal(|ui| {
            ui.set_height(32.0);

            ui.label(
                egui::RichText::new("🔍")
                    .size(16.0)
                    .color(CherryBlossomTheme::TEXT_MUTED)
            );

            ui.add_space(8.0);

            let search_response = ui.add_sized(
                egui::vec2(ui.available_width() - 200.0, 28.0),
                egui::TextEdit::singleline(&mut state.query)
                    .hint_text("Search...")
                    .font(egui::FontId::proportional(14.0))
            );
            
            if search_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                ui.ctx().data_mut(|d| {
                    d.insert_temp(egui::Id::new("global_search_triggered"), true);
                });
            }
            
            if search_response.changed() && state.query.len() >= min_chars {
                ui.ctx().data_mut(|d| {
                    d.insert_temp(egui::Id::new("global_search_triggered"), true);
                });
            }

            ui.add_space(10.0);

            let case_color = if state.case_sensitive {
                CherryBlossomTheme::ACCENT_PINK
            } else {
                CherryBlossomTheme::TEXT_MUTED
            };
            if ui.add(egui::Label::new(
                egui::RichText::new("Aa").color(case_color).strong()
            ).sense(egui::Sense::click())).clicked() {
                state.case_sensitive = !state.case_sensitive;
            }

            let word_color = if state.whole_word {
                CherryBlossomTheme::ACCENT_PINK
            } else {
                CherryBlossomTheme::TEXT_MUTED
            };
            if ui.add(egui::Label::new(
                egui::RichText::new("\\b").color(word_color).monospace()
            ).sense(egui::Sense::click())).clicked() {
                state.whole_word = !state.whole_word;
            }

            let regex_color = if state.use_regex {
                CherryBlossomTheme::ACCENT_PINK
            } else {
                CherryBlossomTheme::TEXT_MUTED
            };
            if ui.add(egui::Label::new(
                egui::RichText::new(".*").color(regex_color).monospace()
            ).sense(egui::Sense::click())).clicked() {
                state.use_regex = !state.use_regex;
            }
        });

        ui.horizontal(|ui| {
            ui.set_height(32.0);
            ui.label(
                egui::RichText::new("↔")
                    .size(16.0)
                    .color(CherryBlossomTheme::TEXT_MUTED)
            );
            ui.add_space(8.0);
            ui.add_sized(
                egui::vec2(ui.available_width() - 100.0, 28.0),
                egui::TextEdit::singleline(&mut state.replace)
                    .hint_text("Replace...")
                    .font(egui::FontId::proportional(14.0))
            );
        });

        ui.add_space(10.0);

        ui.horizontal(|ui| {
            let button_height = 28.0;

            let search_btn = ui.add_sized(
                egui::vec2(80.0, button_height),
                egui::Button::new(
                    egui::RichText::new("Search")
                        .color(CherryBlossomTheme::BG_DARKEST)
                        .strong()
                )
                .fill(CherryBlossomTheme::ACCENT_PINK)
            );
            if search_btn.clicked() {
                ui.ctx().data_mut(|d| {
                    d.insert_temp(egui::Id::new("global_search_triggered"), true);
                });
            }

            ui.add_space(8.0);

            if ui.add_sized(
                egui::vec2(80.0, button_height),
                egui::Button::new("Replace All")
            ).clicked() && !state.replace.is_empty() {
                ui.ctx().data_mut(|d| {
                    d.insert_temp(egui::Id::new("global_replace_all_triggered"), true);
                });
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if !state.results.is_empty() {
                    ui.label(
                        egui::RichText::new(format!("{} matches", state.results.len()))
                            .color(CherryBlossomTheme::TEXT_MUTED)
                            .size(12.0)
                    );
                } else if !state.query.is_empty() {
                    ui.label(
                        egui::RichText::new("No results")
                            .color(CherryBlossomTheme::TEXT_MUTED)
                            .size(12.0)
                    );
                }
            });
        });

        ui.separator();
        ui.add_space(5.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            if state.results.is_empty() {
                if state.query.is_empty() {
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            egui::RichText::new("Enter a search query to find across open files")
                                .color(CherryBlossomTheme::TEXT_MUTED)
                                .size(14.0)
                        );
                    });
                }
            } else {
                let mut current_file: Option<&str> = None;

                for (i, result) in state.results.iter().enumerate() {
                    if current_file != Some(&result.file_path) {
                        ui.add_space(8.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new("📄 ")
                                    .size(12.0)
                            );
                            ui.label(
                                egui::RichText::new(&result.file_path)
                                    .color(CherryBlossomTheme::ACCENT_PINK)
                                    .size(12.0)
                                    .strong()
                            );
                        });
                        ui.separator();
                        current_file = Some(&result.file_path);
                    }

                    let is_current = i == state.current_result;

                    let bg_color = if is_current {
                        CherryBlossomTheme::BG_DARK
                    } else {
                        CherryBlossomTheme::BG_DARKEST
                    };

                    let response = egui::Frame::new()
                        .fill(bg_color)
                        .inner_margin(egui::vec2(8.0, 4.0))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.add_sized(
                                    egui::vec2(50.0, 18.0),
                                    egui::Label::new(
                                        egui::RichText::new(format!("{}", result.line))
                                            .color(CherryBlossomTheme::TEXT_MUTED)
                                            .monospace()
                                            .size(12.0)
                                    )
                                );

                                ui.add_space(8.0);

                                let before = &result.line_content[..result.start_col.saturating_sub(1)];
                                let matched = &result.line_content[
                                    result.start_col.saturating_sub(1)
                                        ..result.end_col.saturating_sub(1).min(result.line_content.len())
                                ];
                                let after = &result.line_content[
                                    result.end_col.saturating_sub(1).min(result.line_content.len())..
                                ];

                                ui.horizontal(|ui| {
                                    ui.monospace(egui::RichText::new(before)
                                        .color(CherryBlossomTheme::TEXT_SECONDARY)
                                        .size(12.0));
                                    ui.monospace(egui::RichText::new(matched)
                                        .color(CherryBlossomTheme::ACCENT_HOT)
                                        .strong()
                                        .size(12.0));
                                    ui.monospace(egui::RichText::new(after)
                                        .color(CherryBlossomTheme::TEXT_SECONDARY)
                                        .size(12.0));
                                });
                            }).response
                        }).response;

                    if response.clicked() {
                        state.current_result = i;
                    }

                    ui.add_space(1.0);
                }
            }
        });
    });
}

pub fn show_search_button(ui: &mut egui::Ui) {
    ui.heading("Search");
    ui.separator();

    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        ui.label(
            egui::RichText::new("🔍")
                .size(32.0)
                .color(CherryBlossomTheme::TEXT_MUTED)
        );
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("Click the search icon above\nor press Ctrl+Shift+F")
                .color(CherryBlossomTheme::TEXT_MUTED)
                .size(11.0)
        );
    });
}