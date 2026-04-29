use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchSettings {
    pub search_ignore_dirs_enabled: bool,
    pub search_ignored_dirs: String,
    pub search_min_chars: usize,
    pub search_case_sensitive: bool,
    pub search_whole_word: bool,
    pub search_use_regex: bool,
    pub search_include_hidden: bool,
    pub search_follow_symlinks: bool,
}

impl Default for SearchSettings {
    fn default() -> Self {
        Self {
            search_ignore_dirs_enabled: true,
            search_ignored_dirs: ".git, node_modules, venv, .venv, target, dist, build, .next, .cache, __pycache__, .idea, .vscode".to_string(),
            search_min_chars: 2,
            search_case_sensitive: false,
            search_whole_word: false,
            search_use_regex: false,
            search_include_hidden: false,
            search_follow_symlinks: false,
        }
    }
}

use crate::Settings;
use crate::ui::{setting_card, cozy_row_filtered};

pub fn show_search_settings(settings: &mut Settings, ui: &mut egui::Ui, has_search: bool, query: &str) {
    use theme::CherryBlossomTheme;
    let query = query.to_lowercase();

    setting_card(settings, ui, "Search Behavior", |ui, settings| {
        cozy_row_filtered(
            settings,
            ui,
            has_search,
            &query,
            "Ignore directories",
            "Exclude certain directories from search",
            |ui, settings| {
                ui.checkbox(&mut settings.search_ignore_dirs_enabled, "");
            },
        );
        if settings.search_ignore_dirs_enabled {
            ui.add_space(16.0);
            ui.add(
                egui::Label::new(
                    egui::RichText::new("Ignored patterns")
                        .size(13.0)
                        .color(CherryBlossomTheme::TEXT_PRIMARY())
                ).selectable(false)
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new("Comma-separated, use * for wildcards")
                        .size(11.0)
                        .color(CherryBlossomTheme::TEXT_MUTED())
                ).selectable(false)
            );
            ui.add_space(4.0);
            ui.add(
                egui::TextEdit::multiline(&mut settings.search_ignored_dirs)
                    .desired_rows(3)
                    .desired_width(ui.available_width()),
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new("Examples: .git, node_modules, *venv")
                        .size(11.0)
                        .color(CherryBlossomTheme::TEXT_MUTED())
                ).selectable(false)
            );
            ui.add_space(16.0);
        }
        cozy_row_filtered(
            settings,
            ui,
            has_search,
            &query,
            "Auto-search threshold",
            "Min chars before search triggers",
            |ui, settings| {
                ui.add(
                    egui::DragValue::new(&mut settings.search_min_chars)
                        .speed(1)
                        .range(1..=10),
                );
            },
        );
    });
}
