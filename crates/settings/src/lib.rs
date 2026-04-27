use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum SettingsCategory {
    Editor,
    Appearance,
    Workbench,
    Search,
    About,
}

impl Default for SettingsCategory {
    fn default() -> Self {
        SettingsCategory::Editor
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Settings {
    pub show_line_numbers: bool,
    pub word_wrap: bool,
    pub font_size: f32,
    pub tab_size: usize,
    pub use_spaces: bool,
    pub show_whitespace: bool,
    pub vim_mode: bool,
    pub auto_save: bool,
    pub auto_save_interval: u64,
    pub sidebar_visible: bool,
    pub status_bar_visible: bool,
    pub search_ignore_dirs_enabled: bool,
    pub search_ignored_dirs: String,
    pub search_min_chars: usize,
    pub highlight_current_line: bool,
    pub auto_indent: bool,
    pub scroll_beyond_last_line: bool,
    pub minimap: bool,
    pub recent_files_limit: usize,
    pub recent_projects_limit: usize,
    pub corner_roundness: f32,
    pub theme_family: theme::ThemeFamily,
    pub theme_variant: theme::ThemeVariant,
    pub pinned_files: Vec<std::path::PathBuf>,
    #[serde(skip)]
    pub selected_category: SettingsCategory,
    #[serde(skip)]
    pub search_query: String,
    #[serde(skip)]
    pub edit_as_json_clicked: bool,
    #[serde(skip)]
    pub apply_changes_clicked: bool,
    #[serde(skip)]
    pub confirm_discard_open: bool,
    #[serde(skip)]
    pub close_after_discard: bool,
    #[serde(skip)]
    pub pending_file_open: Option<(std::path::PathBuf, String)>,
    #[serde(skip)]
    pub pending_new_tab: bool,
    #[serde(skip)]
    saved_state: Option<Box<Settings>>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            show_line_numbers: true,
            word_wrap: true,
            font_size: 14.0,
            tab_size: 4,
            use_spaces: true,
            show_whitespace: false,
            vim_mode: false,
            auto_save: false,
            auto_save_interval: 30,
            sidebar_visible: true,
            status_bar_visible: true,
            search_ignore_dirs_enabled: true,
            search_ignored_dirs: ".git, node_modules, venv, .venv, target, dist, build, .next, .cache, __pycache__, .idea, .vscode".to_string(),
            search_min_chars: 2,
            highlight_current_line: true,
            auto_indent: true,
            scroll_beyond_last_line: true,
            minimap: false,
            recent_files_limit: 5,
            recent_projects_limit: 5,
            corner_roundness: 6.0,
            theme_family: theme::ThemeFamily::CherryBlossom,
            theme_variant: theme::ThemeVariant::CherryBlossomDark,
            pinned_files: Vec::new(),
            selected_category: SettingsCategory::default(),
            search_query: String::new(),
            edit_as_json_clicked: false,
            apply_changes_clicked: false,
            confirm_discard_open: false,
            close_after_discard: false,
            pending_file_open: None,
            pending_new_tab: false,
            saved_state: None,
        }
    }
}

impl SettingsCategory {
    fn name(&self) -> &'static str {
        match self {
            SettingsCategory::Editor => "Editor",
            SettingsCategory::Appearance => "Appearance",
            SettingsCategory::Workbench => "Workbench",
            SettingsCategory::Search => "Search",
            SettingsCategory::About => "About",
        }
    }

    #[allow(dead_code)]
    fn icon(&self) -> &'static str {
        match self {
            SettingsCategory::Editor => "📝",
            SettingsCategory::Appearance => "🎨",
            SettingsCategory::Workbench => "🖥️",
            SettingsCategory::Search => "🔍",
            SettingsCategory::About => "ℹ️",
        }
    }
}

fn config_dir() -> Option<std::path::PathBuf> {
    dirs::config_dir().map(|d| d.join("asteride"))
}

fn settings_file_path() -> Option<std::path::PathBuf> {
    config_dir().map(|d| d.join("settings.json"))
}

pub fn get_settings_file_path() -> Option<std::path::PathBuf> {
    settings_file_path()
}

impl Settings {
    pub fn load() -> Self {
        let mut settings = if let Some(path) = settings_file_path() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(settings) = serde_json::from_str::<Settings>(&content) {
                    settings
                } else {
                    Self::default()
                }
            } else {
                Self::default()
            }
        } else {
            Self::default()
        };
        settings.capture_saved_state();
        settings
    }

    pub fn capture_saved_state(&mut self) {
        let mut saved = self.clone();
        saved.saved_state = None;
        self.saved_state = Some(Box::new(saved));
    }

    pub fn has_unsaved_changes(&self) -> bool {
        if let Some(ref saved) = self.saved_state {
            self.show_line_numbers != saved.show_line_numbers
                || self.word_wrap != saved.word_wrap
                || self.font_size != saved.font_size
                || self.tab_size != saved.tab_size
                || self.use_spaces != saved.use_spaces
                || self.show_whitespace != saved.show_whitespace
                || self.vim_mode != saved.vim_mode
                || self.auto_save != saved.auto_save
                || self.auto_save_interval != saved.auto_save_interval
                || self.sidebar_visible != saved.sidebar_visible
                || self.status_bar_visible != saved.status_bar_visible
                || self.search_ignore_dirs_enabled != saved.search_ignore_dirs_enabled
                || self.search_ignored_dirs != saved.search_ignored_dirs
                || self.search_min_chars != saved.search_min_chars
                || self.highlight_current_line != saved.highlight_current_line
                || self.auto_indent != saved.auto_indent
                || self.scroll_beyond_last_line != saved.scroll_beyond_last_line
                || self.minimap != saved.minimap
                || self.recent_files_limit != saved.recent_files_limit
                || self.recent_projects_limit != saved.recent_projects_limit
                || self.corner_roundness != saved.corner_roundness
                || self.theme_family != saved.theme_family
                || self.theme_variant != saved.theme_variant
                || self.pinned_files != saved.pinned_files
        } else {
            false
        }
    }

    pub fn apply_changes(&mut self) {
        self.save();
        self.capture_saved_state();
    }

    pub fn discard_changes(&mut self) {
        if let Some(ref saved) = self.saved_state {
            self.show_line_numbers = saved.show_line_numbers;
            self.word_wrap = saved.word_wrap;
            self.font_size = saved.font_size;
            self.tab_size = saved.tab_size;
            self.use_spaces = saved.use_spaces;
            self.show_whitespace = saved.show_whitespace;
            self.vim_mode = saved.vim_mode;
            self.auto_save = saved.auto_save;
            self.auto_save_interval = saved.auto_save_interval;
            self.sidebar_visible = saved.sidebar_visible;
            self.status_bar_visible = saved.status_bar_visible;
            self.search_ignore_dirs_enabled = saved.search_ignore_dirs_enabled;
            self.search_ignored_dirs = saved.search_ignored_dirs.clone();
            self.search_min_chars = saved.search_min_chars;
            self.highlight_current_line = saved.highlight_current_line;
            self.auto_indent = saved.auto_indent;
            self.scroll_beyond_last_line = saved.scroll_beyond_last_line;
            self.minimap = saved.minimap;
            self.recent_files_limit = saved.recent_files_limit;
            self.recent_projects_limit = saved.recent_projects_limit;
            self.corner_roundness = saved.corner_roundness;
            self.theme_family = saved.theme_family;
            self.theme_variant = saved.theme_variant;
            self.pinned_files = saved.pinned_files.clone();
        }
    }

    pub fn save(&self) {
        if let Some(path) = settings_file_path() {
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = std::fs::write(&path, json);
            }
        }
    }
}

impl Settings {
    pub fn show_panel(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(true)
            .default_size([600.0, 500.0])
            .min_size([400.0, 300.0])
            .show(ctx, |ui| {
                self.show_content(ui);
            });

        self.show_confirm_discard_dialog(ctx);
    }

    pub fn show_confirm_discard_dialog(&mut self, ctx: &egui::Context) {
        if !self.confirm_discard_open {
            return;
        }

        let screen_rect = ctx.content_rect();

        egui::Area::new(egui::Id::new("settings_dim_overlay"))
            .fixed_pos(screen_rect.min)
            .show(ctx, |ui| {
                ui.painter().rect_filled(
                    screen_rect,
                    0.0,
                    egui::Color32::from_rgba_premultiplied(0, 0, 0, 120),
                );
            });

        let modal_frame = egui::Frame::new()
            .fill(theme::CherryBlossomTheme::BG_DARKEST())
            .corner_radius(12.0)
            .stroke(egui::Stroke::new(1.0, theme::CherryBlossomTheme::BG_LIGHT()))
            .inner_margin(egui::Margin::symmetric(32, 28))
            .shadow(egui::epaint::Shadow {
                offset: [0, 8],
                blur: 16,
                spread: 0,
                color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 80),
            });

        egui::Window::new("")
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .title_bar(false)
            .frame(modal_frame)
            .default_size([340.0, 160.0])
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Unsaved Changes")
                            .size(18.0)
                            .strong()
                            .color(theme::CherryBlossomTheme::TEXT_PRIMARY()),
                    );
                    ui.add_space(12.0);
                    ui.label(
                        egui::RichText::new("You have unsaved settings changes.")
                            .size(14.0)
                            .color(theme::CherryBlossomTheme::TEXT_SECONDARY()),
                    );
                    ui.label(
                        egui::RichText::new("Discard them?")
                            .size(14.0)
                            .color(theme::CherryBlossomTheme::TEXT_SECONDARY()),
                    );
                    ui.add_space(24.0);

                    ui.horizontal(|ui| {
                        ui.add_space(ui.available_width() / 2.0 - 130.0);

                        let discard_btn = ui.add_sized(
                            [120.0, 36.0],
                            egui::Button::new(
                                egui::RichText::new("Discard")
                                    .size(14.0)
                                    .strong()
                                    .color(theme::CherryBlossomTheme::BG_DARKEST()),
                            )
                            .corner_radius(8.0)
                            .fill(theme::CherryBlossomTheme::ACCENT_PINK()),
                        );
                        if discard_btn.clicked() {
                            self.discard_changes();
                            self.confirm_discard_open = false;
                            if self.pending_file_open.is_some() || self.pending_new_tab {
                            } else {
                                self.close_after_discard = true;
                            }
                        }

                        ui.add_space(12.0);

                        let cancel_btn = ui.add_sized(
                            [120.0, 36.0],
                            egui::Button::new(
                                egui::RichText::new("Cancel")
                                    .size(14.0)
                                    .color(theme::CherryBlossomTheme::TEXT_PRIMARY()),
                            )
                            .corner_radius(8.0)
                            .fill(theme::CherryBlossomTheme::BG_MID()),
                        );
                        if cancel_btn.clicked() {
                            self.confirm_discard_open = false;
                            self.pending_file_open = None;
                            self.pending_new_tab = false;
                        }
                    });
                });
            });
    }

    pub fn request_close_with_confirmation(&mut self) -> bool {
        if self.has_unsaved_changes() {
            self.confirm_discard_open = true;
            false
        } else {
            true
        }
    }

    pub fn request_file_open_with_confirmation(
        &mut self,
        path: std::path::PathBuf,
        content: String,
    ) -> bool {
        if self.has_unsaved_changes() {
            self.pending_file_open = Some((path, content));
            self.confirm_discard_open = true;
            false
        } else {
            true
        }
    }

    pub fn take_pending_file_open(&mut self) -> Option<(std::path::PathBuf, String)> {
        self.pending_file_open.take()
    }

    pub fn request_new_tab_with_confirmation(&mut self) -> bool {
        if self.has_unsaved_changes() {
            self.pending_new_tab = true;
            self.confirm_discard_open = true;
            false
        } else {
            true
        }
    }

    pub fn take_pending_new_tab(&mut self) -> bool {
        if self.pending_new_tab {
            self.pending_new_tab = false;
            true
        } else {
            false
        }
    }

    pub fn show_content(&mut self, ui: &mut egui::Ui) {
        use theme::CherryBlossomTheme;

        let available_height = ui.available_height();

        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.set_width(ui.available_width());

            ui.label(
                egui::RichText::new(self.selected_category.name())
                    .size(18.0)
                    .strong()
                    .color(theme::CherryBlossomTheme::TEXT_PRIMARY()),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let btn_height = 28.0;
                let btn_rounding = self.corner_roundness as u8;

                if self.has_unsaved_changes() {
                    let apply_btn = ui.add_sized(
                        [100.0, btn_height],
                        egui::Button::new(
                            egui::RichText::new("Apply Changes")
                                .size(13.0)
                                .strong()
                                .color(theme::CherryBlossomTheme::BG_DARKEST()),
                        )
                        .corner_radius(btn_rounding)
                        .fill(theme::CherryBlossomTheme::ACCENT_PINK()),
                    );
                    if apply_btn.clicked() {
                        self.apply_changes_clicked = true;
                    }
                    ui.add_space(8.0);
                }

                let json_btn = ui.add_sized(
                    [90.0, btn_height],
                    egui::Button::new(
                        egui::RichText::new("Edit as JSON")
                            .size(13.0)
                            .color(theme::CherryBlossomTheme::TEXT_PRIMARY()),
                    )
                    .corner_radius(btn_rounding)
                    .fill(theme::CherryBlossomTheme::BG_MID()),
                );
                if json_btn.clicked() {
                    self.edit_as_json_clicked = true;
                }

                ui.add_space(8.0);

                let reset_btn = ui.add_sized(
                    [95.0, btn_height],
                    egui::Button::new(
                        egui::RichText::new("Reset Settings")
                            .size(13.0)
                            .color(theme::CherryBlossomTheme::TEXT_PRIMARY()),
                    )
                    .corner_radius(btn_rounding)
                    .fill(theme::CherryBlossomTheme::BG_MID()),
                );
                if reset_btn.clicked() {
                    *self = Self::default();
                    self.capture_saved_state();
                }

                ui.add_space(8.0);

                let total_count = self.count_settings();
                let matches = if self.search_query.is_empty() {
                    total_count
                } else {
                    self.count_matching_settings(&self.search_query)
                };
                ui.label(
                    egui::RichText::new(format!("{} / {} Settings", matches, total_count))
                        .size(12.0)
                        .color(theme::CherryBlossomTheme::TEXT_MUTED()),
                );

                ui.add_space(16.0);

                ui.add(
                    egui::TextEdit::singleline(&mut self.search_query)
                        .hint_text("Search settings...")
                        .desired_width(180.0),
                );
            });
        });
        ui.separator();

        let content_height = available_height - ui.cursor().top();

        ui.horizontal(|ui| {
            ui.add_space(6.0);

            let sidebar_width = 140.0;
            let item_height = 36.0;
            let corner_radius = 8.0;

            ui.allocate_ui_with_layout(
                egui::vec2(sidebar_width, content_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(sidebar_width);
                    ui.add_space(8.0);

                    for category in [
                        SettingsCategory::Editor,
                        SettingsCategory::Appearance,
                        SettingsCategory::Workbench,
                        SettingsCategory::Search,
                        SettingsCategory::About,
                    ] {
                        let is_selected = self.selected_category == category;

                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(sidebar_width - 8.0, item_height),
                            egui::Sense::click(),
                        );

                        let bg_color = if is_selected {
                            theme::CherryBlossomTheme::BG_MID()
                        } else if response.hovered() {
                            theme::CherryBlossomTheme::BG_LIGHT()
                        } else {
                            theme::CherryBlossomTheme::BG_DARK()
                        };

                        ui.painter().rect_filled(rect, corner_radius, bg_color);

                        if is_selected {
                            let indicator_rect = egui::Rect::from_min_size(
                                rect.left_top() + egui::vec2(4.0, 8.0),
                                egui::vec2(3.0, item_height - 16.0),
                            );
                            ui.painter().rect_filled(indicator_rect, 1.5, theme::CherryBlossomTheme::ACCENT_PINK());
                        }

                        let text = format!("{}", category.name());
                        let text_color = if is_selected {
                            theme::CherryBlossomTheme::TEXT_PRIMARY()
                        } else {
                            theme::CherryBlossomTheme::TEXT_SECONDARY()
                        };

                        let text_x = if is_selected { 16.0 } else { 12.0 };

                        ui.painter().text(
                            rect.left_center() + egui::vec2(text_x, 0.0),
                            egui::Align2::LEFT_CENTER,
                            text,
                            egui::FontId::new(14.0, egui::FontFamily::Proportional),
                            text_color,
                        );

                        if response.clicked() {
                            self.selected_category = category;
                        }

                        ui.add_space(4.0);
                    }
                },
            );

            ui.add_space(4.0);

            ui.separator();
            ui.add_space(4.0);

            let has_search = !self.search_query.is_empty();
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width() - 8.0, content_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.add_space(8.0);
                    egui::ScrollArea::vertical().show(ui, |ui| match self.selected_category {
                        SettingsCategory::Editor => {
                            self.show_editor_settings(ui, has_search, &self.search_query.clone())
                        }
                        SettingsCategory::Appearance => {
                            self.show_appearance_settings(ui, has_search, &self.search_query.clone())
                        }
                        SettingsCategory::Workbench => {
                            self.show_workbench_settings(ui, has_search, &self.search_query.clone())
                        }
                        SettingsCategory::Search => {
                            self.show_search_settings(ui, has_search, &self.search_query.clone())
                        }
                        SettingsCategory::About => {
                            self.show_about_settings(ui)
                        }
                    });
                },
            );

            ui.add_space(4.0);
        });
    }

    fn count_settings(&self) -> usize {
        16
    }

    fn count_matching_settings(&self, query: &str) -> usize {
        let query = query.to_lowercase();
        let mut count = 0;

        let setting_names = [
            "show line numbers",
            "word wrap",
            "show whitespace",
            "font size",
            "tab size",
            "use spaces",
            "vim mode",
            "auto save",
            "auto save interval",
            "sidebar",
            "status bar",
            "ignore directories",
            "ignored directories",
            "auto-search threshold",
            "highlight current line",
            "auto indent",
            "scroll beyond last line",
            "minimap",
        ];

        for name in setting_names {
            if name.contains(&query) {
                count += 1;
            }
        }

        count
    }

    fn show_editor_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        let query = query.to_lowercase();

        if !has_search
            || self.matches_search(
                &query,
                &["display", "line numbers", "word wrap", "whitespace"],
            )
        {
            self.setting_card(ui, "Display", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Show line numbers",
                    "Display line numbers in the editor. Note: Lines may be off by a few pixels but remain accurate",
                    |ui, settings| {
                        ui.checkbox(&mut settings.show_line_numbers, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Word wrap",
                    "Wrap lines to fit the viewport",
                    |ui, settings| {
                        ui.checkbox(&mut settings.word_wrap, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Show whitespace",
                    "Render whitespace characters",
                    |ui, settings| {
                        ui.checkbox(&mut settings.show_whitespace, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Highlight current line",
                    "Highlight the line where the cursor is",
                    |ui, settings| {
                        ui.checkbox(&mut settings.highlight_current_line, "");
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["font", "indentation", "tab", "spaces"]) {
            self.setting_card(ui, "Font & Indentation", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Font size",
                    "Editor font size in pixels",
                    |ui, settings| {
                        ui.add(
                            egui::Slider::new(&mut settings.font_size, 8.0..=32.0)
                                .show_value(true)
                                .text("px"),
                        );
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Tab size",
                    "Number of spaces per tab",
                    |ui, settings| {
                        ui.add(
                            egui::Slider::new(&mut settings.tab_size, 2..=8)
                                .show_value(true)
                                .text("spaces"),
                        );
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Use spaces",
                    "Insert spaces when pressing Tab",
                    |ui, settings| {
                        ui.checkbox(&mut settings.use_spaces, "");
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["behavior", "vim", "auto save", "interval", "auto indent", "scroll"])
        {
            self.setting_card(ui, "Behavior", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Vim mode",
                    "Enable vim-style keybindings",
                    |ui, settings| {
                        ui.checkbox(&mut settings.vim_mode, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Auto save",
                    "Automatically save files",
                    |ui, settings| {
                        ui.checkbox(&mut settings.auto_save, "");
                    },
                );
                if settings.auto_save {
                    ui.add_space(8.0);
                    settings.cozy_row_filtered(
                        ui,
                        has_search,
                        &query,
                        "Auto save interval",
                        "Seconds between auto-saves",
                        |ui, settings| {
                            ui.horizontal(|ui| {
                                ui.add(
                                    egui::Slider::new(&mut settings.auto_save_interval, 5..=300)
                                        .show_value(true)
                                        .text("sec"),
                                );
                            });
                        },
                    );
                }
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Auto indent",
                    "Automatically indent new lines",
                    |ui, settings| {
                        ui.checkbox(&mut settings.auto_indent, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Scroll beyond last line",
                    "Allow scrolling past the end of file",
                    |ui, settings| {
                        ui.checkbox(&mut settings.scroll_beyond_last_line, "");
                    },
                );
            });
        }
    }

    fn show_appearance_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        let query = query.to_lowercase();

        if !has_search || self.matches_search(&query, &["theme", "color scheme"]) {
            self.setting_card(ui, "Theme", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Theme Family",
                    "Select the theme family",
                    |ui, settings| {
                        egui::ComboBox::from_id_salt("settings_theme_family")
                            .selected_text(settings.theme_family.name())
                            .width(140.0)
                            .show_ui(ui, |ui| {
                                for family in theme::ThemeManager::all_families() {
                                    if ui
                                        .selectable_label(
                                            settings.theme_family == *family,
                                            family.name(),
                                        )
                                        .clicked()
                                    {
                                        settings.theme_family = *family;
                                        settings.theme_variant = family.default_variant();
                                    }
                                }
                            });
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Theme Variant",
                    "Select the theme variant",
                    |ui, settings| {
                        egui::ComboBox::from_id_salt("settings_theme_variant")
                            .selected_text(settings.theme_variant.name())
                            .width(140.0)
                            .show_ui(ui, |ui| {
                                for &variant in settings.theme_family.variants() {
                                    if ui
                                        .selectable_label(
                                            settings.theme_variant == variant,
                                            variant.name(),
                                        )
                                        .clicked()
                                    {
                                        settings.theme_variant = variant;
                                    }
                                }
                            });
                    },
                );
            });
            ui.add_space(12.0);
        }

        self.setting_card(ui, "UI Elements", |ui, settings| {
            settings.cozy_row_filtered(
                ui,
                has_search,
                &query,
                "Sidebar",
                "Show the left sidebar",
                |ui, settings| {
                    ui.checkbox(&mut settings.sidebar_visible, "");
                },
            );
            settings.cozy_row_filtered(
                ui,
                has_search,
                &query,
                "Status bar",
                "Show the bottom status bar",
                |ui, settings| {
                    ui.checkbox(&mut settings.status_bar_visible, "");
                },
            );
            settings.cozy_row_filtered(
                ui,
                has_search,
                &query,
                "Minimap",
                "Show code minimap on the right",
                |ui, settings| {
                    ui.checkbox(&mut settings.minimap, "");
                },
            );
            settings.cozy_row_filtered(
                ui,
                has_search,
                &query,
                "Corner roundness",
                "UI element corner radius",
                |ui, settings| {
                    ui.add(
                        egui::Slider::new(&mut settings.corner_roundness, 0.0..=20.0)
                            .show_value(true)
                            .text("px"),
                    );
                },
            );
        });
    }

    fn show_workbench_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        let query = query.to_lowercase();

        if !has_search || self.matches_search(&query, &["recent", "files", "projects", "history"]) {
            self.setting_card(ui, "Recent Items", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Recent files limit",
                    "Number of recent files to show",
                    |ui, settings| {
                        ui.add(
                            egui::Slider::new(&mut settings.recent_files_limit, 1..=20)
                                .show_value(true)
                                .text("files"),
                        );
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Recent projects limit",
                    "Number of recent projects to show",
                    |ui, settings| {
                        ui.add(
                            egui::Slider::new(&mut settings.recent_projects_limit, 1..=20)
                                .show_value(true)
                                .text("projects"),
                        );
                    },
                );
            });
        }
    }

    fn show_search_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        use theme::CherryBlossomTheme;
        let query = query.to_lowercase();

        self.setting_card(ui, "Search Behavior", |ui, settings| {
            settings.cozy_row_filtered(
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
                ui.label(
                    egui::RichText::new("Ignored patterns")
                        .size(13.0)
                        .color(theme::CherryBlossomTheme::TEXT_PRIMARY()),
                );
                ui.label(
                    egui::RichText::new("Comma-separated, use * for wildcards")
                        .size(11.0)
                        .color(theme::CherryBlossomTheme::TEXT_MUTED()),
                );
                ui.add_space(4.0);
                ui.add(
                    egui::TextEdit::multiline(&mut settings.search_ignored_dirs)
                        .desired_rows(3)
                        .desired_width(ui.available_width()),
                );
                ui.label(
                    egui::RichText::new("Examples: .git, node_modules, *venv")
                        .size(11.0)
                        .color(theme::CherryBlossomTheme::TEXT_MUTED()),
                );
                ui.add_space(16.0);
            }
            settings.cozy_row_filtered(
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

    fn matches_search(&self, query: &str, keywords: &[&str]) -> bool {
        keywords.iter().any(|kw| kw.to_lowercase().contains(query))
    }

    fn setting_card(
        &mut self,
        ui: &mut egui::Ui,
        title: &str,
        content: impl FnOnce(&mut egui::Ui, &mut Settings),
    ) {
        use theme::CherryBlossomTheme;

        let card_margin = 16.0;

        egui::Frame::group(ui.style())
            .fill(theme::CherryBlossomTheme::BG_DARK())
            .corner_radius(self.corner_roundness)
            .stroke(egui::Stroke::new(1.0, theme::CherryBlossomTheme::BG_LIGHT()))
            .inner_margin(egui::Margin::same(card_margin as i8))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.label(
                    egui::RichText::new(title)
                        .size(14.0)
                        .strong()
                        .color(theme::CherryBlossomTheme::TEXT_PRIMARY()),
                );

                ui.add_space(12.0);

                ui.painter().line_segment(
                    [
                        ui.cursor().left_center(),
                        ui.cursor().left_center() + egui::vec2(ui.available_width(), 0.0),
                    ],
                    egui::Stroke::new(1.0, theme::CherryBlossomTheme::BG_LIGHT()),
                );
                ui.add_space(12.0);

                content(ui, self);
            });
    }

    fn show_about_settings(&mut self, ui: &mut egui::Ui) {
        use theme::CherryBlossomTheme;

        egui::Frame::group(ui.style())
            .fill(CherryBlossomTheme::BG_DARK())
            .corner_radius(self.corner_roundness)
            .stroke(egui::Stroke::new(1.0, CherryBlossomTheme::BG_LIGHT()))
            .inner_margin(egui::Margin::same(24))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());

                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    
                    ui.label(
                        egui::RichText::new("AsterIDE 🌸")
                            .size(32.0)
                            .strong()
                            .color(CherryBlossomTheme::ACCENT_PINK()),
                    );
                    ui.add_space(8.0);
                    let version = env!("CARGO_PKG_VERSION");
                    ui.label(
                        egui::RichText::new(format!("AsterIDE v{}", version))
                            .size(14.0)
                            .color(CherryBlossomTheme::TEXT_SECONDARY()),
                    );
                    
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);
                    
                    ui.label(
                        egui::RichText::new("A Simple Text Editor written in Rust")
                            .size(16.0)
                            .color(CherryBlossomTheme::TEXT_PRIMARY()),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Built with 💝 and Rust.")
                            .size(13.0)
                            .color(CherryBlossomTheme::TEXT_SECONDARY()),
                    );
                    
                    ui.add_space(24.0);
                    
                    ui.hyperlink_to(
                        egui::RichText::new("Website")
                            .size(14.0)
                            .color(CherryBlossomTheme::ACCENT_PINK()),
                        "https://asteride.dev",
                    );
                    ui.add_space(8.0);
                    ui.hyperlink_to(
                        egui::RichText::new("GitHub")
                            .size(14.0)
                            .color(CherryBlossomTheme::ACCENT_PINK()),
                        "https://github.com/Aster-IDE/AsterIDE",
                    );
                    ui.add_space(8.0);
                    ui.hyperlink_to(
                        egui::RichText::new("Documentation")
                            .size(14.0)
                            .color(CherryBlossomTheme::ACCENT_PINK()),
                        "https://docs.asteride.dev",
                    );
                    
                    ui.add_space(24.0);
                    ui.separator();
                    ui.add_space(16.0);
                    
                    ui.label(
                        egui::RichText::new("Version Information")
                            .size(14.0)
                            .strong()
                            .color(CherryBlossomTheme::TEXT_PRIMARY()),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(format!("Version: v{}", env!("CARGO_PKG_VERSION")))
                            .size(12.0)
                            .color(CherryBlossomTheme::TEXT_SECONDARY()),
                    );
                    ui.label(
                        egui::RichText::new("Build: Release")
                            .size(12.0)
                            .color(CherryBlossomTheme::TEXT_SECONDARY()),
                    );
                    ui.label(
                        egui::RichText::new("Rust Edition: 2024")
                            .size(12.0)
                            .color(CherryBlossomTheme::TEXT_SECONDARY()),
                    );
                    
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);
                    
                    ui.label(
                        egui::RichText::new("Acknowledgments")
                            .size(14.0)
                            .strong()
                            .color(CherryBlossomTheme::TEXT_PRIMARY()),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Built with egui, eframe, and the Rust ecosystem")
                            .size(12.0)
                            .color(CherryBlossomTheme::TEXT_SECONDARY()),
                    );
                    
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);
                    
                    ui.hyperlink_to(
                        egui::RichText::new("© 2026 AsterIDE. This software follows the principles of the Free Software Foundation.")
                            .size(12.0)
                            .color(CherryBlossomTheme::TEXT_MUTED()),
                        "https://www.fsf.org",
                    );
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new("Licensed under GLPv3 LICENSE")
                            .size(12.0)
                            .color(CherryBlossomTheme::TEXT_MUTED()),
                    );
                    
                    ui.add_space(20.0);
                });
            });
    }

    fn cozy_row(
        &mut self,
        ui: &mut egui::Ui,
        title: &str,
        description: &str,
        control: impl FnOnce(&mut egui::Ui, &mut Settings),
    ) {
        use theme::CherryBlossomTheme;

        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());

            ui.vertical(|ui| {
                ui.label(
                    egui::RichText::new(title)
                        .size(13.0)
                        .color(theme::CherryBlossomTheme::TEXT_PRIMARY()),
                );
                ui.label(
                    egui::RichText::new(description)
                        .size(11.0)
                        .color(theme::CherryBlossomTheme::TEXT_MUTED()),
                );
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                control(ui, self);
            });
        });

        ui.add_space(12.0);
    }

    fn cozy_row_filtered(
        &mut self,
        ui: &mut egui::Ui,
        has_search: bool,
        query: &str,
        title: &str,
        description: &str,
        control: impl FnOnce(&mut egui::Ui, &mut Settings),
    ) {
        if has_search {
            let search_text = format!(
                "{} {} {}",
                title,
                description,
                self.get_setting_keywords(title)
            )
            .to_lowercase();
            if !search_text.contains(query) {
                return;
            }
        }
        self.cozy_row(ui, title, description, control);
    }

    fn get_setting_keywords(&self, title: &str) -> &'static str {
        match title {
            "Show line numbers" => "gutter numbers",
            "Word wrap" => "wrap line break",
            "Show whitespace" => "space tab visible",
            "Font size" => "text zoom",
            "Tab size" => "indent width",
            "Use spaces" => "soft tab indent",
            "Vim mode" => "modal editing",
            "Auto save" => "autosave backup",
            "Auto save interval" => "frequency delay",
            "Sidebar" => "explorer panel",
            "Status bar" => "bottom panel info",
            "Ignore directories" => "exclude skip folders",
            "Auto-search threshold" => "minimum characters",
            "Highlight current line" => "cursor row",
            "Auto indent" => "automatic indentation",
            "Scroll beyond last line" => "overscroll end of file",
            "Minimap" => "code overview zoomout",
            "Corner roundness" => "border radius curve",
            _ => "",
        }
    }
}
