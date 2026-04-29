use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum KeymapScheme {
    Default,
    VSCode,
    SublimeText,
    Atom,
    Emacs,
}

impl Default for KeymapScheme {
    fn default() -> Self {
        KeymapScheme::VSCode
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyboardSettings {
    pub keymap_scheme: KeymapScheme,
    pub vim_leader_key: String,
    pub multi_cursor_enabled: bool,
    pub bracket_pair_colorization: bool,
    pub suggest_snippets: bool,
    pub quick_suggestions: bool,
}

impl Default for KeyboardSettings {
    fn default() -> Self {
        Self {
            keymap_scheme: KeymapScheme::default(),
            vim_leader_key: ",".to_string(),
            multi_cursor_enabled: true,
            bracket_pair_colorization: true,
            suggest_snippets: true,
            quick_suggestions: true,
        }
    }
}
