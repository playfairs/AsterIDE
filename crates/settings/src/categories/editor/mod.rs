use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum CursorStyle {
    Line,
    Block,
    Underline,
}

impl Default for CursorStyle {
    fn default() -> Self {
        CursorStyle::Line
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EditorSettings {
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
}

impl Default for EditorSettings {
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
        }
    }
}

pub trait EditorSettingsUi {
    fn show_editor_settings(&mut self, ui: &mut egui::Ui, has_search: bool, query: &str);
}
