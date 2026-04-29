use serde::{Deserialize, Serialize};
use std::fs;

pub mod categories;
pub mod ui;

pub use categories::editor::{CursorStyle, EditorSettings};
pub use categories::appearance::AppearanceSettings;
pub use categories::workbench::{PanelPosition, WorkbenchSettings};
pub use categories::search::SearchSettings;
pub use categories::keyboard::{KeymapScheme, KeyboardSettings};
pub use categories::files::FilesSettings;
pub use categories::lsp::LspSettings;
pub use categories::scm::ScmSettings;

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum SettingsCategory {
    About,
    Appearance,
    Editor,
    Files,
    Keyboard,
    Search,
    Workbench,
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
    pub font_family: String,
    pub tab_size: usize,
    pub use_spaces: bool,
    pub show_whitespace: bool,
    pub show_indent_guides: bool,
    pub vim_mode: bool,
    pub auto_save: bool,
    pub auto_save_interval: u64,
    pub auto_save_on_focus_lost: bool,
    pub highlight_current_line: bool,
    pub highlight_matching_brackets: bool,
    pub auto_indent: bool,
    pub auto_close_brackets: bool,
    pub auto_close_quotes: bool,
    pub scroll_beyond_last_line: bool,
    pub minimap: bool,
    pub line_height: f32,
    pub cursor_blinking: bool,
    pub cursor_style: CursorStyle,
    
    pub sidebar_visible: bool,
    pub status_bar_visible: bool,
    pub activity_bar_visible: bool,
    pub corner_roundness: f32,
    pub theme_family: theme::ThemeFamily,
    pub theme_variant: theme::ThemeVariant,
    pub custom_background_color: Option<String>,
    pub custom_accent_color: Option<String>,
    pub window_opacity: f32,
    pub animations_enabled: bool,
    pub font_smoothing: bool,
    
    pub panel_size: f32,
    pub panel_position: PanelPosition,
    pub auto_hide_panel: bool,
    pub show_open_editors: bool,
    pub show_explorer: bool,
    pub show_search: bool,
    pub show_git: bool,
    pub show_extensions: bool,
    pub compact_mode: bool,
    
    pub search_ignore_dirs_enabled: bool,
    pub search_ignored_dirs: String,
    pub search_min_chars: usize,
    pub search_case_sensitive: bool,
    pub search_whole_word: bool,
    pub search_use_regex: bool,
    pub search_include_hidden: bool,
    pub search_follow_symlinks: bool,
    
    pub keymap_scheme: KeymapScheme,
    pub vim_leader_key: String,
    pub multi_cursor_enabled: bool,
    pub bracket_pair_colorization: bool,
    pub suggest_snippets: bool,
    pub quick_suggestions: bool,
    
    pub recent_files_limit: usize,
    pub recent_projects_limit: usize,
    pub auto_detect_indentation: bool,
    pub trim_trailing_whitespace: bool,
    pub insert_final_newline: bool,
    pub trim_auto_whitespace: bool,
    pub pinned_files: Vec<std::path::PathBuf>,
    pub file_associations: std::collections::HashMap<String, String>,
    pub default_file_encoding: String,
    pub auto_reload_files: bool,
    pub confirm_before_save: bool,
    
    #[serde(skip)]
    pub new_file_ext_input: String,
    #[serde(skip)]
    pub new_file_lang_input: String,
    
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
            font_family: "SF Mono".to_string(),
            tab_size: 4,
            use_spaces: true,
            show_whitespace: false,
            show_indent_guides: true,
            vim_mode: false,
            auto_save: false,
            auto_save_interval: 30,
            auto_save_on_focus_lost: true,
            highlight_current_line: true,
            highlight_matching_brackets: true,
            auto_indent: true,
            auto_close_brackets: true,
            auto_close_quotes: true,
            scroll_beyond_last_line: true,
            minimap: false,
            line_height: 1.4,
            cursor_blinking: true,
            cursor_style: CursorStyle::default(),
            
            sidebar_visible: true,
            status_bar_visible: true,
            activity_bar_visible: true,
            corner_roundness: 6.0,
            theme_family: theme::ThemeFamily::CherryBlossom,
            theme_variant: theme::ThemeVariant::CherryBlossomDark,
            custom_background_color: None,
            custom_accent_color: None,
            window_opacity: 1.0,
            animations_enabled: true,
            font_smoothing: true,
            
            panel_size: 200.0,
            panel_position: PanelPosition::default(),
            auto_hide_panel: false,
            show_open_editors: true,
            show_explorer: true,
            show_search: true,
            show_git: true,
            show_extensions: true,
            compact_mode: false,
            
            search_ignore_dirs_enabled: true,
            search_ignored_dirs: ".git, node_modules, venv, .venv, target, dist, build, .next, .cache, __pycache__, .idea, .vscode".to_string(),
            search_min_chars: 2,
            search_case_sensitive: false,
            search_whole_word: false,
            search_use_regex: false,
            search_include_hidden: false,
            search_follow_symlinks: false,
            
            keymap_scheme: KeymapScheme::default(),
            vim_leader_key: ",".to_string(),
            multi_cursor_enabled: true,
            bracket_pair_colorization: true,
            suggest_snippets: true,
            quick_suggestions: true,
            
            recent_files_limit: 5,
            recent_projects_limit: 5,
            auto_detect_indentation: true,
            trim_trailing_whitespace: false,
            insert_final_newline: true,
            trim_auto_whitespace: true,
            pinned_files: Vec::new(),
            file_associations: std::collections::HashMap::new(),
            default_file_encoding: "utf8".to_string(),
            auto_reload_files: true,
            confirm_before_save: false,
            
            new_file_ext_input: String::new(),
            new_file_lang_input: String::new(),
            
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
            SettingsCategory::About => "About",
            SettingsCategory::Appearance => "Appearance",
            SettingsCategory::Editor => "Editor",
            SettingsCategory::Files => "Files",
            SettingsCategory::Keyboard => "Keyboard",
            SettingsCategory::Search => "Search",
            SettingsCategory::Workbench => "Workbench",
        }
    }

    #[allow(dead_code)]
    fn icon(&self) -> &'static str {
        match self {
            SettingsCategory::About => "ℹ️",
            SettingsCategory::Appearance => "🎨",
            SettingsCategory::Editor => "�",
            SettingsCategory::Files => "�",
            SettingsCategory::Keyboard => "⌨️",
            SettingsCategory::Search => "�",
            SettingsCategory::Workbench => "🖥️",
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
                || self.file_associations != saved.file_associations
                || self.default_file_encoding != saved.default_file_encoding
                || self.auto_reload_files != saved.auto_reload_files
                || self.confirm_before_save != saved.confirm_before_save
                || self.auto_detect_indentation != saved.auto_detect_indentation
                || self.trim_trailing_whitespace != saved.trim_trailing_whitespace
                || self.insert_final_newline != saved.insert_final_newline
                || self.trim_auto_whitespace != saved.trim_auto_whitespace
                || self.keymap_scheme != saved.keymap_scheme
                || self.vim_leader_key != saved.vim_leader_key
                || self.multi_cursor_enabled != saved.multi_cursor_enabled
                || self.bracket_pair_colorization != saved.bracket_pair_colorization
                || self.suggest_snippets != saved.suggest_snippets
                || self.quick_suggestions != saved.quick_suggestions
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
            self.file_associations = saved.file_associations.clone();
            self.default_file_encoding = saved.default_file_encoding.clone();
            self.auto_reload_files = saved.auto_reload_files;
            self.confirm_before_save = saved.confirm_before_save;
            self.auto_detect_indentation = saved.auto_detect_indentation;
            self.trim_trailing_whitespace = saved.trim_trailing_whitespace;
            self.insert_final_newline = saved.insert_final_newline;
            self.trim_auto_whitespace = saved.trim_auto_whitespace;
            self.keymap_scheme = saved.keymap_scheme;
            self.vim_leader_key = saved.vim_leader_key.clone();
            self.multi_cursor_enabled = saved.multi_cursor_enabled;
            self.bracket_pair_colorization = saved.bracket_pair_colorization;
            self.suggest_snippets = saved.suggest_snippets;
            self.quick_suggestions = saved.quick_suggestions;
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
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Unsaved Changes")
                                .size(18.0)
                                .strong()
                                .color(theme::CherryBlossomTheme::TEXT_PRIMARY())
                        ).selectable(false)
                    );
                    ui.add_space(12.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("You have unsaved settings changes.")
                                .size(14.0)
                                .color(theme::CherryBlossomTheme::TEXT_SECONDARY())
                        ).selectable(false)
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Discard them?")
                                .size(14.0)
                                .color(theme::CherryBlossomTheme::TEXT_SECONDARY())
                        ).selectable(false)
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

            ui.add(
                egui::Label::new(
                    egui::RichText::new(self.selected_category.name())
                        .size(18.0)
                        .strong()
                        .color(theme::CherryBlossomTheme::TEXT_PRIMARY())
                ).selectable(false)
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
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(format!("{} / {} Settings", matches, total_count))
                            .size(12.0)
                            .color(theme::CherryBlossomTheme::TEXT_MUTED())
                    ).selectable(false)
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
            let corner_radius = self.corner_roundness;

            ui.allocate_ui_with_layout(
                egui::vec2(sidebar_width, content_height),
                egui::Layout::top_down(egui::Align::Min),
                |ui| {
                    ui.set_width(sidebar_width);
                    ui.add_space(8.0);

                    let categories: Vec<SettingsCategory> = vec![
                        SettingsCategory::About,
                        SettingsCategory::Appearance,
                        SettingsCategory::Editor,
                        SettingsCategory::Files,
                        SettingsCategory::Keyboard,
                        SettingsCategory::Search,
                        SettingsCategory::Workbench,
                    ];

                    for category in categories {
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
                            let indicator_margin = corner_radius.clamp(4.0, 10.0);
                            let indicator_height = item_height - (indicator_margin * 2.0);
                            let indicator_width = (corner_radius * 0.4).clamp(3.0, 6.0);
                            let indicator_rect = egui::Rect::from_min_size(
                                rect.left_top() + egui::vec2(indicator_margin * 0.5, indicator_margin),
                                egui::vec2(indicator_width, indicator_height),
                            );
                            ui.painter().rect_filled(indicator_rect, corner_radius * 0.6, theme::CherryBlossomTheme::ACCENT_PINK());
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
                    egui::ScrollArea::both()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| match self.selected_category {
                        SettingsCategory::About => {
                            self.show_about_settings(ui)
                        }
                        SettingsCategory::Appearance => {
                            self.show_appearance_settings(ui, has_search, &self.search_query.clone())
                        }
                        SettingsCategory::Editor => {
                            self.show_editor_settings(ui, has_search, &self.search_query.clone())
                        }
                        SettingsCategory::Files => {
                            self.show_file_settings(ui, has_search, &self.search_query.clone())
                        }
                        SettingsCategory::Keyboard => {
                            self.show_keyboard_settings(ui, has_search, &self.search_query.clone())
                        }
                        SettingsCategory::Search => {
                            self.show_search_settings(ui, has_search, &self.search_query.clone())
                        }
                        SettingsCategory::Workbench => {
                            self.show_workbench_settings(ui, has_search, &self.search_query.clone())
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

    
    fn show_appearance_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        crate::categories::appearance::show_appearance_settings(self, ui, has_search, query);
    }

    fn show_workbench_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        crate::categories::workbench::show_workbench_settings(self, ui, has_search, query);
    }

    fn show_search_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        crate::categories::search::show_search_settings(self, ui, has_search, query);
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

                ui.add(
                    egui::Label::new(
                        egui::RichText::new(title)
                            .size(14.0)
                            .strong()
                            .color(theme::CherryBlossomTheme::TEXT_PRIMARY())
                    ).selectable(false)
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
                    
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("AsterIDE 🌸")
                                .size(32.0)
                                .strong()
                                .color(CherryBlossomTheme::ACCENT_PINK())
                        ).selectable(false)
                    );
                    ui.add_space(8.0);
                    let version = env!("CARGO_PKG_VERSION");
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!("AsterIDE v{}", version))
                                .size(14.0)
                                .color(CherryBlossomTheme::TEXT_SECONDARY())
                        ).selectable(false)
                    );
                    
                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);
                    
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("A Simple Text Editor written in Rust")
                                .size(16.0)
                                .color(CherryBlossomTheme::TEXT_PRIMARY())
                        ).selectable(false)
                    );
                    ui.add_space(8.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Built with 💝 and Rust.")
                                .size(13.0)
                                .color(CherryBlossomTheme::TEXT_SECONDARY())
                        ).selectable(false)
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
                    
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Version Information")
                                .size(14.0)
                                .strong()
                                .color(CherryBlossomTheme::TEXT_PRIMARY())
                        ).selectable(false)
                    );
                    ui.add_space(8.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!("Version: v{}", env!("CARGO_PKG_VERSION")))
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_SECONDARY())
                        ).selectable(false)
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Build: Release")
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_SECONDARY())
                        ).selectable(false)
                    );
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Rust Edition: 2024")
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_SECONDARY())
                        ).selectable(false)
                    );
                    
                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);
                    
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Acknowledgments")
                                .size(14.0)
                                .strong()
                                .color(CherryBlossomTheme::TEXT_PRIMARY())
                        ).selectable(false)
                    );
                    ui.add_space(8.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Built with egui, eframe, and the Rust ecosystem")
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_SECONDARY())
                        ).selectable(false)
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
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("Licensed under GLPv3 LICENSE")
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_MUTED())
                        ).selectable(false)
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
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(title)
                            .size(13.0)
                            .color(theme::CherryBlossomTheme::TEXT_PRIMARY())
                    ).selectable(false)
                );
                ui.add(
                    egui::Label::new(
                        egui::RichText::new(description)
                            .size(11.0)
                            .color(theme::CherryBlossomTheme::TEXT_MUTED())
                    ).selectable(false)
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

    fn show_keyboard_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        let query = query.to_lowercase();

        if !has_search || self.matches_search(&query, &["keymap", "scheme", "shortcuts"]) {
            self.setting_card(ui, "Keymap", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Keymap Scheme",
                    "Choose keyboard shortcut layout",
                    |ui, settings| {
                        egui::ComboBox::from_id_salt("settings_keymap_scheme")
                            .selected_text(format!("{:?}", settings.keymap_scheme))
                            .width(140.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut settings.keymap_scheme, KeymapScheme::Default, "Default");
                                ui.selectable_value(&mut settings.keymap_scheme, KeymapScheme::VSCode, "VSCode");
                                ui.selectable_value(&mut settings.keymap_scheme, KeymapScheme::SublimeText, "Sublime Text");
                                ui.selectable_value(&mut settings.keymap_scheme, KeymapScheme::Atom, "Atom");
                                ui.selectable_value(&mut settings.keymap_scheme, KeymapScheme::Emacs, "Emacs");
                            });
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["vim", "modal", "leader"]) {
            self.setting_card(ui, "Vim", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Vim Mode",
                    "Enable vim-style keybindings",
                    |ui, settings| {
                        ui.checkbox(&mut settings.vim_mode, "");
                    },
                );
                if settings.vim_mode {
                    settings.cozy_row_filtered(
                        ui,
                        has_search,
                        &query,
                        "Leader Key",
                        "Vim leader key for custom commands",
                        |ui, settings| {
                        ui.add(egui::TextEdit::singleline(&mut settings.vim_leader_key)
                            .desired_width(40.0)
                            .id_salt("vim_leader_key"));
                        },
                    );
                }
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["editor", "features", "multi", "cursor", "bracket", "snippet"]) {
            self.setting_card(ui, "Editor Features", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Multi-cursor",
                    "Enable multiple cursors",
                    |ui, settings| {
                        ui.checkbox(&mut settings.multi_cursor_enabled, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Bracket Pair Colorization",
                    "Colorize matching bracket pairs",
                    |ui, settings| {
                        ui.checkbox(&mut settings.bracket_pair_colorization, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Suggest Snippets",
                    "Show code snippet suggestions",
                    |ui, settings| {
                        ui.checkbox(&mut settings.suggest_snippets, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Quick Suggestions",
                    "Show quick suggestions while typing",
                    |ui, settings| {
                        ui.checkbox(&mut settings.quick_suggestions, "");
                    },
                );
            });
        }
    }

    fn show_editor_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        let query = query.to_lowercase();

        if !has_search || self.matches_search(&query, &["font", "text", "editor"]) {
            self.setting_card(ui, "Font", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Font Family",
                    "Editor font family",
                    |ui, settings| {
                        ui.add(egui::TextEdit::singleline(&mut settings.font_family)
                            .hint_text("SF Mono")
                            .id_salt("editor_font_family"));
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Font Size",
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
                    "Line Height",
                    "Line height multiplier",
                    |ui, settings| {
                        ui.add(
                            egui::Slider::new(&mut settings.line_height, 1.0..=3.0)
                                .show_value(true),
                        );
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["cursor", "caret", "blink"]) {
            self.setting_card(ui, "Cursor", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Cursor Blinking",
                    "Enable cursor animation",
                    |ui, settings| {
                        ui.checkbox(&mut settings.cursor_blinking, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Cursor Style",
                    "Cursor appearance",
                    |ui, settings| {
                        egui::ComboBox::from_id_salt("settings_cursor_style")
                            .selected_text(format!("{:?}", settings.cursor_style))
                            .width(120.0)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut settings.cursor_style, CursorStyle::Line, "Line");
                                ui.selectable_value(&mut settings.cursor_style, CursorStyle::Block, "Block");
                                ui.selectable_value(&mut settings.cursor_style, CursorStyle::Underline, "Underline");
                            });
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["tab", "indent", "space"]) {
            self.setting_card(ui, "Indentation", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Tab Size",
                    "Number of spaces per tab",
                    |ui, settings| {
                        ui.add(
                            egui::Slider::new(&mut settings.tab_size, 1..=16)
                                .show_value(true)
                                .text("spaces"),
                        );
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Use Spaces",
                    "Insert spaces when pressing Tab",
                    |ui, settings| {
                        ui.checkbox(&mut settings.use_spaces, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Auto Indent",
                    "Automatically indent new lines",
                    |ui, settings| {
                        ui.checkbox(&mut settings.auto_indent, "");
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["display", "line", "whitespace", "wrap"]) {
            self.setting_card(ui, "Display", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Show Line Numbers",
                    "Display line numbers in the editor",
                    |ui, settings| {
                        ui.checkbox(&mut settings.show_line_numbers, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Show Whitespace",
                    "Render whitespace characters",
                    |ui, settings| {
                        ui.checkbox(&mut settings.show_whitespace, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Show Indent Guides",
                    "Display vertical lines for indentation",
                    |ui, settings| {
                        ui.checkbox(&mut settings.show_indent_guides, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Word Wrap",
                    "Wrap lines to fit the viewport",
                    |ui, settings| {
                        ui.checkbox(&mut settings.word_wrap, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Highlight Current Line",
                    "Highlight the line where the cursor is",
                    |ui, settings| {
                        ui.checkbox(&mut settings.highlight_current_line, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Highlight Matching Brackets",
                    "Highlight matching bracket pairs",
                    |ui, settings| {
                        ui.checkbox(&mut settings.highlight_matching_brackets, "");
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["behavior", "vim", "auto", "save", "scroll"]) {
            self.setting_card(ui, "Behavior", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Vim Mode",
                    "Enable vim-style keybindings",
                    |ui, settings| {
                        ui.checkbox(&mut settings.vim_mode, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Auto-save",
                    "Automatically save files",
                    |ui, settings| {
                        ui.checkbox(&mut settings.auto_save, "");
                    },
                );
                if settings.auto_save {
                    settings.cozy_row_filtered(
                        ui,
                        has_search,
                        &query,
                        "Auto-save Interval",
                        "Seconds between auto-saves",
                        |ui, settings| {
                            ui.add(
                                egui::Slider::new(&mut settings.auto_save_interval, 10..=300)
                                    .show_value(true)
                                    .text("sec"),
                            );
                        },
                    );
                }
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Auto-save on Focus Lost",
                    "Save when window loses focus",
                    |ui, settings| {
                        ui.checkbox(&mut settings.auto_save_on_focus_lost, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Auto-close Brackets",
                    "Automatically close bracket pairs",
                    |ui, settings| {
                        ui.checkbox(&mut settings.auto_close_brackets, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Auto-close Quotes",
                    "Automatically close quote pairs",
                    |ui, settings| {
                        ui.checkbox(&mut settings.auto_close_quotes, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Scroll Beyond Last Line",
                    "Allow scrolling past the end of file",
                    |ui, settings| {
                        ui.checkbox(&mut settings.scroll_beyond_last_line, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Show Minimap",
                    "Display code minimap",
                    |ui, settings| {
                        ui.checkbox(&mut settings.minimap, "");
                    },
                );
            });
        }
    }

    fn show_file_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str) {
        let query = query.to_lowercase();

        if !has_search || self.matches_search(&query, &["recent", "history", "files", "projects"]) {
            self.setting_card(ui, "Recent Items", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Recent Files Limit",
                    "Number of recent files to show",
                    |ui, settings| {
                        ui.add(
                            egui::Slider::new(&mut settings.recent_files_limit, 0..=20)
                                .show_value(true)
                                .text("files"),
                        );
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Recent Projects Limit",
                    "Number of recent projects to show",
                    |ui, settings| {
                        ui.add(
                            egui::Slider::new(&mut settings.recent_projects_limit, 0..=10)
                                .show_value(true)
                                .text("projects"),
                        );
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["file", "handling", "auto", "save", "whitespace"]) {
            self.setting_card(ui, "File Handling", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Auto-detect Indentation",
                    "Automatically detect file indentation style",
                    |ui, settings| {
                        ui.checkbox(&mut settings.auto_detect_indentation, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Trim Trailing Whitespace",
                    "Remove whitespace at end of lines",
                    |ui, settings| {
                        ui.checkbox(&mut settings.trim_trailing_whitespace, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Insert Final Newline",
                    "Add newline at end of file",
                    |ui, settings| {
                        ui.checkbox(&mut settings.insert_final_newline, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Trim Auto Whitespace",
                    "Remove automatically added whitespace",
                    |ui, settings| {
                        ui.checkbox(&mut settings.trim_auto_whitespace, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Auto-reload Files",
                    "Reload files when changed externally",
                    |ui, settings| {
                        ui.checkbox(&mut settings.auto_reload_files, "");
                    },
                );
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Confirm Before Save",
                    "Show confirmation dialog before saving",
                    |ui, settings| {
                        ui.checkbox(&mut settings.confirm_before_save, "");
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["encoding", "charset", "utf"]) {
            self.setting_card(ui, "Encoding", |ui, settings| {
                settings.cozy_row_filtered(
                    ui,
                    has_search,
                    &query,
                    "Default File Encoding",
                    "Default encoding for new files",
                    |ui, settings| {
                        ui.add(egui::TextEdit::singleline(&mut settings.default_file_encoding)
                            .hint_text("utf8")
                            .id_salt("default_file_encoding"));
                    },
                );
            });
            ui.add_space(12.0);
        }

        if !has_search || self.matches_search(&query, &["association", "extension", "language", "mapping"]) {
            self.setting_card(ui, "File Associations", |ui, settings| {
                ui.add(egui::Label::new(egui::RichText::new("Add Association").size(13.0).strong()).selectable(false));
                ui.add(egui::Label::new(egui::RichText::new("Map file extension to language mode").size(11.0).color(theme::CherryBlossomTheme::TEXT_MUTED())).selectable(false));
                ui.add_space(8.0);
                
                egui::Grid::new("file_assoc_grid").num_columns(2).spacing([8.0, 6.0]).show(ui, |ui| {
                    ui.add(egui::Label::new("Extension:").selectable(false));
                    ui.add(egui::TextEdit::singleline(&mut settings.new_file_ext_input)
                        .hint_text(".rs")
                        .id_salt("new_file_ext")
                        .desired_width(120.0));
                    ui.end_row();
                    
                    ui.add(egui::Label::new("Language:").selectable(false));
                    ui.horizontal(|ui| {
                        ui.add(egui::TextEdit::singleline(&mut settings.new_file_lang_input)
                            .hint_text("rust")
                            .id_salt("new_file_lang")
                            .desired_width(120.0));
                        ui.add_space(8.0);
                        if ui.button("Add").clicked() && !settings.new_file_ext_input.is_empty() && !settings.new_file_lang_input.is_empty() {
                            settings.file_associations.insert(settings.new_file_ext_input.clone(), settings.new_file_lang_input.clone());
                            settings.new_file_ext_input.clear();
                            settings.new_file_lang_input.clear();
                        }
                    });
                    ui.end_row();
                });
                
                ui.add_space(16.0);
                
                if !settings.file_associations.is_empty() {
                    ui.add(egui::Label::new(egui::RichText::new("Existing Associations").size(13.0).strong()).selectable(false));
                    ui.add_space(8.0);
                    
                    egui::Grid::new("existing_assocs_grid").num_columns(2).spacing([16.0, 4.0]).show(ui, |ui| {
                        let associations: Vec<_> = settings.file_associations.clone().into_iter().collect();
                        for (ext, lang) in associations.into_iter() {
                            ui.add(egui::Label::new(format!("{} : {}", ext, lang)).selectable(false));
                            if ui.button("×").clicked() {
                                settings.file_associations.remove(&ext);
                            }
                            ui.end_row();
                        }
                    });
                }
            });
        }
    }
}
