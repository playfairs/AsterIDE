use core::editor::Editor;
use std::path::PathBuf;

#[derive(PartialEq, Copy, Clone)]
pub enum TabType {
    File,
    Settings,
    SearchResults,
}

pub struct Tab {
    pub id: usize,
    pub name: String,
    pub path: Option<PathBuf>,
    pub editor: Editor,
    pub is_modified: bool,
    pub tab_type: TabType,
}

pub struct TabManager {
    pub tabs: Vec<Tab>,
    pub active_tab: usize,
    next_id: usize,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: Vec::new(),
            active_tab: 0,
            next_id: 1,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.tabs.is_empty()
    }

    pub fn new_tab(&mut self) {
        let id = self.next_id;
        self.next_id += 1;

        let tab = Tab {
            id,
            name: format!("untitled-{}", id),
            path: None,
            editor: Editor::new(),
            is_modified: false,
            tab_type: TabType::File,
        };

        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    pub fn open_settings_tab(&mut self) {
        for (i, tab) in self.tabs.iter().enumerate() {
            if tab.tab_type == TabType::Settings {
                self.active_tab = i;
                return;
            }
        }

        let id = self.next_id;
        self.next_id += 1;

        let tab = Tab {
            id,
            name: "Settings".to_string(),
            path: None,
            editor: Editor::new(),
            is_modified: false,
            tab_type: TabType::Settings,
        };

        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    pub fn open_search_tab(&mut self) {
        for (i, tab) in self.tabs.iter().enumerate() {
            if tab.tab_type == TabType::SearchResults {
                self.active_tab = i;
                return;
            }
        }

        let id = self.next_id;
        self.next_id += 1;

        let tab = Tab {
            id,
            name: "Search Results".to_string(),
            path: None,
            editor: Editor::new(),
            is_modified: false,
            tab_type: TabType::SearchResults,
        };

        self.tabs.push(tab);
        self.active_tab = self.tabs.len() - 1;
    }

    pub fn open_file(&mut self, path: PathBuf, content: String) {
        self.open_file_internal(path, content, true);
    }

    pub fn open_file_in_background(&mut self, path: PathBuf, content: String) {
        self.open_file_internal(path, content, false);
    }

    fn open_file_internal(&mut self, path: PathBuf, content: String, switch_to_tab: bool) {
        for (i, tab) in self.tabs.iter().enumerate() {
            if tab.tab_type == TabType::File && tab.path.as_ref() == Some(&path) {
                if switch_to_tab {
                    self.active_tab = i;
                }
                return;
            }
        }

        let id = self.next_id;
        self.next_id += 1;

        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "untitled".to_string());

        let mut editor = Editor::new();
        editor.buffer = core::buffer::Buffer::from_str(&content);

        let tab = Tab {
            id,
            name,
            path: Some(path),
            editor,
            is_modified: false,
            tab_type: TabType::File,
        };

        self.tabs.push(tab);
        if switch_to_tab {
            self.active_tab = self.tabs.len() - 1;
        }
    }

    pub fn close_tab(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.tabs.remove(index);
            if !self.tabs.is_empty() && self.active_tab >= self.tabs.len() {
                self.active_tab = self.tabs.len() - 1;
            }
        }
    }

    pub fn close_active_tab(&mut self) {
        self.close_tab(self.active_tab);
    }

    pub fn set_active(&mut self, index: usize) {
        if index < self.tabs.len() {
            self.active_tab = index;
        }
    }

    pub fn active_tab(&self) -> Option<&Tab> {
        if self.tabs.is_empty() {
            None
        } else {
            self.tabs.get(self.active_tab)
        }
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut Tab> {
        if self.tabs.is_empty() {
            None
        } else {
            self.tabs.get_mut(self.active_tab)
        }
    }

    pub fn current_editor(&self) -> Option<&Editor> {
        self.active_tab().map(|t| &t.editor)
    }

    pub fn current_editor_mut(&mut self) -> Option<&mut Editor> {
        self.active_tab_mut().map(|t| &mut t.editor)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Tab> {
        self.tabs.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Tab> {
        self.tabs.iter_mut()
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
}
