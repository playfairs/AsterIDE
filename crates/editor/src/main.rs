mod command_palette;
mod search;
mod settings;
mod tabs;
mod theme;

use command_palette::CommandPalette;
use eframe::egui;
use serde::{Deserialize, Serialize};
use settings::Settings;
use tabs::{TabManager, TabType};
use theme::CherryBlossomTheme;

#[derive(Serialize, Deserialize, Default)]
struct AppState {
    recent_projects: Vec<std::path::PathBuf>,
    recent_files: Vec<RecentFile>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct RecentFile {
    path: std::path::PathBuf,
    project_dir: Option<std::path::PathBuf>,
}

const MAX_RECENT_PROJECTS: usize = 10;

const MAX_RECENT_FILES: usize = 10;

struct AsterIDE {
    tabs: TabManager,
    settings: Settings,
    sidebar_width: f32,
    active_sidebar_tab: SidebarTab,
    command_palette: CommandPalette,
    status_message: String,
    status_message_time: f64,
    opened_folder: Option<std::path::PathBuf>,
    expanded_folders: std::collections::HashSet<std::path::PathBuf>,
    selected_folder: Option<std::path::PathBuf>,
    editor_had_focus: bool,
    editor_id: Option<egui::Id>,
    recent_projects: Vec<std::path::PathBuf>,
    recent_files: Vec<RecentFile>,
}

#[derive(PartialEq)]
enum SidebarTab {
    Explorer,
    Search,
    Git,
    Extensions,
}

impl Default for AsterIDE {
    fn default() -> Self {
        let recent_projects = Self::load_recent_projects();
        let recent_files = Self::load_recent_files();
        Self {
            tabs: TabManager::new(),
            settings: Settings::load(),
            sidebar_width: 250.0,
            active_sidebar_tab: SidebarTab::Explorer,
            command_palette: CommandPalette::default(),
            status_message: "Ready".to_string(),
            status_message_time: 0.0,
            opened_folder: None,
            expanded_folders: std::collections::HashSet::new(),
            selected_folder: None,
            editor_had_focus: false,
            editor_id: None,
            recent_projects,
            recent_files,
        }
    }
}

impl AsterIDE {
    fn config_dir() -> Option<std::path::PathBuf> {
        dirs::config_dir().map(|d| d.join("asteride"))
    }

    fn state_file_path() -> Option<std::path::PathBuf> {
        Self::config_dir().map(|d| d.join("state.json"))
    }

    fn load_recent_projects() -> Vec<std::path::PathBuf> {
        if let Some(path) = Self::state_file_path() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(state) = serde_json::from_str::<AppState>(&content) {
                    return state
                        .recent_projects
                        .into_iter()
                        .filter(|p| p.exists())
                        .take(MAX_RECENT_PROJECTS)
                        .collect();
                }
            }
        }
        Vec::new()
    }

    fn load_recent_files() -> Vec<RecentFile> {
        if let Some(path) = Self::state_file_path() {
            if let Ok(content) = std::fs::read_to_string(&path) {
                if let Ok(state) = serde_json::from_str::<AppState>(&content) {
                    return state
                        .recent_files
                        .into_iter()
                        .filter(|f| f.path.exists())
                        .take(MAX_RECENT_FILES)
                        .collect();
                }
            }
        }
        Vec::new()
    }

    fn save_state(&self) {
        if let Some(path) = Self::state_file_path() {
            if let Some(dir) = path.parent() {
                let _ = std::fs::create_dir_all(dir);
            }
            let state = AppState {
                recent_projects: self.recent_projects.clone(),
                recent_files: self.recent_files.clone(),
            };
            if let Ok(json) = serde_json::to_string_pretty(&state) {
                let _ = std::fs::write(&path, json);
            }
        }
    }

    fn add_recent_file(&mut self, path: std::path::PathBuf) {
        let project_dir = self.opened_folder.clone();
        let recent = RecentFile {
            path,
            project_dir,
        };
        self.recent_files.retain(|f| f.path != recent.path);
        self.recent_files.insert(0, recent);
        if self.recent_files.len() > MAX_RECENT_FILES {
            self.recent_files.truncate(MAX_RECENT_FILES);
        }
        self.save_state();
    }

    fn get_relevant_recent_files(&self) -> Vec<RecentFile> {
        match &self.opened_folder {
            Some(project) => {
                self.recent_files
                    .iter()
                    .filter(|f| {
                        f.project_dir.as_ref() == Some(project)
                            && !self.tabs.is_file_open(&f.path)
                    })
                    .take(self.settings.recent_files_limit)
                    .cloned()
                    .collect()
            }
            None => {
                self.recent_files
                    .iter()
                    .filter(|f| {
                        f.project_dir.is_none() && !self.tabs.is_file_open(&f.path)
                    })
                    .take(self.settings.recent_files_limit)
                    .cloned()
                    .collect()
            }
        }
    }

    fn set_status(&mut self, msg: String, _ctx: &egui::Context) {
        self.status_message = msg;
        self.status_message_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs_f64();
    }

    fn should_ignore_dir(&self, path: &std::path::Path) -> bool {
        if !self.settings.search_ignore_dirs_enabled {
            return false;
        }

        let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        for pattern in self.settings.search_ignored_dirs.split(',') {
            let pattern = pattern.trim();
            if pattern.is_empty() {
                continue;
            }

            if pattern.starts_with('*') && pattern.ends_with('*') && pattern.len() > 2 {
                let middle = &pattern[1..pattern.len() - 1];
                if dir_name.contains(middle) {
                    return true;
                }
            } else if pattern.starts_with('*') {
                let suffix = &pattern[1..];
                if dir_name.ends_with(suffix) {
                    return true;
                }
            } else if pattern.ends_with('*') {
                let prefix = &pattern[..pattern.len() - 1];
                if dir_name.starts_with(prefix) {
                    return true;
                }
            } else if dir_name == pattern {
                return true;
            }
        }

        false
    }

    fn open_file(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    self.tabs.open_file(path.clone(), content);
                    self.add_recent_file(path);
                    self.set_status(
                        format!(
                            "Opened: {}",
                            self.tabs
                                .active_tab()
                                .map(|t| t.name.clone())
                                .unwrap_or_default()
                        ),
                        ctx,
                    );
                }
                Err(e) => {
                    self.set_status(format!("Error opening file: {}", e), ctx);
                }
            }
        }
    }

    fn add_recent_project(&mut self, path: std::path::PathBuf) {
        self.recent_projects.retain(|p| p != &path);
        self.recent_projects.insert(0, path);
        self.recent_projects.truncate(MAX_RECENT_PROJECTS);
        self.save_state();
    }

    fn open_folder(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.opened_folder = Some(path.clone());
            self.expanded_folders.insert(path.clone());
            self.add_recent_project(path.clone());
            self.set_status(format!("Opened folder: {}", path.display()), ctx);
        }
    }

    fn open_folder_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.opened_folder = Some(path.clone());
            self.expanded_folders.insert(path.clone());
            self.add_recent_project(path.clone());
        }
    }

    fn open_recent_project(&mut self, path: &std::path::PathBuf) {
        self.opened_folder = Some(path.clone());
        self.expanded_folders.insert(path.clone());
        self.add_recent_project(path.clone());
    }

    fn create_new_file(&mut self) {
        let parent_dir = if let Some(folder) = &self.selected_folder {
            folder.clone()
        } else if let Some(folder) = &self.opened_folder {
            folder.clone()
        } else {
            return;
        };

        // For now, just create a new untitled tab
        // we can mess around with this shit later
        self.tabs.new_tab();
    }

    fn create_new_folder(&mut self) {
        let parent_dir = if let Some(folder) = &self.selected_folder {
            folder.clone()
        } else if let Some(folder) = &self.opened_folder {
            folder.clone()
        } else {
            return;
        };

        // this doesn't do anything right now, will have to work on this
        self.expanded_folders.insert(parent_dir);
    }

    fn save_current_file(&mut self, ctx: &egui::Context) {
        let (path, content, name) = if let Some(tab) = self.tabs.active_tab() {
            let path = tab.path.clone();
            let content = tab.editor.buffer.content().to_string();
            let name = tab.name.clone();
            (path, content, name)
        } else {
            return;
        };

        if let Some(path) = path {
            match std::fs::write(&path, content) {
                Ok(_) => {
                    if let Some(tab) = self.tabs.active_tab_mut() {
                        tab.is_modified = false;
                    }
                    self.set_status(format!("Saved: {}", name), ctx);
                }
                Err(e) => {
                    self.set_status(format!("Error saving file: {}", e), ctx);
                }
            }
        } else {
            self.save_as_current_file(ctx);
        }
    }

    fn save_as_current_file(&mut self, ctx: &egui::Context) {
        if let Some(tab) = self.tabs.active_tab() {
            let content = tab.editor.buffer.content().to_string();
            let suggested_name = tab.name.clone();

            if let Some(path) = rfd::FileDialog::new()
                .set_file_name(&suggested_name)
                .save_file()
            {
                let path: std::path::PathBuf = path;
                match std::fs::write(&path, content) {
                    Ok(_) => {
                        let name = path
                            .file_name()
                            .map(|n: &std::ffi::OsStr| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "untitled".to_string());

                        if let Some(tab) = self.tabs.active_tab_mut() {
                            tab.name = name;
                            tab.path = Some(path);
                            tab.is_modified = false;
                        }
                        self.set_status(
                            format!(
                                "Saved as: {}",
                                self.tabs
                                    .active_tab()
                                    .map(|t| t.name.clone())
                                    .unwrap_or_default()
                            ),
                            ctx,
                        );
                    }
                    Err(e) => {
                        self.set_status(format!("Error saving file: {}", e), ctx);
                    }
                }
            }
        }
    }

    fn show_menu_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar")
            .exact_height(30.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill =
                        CherryBlossomTheme::BG_DARK;

                    egui::menu::bar(ui, |ui| {
                        ui.menu_button("File", |ui| {
                            if ui.button("New File").clicked() {
                                self.tabs.new_tab();
                                self.set_status("New file created".to_string(), ctx);
                                ui.close_menu();
                            }
                            if ui.button("Open File...").clicked() {
                                self.open_file(ctx);
                                ui.close_menu();
                            }
                            if ui.button("Open Folder...").clicked() {
                                self.open_folder(ctx);
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.button("Save").clicked() {
                                self.save_current_file(ctx);
                                ui.close_menu();
                            }
                            if ui.button("Save As...").clicked() {
                                self.save_as_current_file(ctx);
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.button("Quit").clicked() {
                                std::process::exit(0);
                            }
                        });

                        ui.menu_button("Edit", |ui| {
                            if ui.button("Undo").clicked() {
                                ui.close_menu();
                            }
                            if ui.button("Redo").clicked() {
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.button("Cut").clicked() {
                                ui.close_menu();
                            }
                            if ui.button("Copy").clicked() {
                                ui.close_menu();
                            }
                            if ui.button("Paste").clicked() {
                                ui.close_menu();
                            }
                        });

                        ui.menu_button("View", |ui| {
                            let mut settings_changed = false;
                            if ui
                                .checkbox(&mut self.settings.sidebar_visible, "Sidebar")
                                .clicked()
                            {
                                settings_changed = true;
                                ui.close_menu();
                            }
                            if ui
                                .checkbox(&mut self.settings.show_line_numbers, "Line Numbers")
                                .clicked()
                            {
                                settings_changed = true;
                                ui.close_menu();
                            }
                            if ui
                                .checkbox(&mut self.settings.word_wrap, "Word Wrap")
                                .clicked()
                            {
                                settings_changed = true;
                                ui.close_menu();
                            }
                            if ui
                                .checkbox(&mut self.settings.status_bar_visible, "Status Bar")
                                .clicked()
                            {
                                settings_changed = true;
                                ui.close_menu();
                            }
                            if settings_changed {
                                self.settings.save();
                            }
                            ui.separator();
                            if ui.button("Command Palette").clicked() {
                                self.command_palette.toggle();
                                ui.close_menu();
                            }
                            if ui.button("Settings").clicked() {
                                self.tabs.open_settings_tab();
                                ui.close_menu();
                            }
                        });

                        ui.menu_button("Help", |ui| {
                            if ui.button("About AsterIDE").clicked() {
                                self.set_status(
                                    "AsterIDE - Cherry Blossom Edition v0.1.0".to_string(),
                                    ctx,
                                );
                                ui.close_menu();
                            }
                        });
                    });
                });
            });
    }

    fn show_activity_bar(&mut self, ctx: &egui::Context) {
        egui::SidePanel::left("activity_bar")
            .exact_width(50.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.set_height(ui.available_height());
                    ui.style_mut().spacing.item_spacing = egui::vec2(0.0, 10.0);

                    let button_size = egui::vec2(40.0, 40.0);

                    let explorer_active = self.active_sidebar_tab == SidebarTab::Explorer;
                    if self.icon_button(ui, "📁", "Explorer", explorer_active, button_size) {
                        self.toggle_sidebar(SidebarTab::Explorer);
                    }

                    let search_active = self.active_sidebar_tab == SidebarTab::Search;
                    if self.icon_button(ui, "🔍", "Search", search_active, button_size) {
                        self.tabs.open_search_tab();
                    }

                    let git_active = self.active_sidebar_tab == SidebarTab::Git;
                    if self.icon_button(ui, "🌸", "Git", git_active, button_size) {
                        self.toggle_sidebar(SidebarTab::Git);
                    }

                    let ext_active = self.active_sidebar_tab == SidebarTab::Extensions;
                    if self.icon_button(ui, "📦", "Extensions", ext_active, button_size) {
                        self.toggle_sidebar(SidebarTab::Extensions);
                    }

                    ui.add_space(ui.available_height() - 50.0);

                    let settings_active = self
                        .tabs
                        .active_tab()
                        .map(|t| t.tab_type == TabType::Settings)
                        .unwrap_or(false);
                    if self.icon_button(ui, "⚙", "Settings", settings_active, button_size) {
                        self.tabs.open_settings_tab();
                    }
                });
            });
    }

    fn icon_button(
        &self,
        ui: &mut egui::Ui,
        icon: &str,
        _tooltip: &str,
        active: bool,
        size: egui::Vec2,
    ) -> bool {
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        let _visuals = ui.style().interact(&response);
        let bg_color = if active {
            CherryBlossomTheme::BG_LIGHTER
        } else if response.hovered() {
            CherryBlossomTheme::BG_LIGHT
        } else {
            CherryBlossomTheme::BG_DARK
        };

        let fg_color = if active {
            CherryBlossomTheme::ACCENT_PINK
        } else {
            CherryBlossomTheme::TEXT_SECONDARY
        };

        ui.painter().rect_filled(rect, 4.0, bg_color);

        let galley = ui.painter().layout(
            icon.to_string(),
            egui::FontId::new(20.0, egui::FontFamily::Proportional),
            fg_color,
            size.x,
        );

        let text_pos = rect.center() - galley.size() / 2.0;
        ui.painter()
            .galley(text_pos, galley, CherryBlossomTheme::TEXT_PRIMARY);

        response.clicked()
    }

    fn toggle_sidebar(&mut self, tab: SidebarTab) {
        if self.active_sidebar_tab == tab && self.settings.sidebar_visible {
            self.settings.sidebar_visible = false;
        } else {
            self.active_sidebar_tab = tab;
            self.settings.sidebar_visible = true;
        }
        self.settings.save();
    }

    fn show_sidebar(&mut self, ctx: &egui::Context) {
        if !self.settings.sidebar_visible {
            return;
        }

        egui::SidePanel::left("sidebar")
            .exact_width(self.sidebar_width)
            .resizable(true)
            .show(ctx, |ui| {
                ui.set_height(ui.available_height());

                match self.active_sidebar_tab {
                    SidebarTab::Explorer => self.show_explorer(ui),
                    SidebarTab::Search => search::show_search_button(ui),
                    SidebarTab::Git => self.show_git(ui),
                    SidebarTab::Extensions => self.show_extensions(ui),
                }
            });
    }

    fn show_explorer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let heading_text = self
                .opened_folder
                .as_ref()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Explorer".to_string());
            ui.heading(heading_text);
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let button_size = egui::vec2(20.0, 20.0);

                if ui.add_sized(button_size, egui::Button::new("⬇")).clicked() {
                    self.expanded_folders.clear();
                }
                if ui.add_sized(button_size, egui::Button::new("🔄")).clicked() {}
                if ui
                    .add_sized(button_size, egui::Button::new("📁+"))
                    .clicked()
                {
                    self.create_new_folder();
                }
                if ui
                    .add_sized(button_size, egui::Button::new("📄+"))
                    .clicked()
                {
                    self.create_new_file();
                }
            });
        });
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            if let Some(folder) = self.opened_folder.clone() {
                self.show_folder_tree(ui, &folder, 0);
            } else {
                ui.label("No folder opened");
                ui.add_space(10.0);
                if ui.button("📂 Open Folder").clicked() {
                    self.open_folder_dialog();
                }
            }

            ui.add_space(20.0);
            ui.heading("Open Editors");
            ui.separator();

            let tab_count = self.tabs.tabs.len();
            for i in 0..tab_count {
                let tab = &self.tabs.tabs[i];
                let is_active = i == self.tabs.active_tab;
                let prefix = if tab.is_modified { "● " } else { "  " };
                let text = format!("{}{}", prefix, tab.name);

                let label = egui::SelectableLabel::new(is_active, text);
                if ui.add(label).clicked() {
                    self.tabs.set_active(i);
                }
            }
        });
    }

    fn show_folder_tree(&mut self, ui: &mut egui::Ui, path: &std::path::PathBuf, depth: usize) {
        let is_expanded = self.expanded_folders.contains(path);
        let is_dir = path.is_dir();

        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.to_string_lossy().to_string());

        let icon = if is_dir {
            if is_expanded { "📂" } else { "📁" }
        } else {
            "📄"
        };

        let indent = "  ".repeat(depth);
        let text = format!("{}{} {}", indent, icon, name);

        let response = ui.selectable_label(false, &text);

        if response.clicked() {
            if is_dir {
                if is_expanded {
                    self.expanded_folders.remove(path);
                } else {
                    self.expanded_folders.insert(path.clone());
                }
            } else {
                if let Ok(content) = std::fs::read_to_string(&path) {
                    self.tabs.open_file(path.clone(), content);
                    self.add_recent_file(path.to_path_buf());
                }
            }
        }

        if response.middle_clicked() && !is_dir {
            if let Ok(content) = std::fs::read_to_string(&path) {
                self.tabs.open_file_in_background(path.clone(), content);
                self.add_recent_file(path.to_path_buf());
            }
        }

        if is_expanded && is_dir {
            if let Ok(entries) = std::fs::read_dir(path) {
                let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
                // sort directories first, then files, both alphabetically
                // might add exceptions for dotfiles / hidden folders and files
                entries.sort_by(|a, b| {
                    let a_is_dir = a.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    let b_is_dir = b.file_type().map(|t| t.is_dir()).unwrap_or(false);
                    match (a_is_dir, b_is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.file_name().cmp(&b.file_name()),
                    }
                });

                for entry in entries {
                    let child_path = entry.path();
                    self.show_folder_tree(ui, &child_path, depth + 1);
                }
            }
        }
    }

    fn show_git(&mut self, ui: &mut egui::Ui) {
        ui.heading("Source Control");
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("🌸 Commit").clicked() {
                self.set_status("Git commit not yet implemented".to_string(), ui.ctx());
            }
            if ui.button("↻ Refresh").clicked() {
                self.set_status("Git refresh not yet implemented".to_string(), ui.ctx());
            }
        });

        ui.add_space(10.0);
        ui.label("Changes");
        ui.separator();
        ui.label("No changes");
    }

    fn show_extensions(&mut self, ui: &mut egui::Ui) {
        ui.heading("Extensions");
        ui.separator();

        let mut search = String::new();
        ui.text_edit_singleline(&mut search);

        ui.add_space(10.0);
        ui.label("Installed");
        ui.separator();
        ui.label("No extensions installed");
    }

    fn show_tab_bar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("tab_bar")
            .exact_height(35.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.set_width(ui.available_width());

                    let tab_count = self.tabs.tabs.len();
                    let mut tab_to_close: Option<usize> = None;
                    let mut tab_to_activate: Option<usize> = None;

                    for i in 0..tab_count {
                        let tab = &self.tabs.tabs[i];
                        let is_active = i == self.tabs.active_tab;
                        let is_modified = tab.is_modified;

                        let bg_color = if is_active {
                            CherryBlossomTheme::BG_MID
                        } else {
                            CherryBlossomTheme::BG_DARK
                        };

                        let prefix = if is_modified { "● " } else { "" };
                        let label_text = format!("{}{}", prefix, tab.name);

                        let (rect, response) =
                            ui.allocate_exact_size(egui::vec2(120.0, 30.0), egui::Sense::click());

                        ui.painter().rect_filled(rect, 4.0, bg_color);

                        if is_active {
                            ui.painter().line_segment(
                                [rect.left_top(), rect.right_top()],
                                egui::Stroke::new(2.0, CherryBlossomTheme::ACCENT_PINK),
                            );
                        }

                        let galley = ui.painter().layout(
                            label_text.clone(),
                            egui::FontId::new(12.0, egui::FontFamily::Proportional),
                            if is_active {
                                CherryBlossomTheme::TEXT_PRIMARY
                            } else {
                                CherryBlossomTheme::TEXT_SECONDARY
                            },
                            100.0,
                        );

                        let text_pos =
                            rect.left_center() + egui::vec2(10.0, -galley.size().y / 2.0);
                        ui.painter()
                            .galley(text_pos, galley, CherryBlossomTheme::TEXT_PRIMARY);

                        let close_rect = egui::Rect::from_min_size(
                            rect.right_top() - egui::vec2(25.0, 0.0),
                            egui::vec2(20.0, rect.height()),
                        );
                        let close_response = ui.interact(
                            close_rect,
                            egui::Id::new(("close", i)),
                            egui::Sense::click(),
                        );

                        if close_response.hovered() || (is_active && close_response.hovered()) {
                            ui.painter().text(
                                close_rect.center(),
                                egui::Align2::CENTER_CENTER,
                                "×",
                                egui::FontId::new(16.0, egui::FontFamily::Proportional),
                                CherryBlossomTheme::TEXT_PRIMARY,
                            );
                        }

                        if response.clicked() {
                            tab_to_activate = Some(i);
                        }

                        if response.middle_clicked() {
                            tab_to_close = Some(i);
                        }

                        if close_response.clicked() {
                            tab_to_close = Some(i);
                        }
                    }

                    if let Some(i) = tab_to_activate {
                        self.tabs.set_active(i);
                    }

                    if let Some(i) = tab_to_close {
                        self.tabs.close_tab(i);
                    }

                    ui.add_space(5.0);

                    let button_size = egui::vec2(30.0, 30.0);
                    let (rect, response) =
                        ui.allocate_exact_size(button_size, egui::Sense::click());

                    let bg_color = if response.hovered() {
                        CherryBlossomTheme::BG_LIGHT
                    } else {
                        CherryBlossomTheme::BG_DARK
                    };
                    ui.painter().rect_filled(rect, 4.0, bg_color);

                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "+",
                        egui::FontId::new(16.0, egui::FontFamily::Proportional),
                        CherryBlossomTheme::TEXT_PRIMARY,
                    );

                    if response.clicked() {
                        self.tabs.new_tab();
                    }
                });
            });
    }

    fn show_welcome_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(CherryBlossomTheme::BG_DARKEST))
            .show(ctx, |ui| {
                let recent_files_data: Vec<(std::path::PathBuf, String)> = self
                    .get_relevant_recent_files()
                    .into_iter()
                    .take(self.settings.recent_files_limit)
                    .map(|f| {
                        let name = f
                            .path
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| f.path.display().to_string());
                        (f.path, name)
                    })
                    .collect();
                let recent_projects_data: Vec<(std::path::PathBuf, String)> = self
                    .recent_projects
                    .iter()
                    .take(self.settings.recent_projects_limit)
                    .map(|p| {
                        let name = p
                            .file_name()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| p.display().to_string());
                        (p.clone(), name)
                    })
                    .collect();
                let has_recent_projects = !recent_projects_data.is_empty();
                let has_recent_files = !recent_files_data.is_empty();
                let has_project_folder = self.opened_folder.is_some();
                let has_any_recents = has_recent_files || has_recent_projects;

                ui.vertical_centered(|ui| {
                    ui.add_space(ui.available_height() * 0.08);
                    ui.heading(
                        egui::RichText::new("AsterIDE 🌸")
                            .size(48.0)
                            .color(CherryBlossomTheme::ACCENT_PINK),
                    );
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("A Simple Text Editor written in Rust.")
                            .size(16.0)
                            .color(CherryBlossomTheme::TEXT_SECONDARY),
                    );
                });

                ui.add_space(60.0);

                ui.horizontal(|ui| {
                    let total_width = ui.available_width();
                    let left_width = if has_any_recents {
                        total_width * 0.4
                    } else {
                        total_width
                    };
                    let right_width = total_width * 0.55;

                    ui.allocate_ui_with_layout(
                        egui::vec2(left_width, ui.available_height()),
                        egui::Layout::top_down(egui::Align::Center),
                        |ui| {
                            egui::Frame::group(&ui.style())
                                .fill(CherryBlossomTheme::BG_DARK)
                                .inner_margin(20.0)
                                .show(ui, |ui| {
                                    let button_size = egui::vec2(200.0, 40.0);
                                    if ui
                                        .add_sized(button_size, egui::Button::new("📄  Open File"))
                                        .clicked()
                                    {
                                        self.open_file(ctx);
                                    }
                                    ui.add_space(10.0);
                                    if ui
                                        .add_sized(button_size, egui::Button::new("📁  Open Folder"))
                                        .clicked()
                                    {
                                        self.open_folder(ctx);
                                    }
                                    ui.add_space(10.0);
                                    if ui
                                        .add_sized(button_size, egui::Button::new("📝  New File"))
                                        .clicked()
                                    {
                                        self.tabs.new_tab();
                                    }
                                });
                        },
                    );

                    if has_any_recents {
                        ui.add_space(20.0);
                        ui.vertical(|ui| {
                            let available_height = ui.available_height();
                            ui.add_space(available_height * 0.1);
                            ui.add(egui::Separator::default().vertical().spacing(0.0));
                            ui.add_space(available_height * 0.1);
                        });
                        ui.add_space(20.0);

                        ui.allocate_ui_with_layout(
                            egui::vec2(right_width, ui.available_height()),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| {
                                egui::Frame::group(&ui.style())
                                    .fill(CherryBlossomTheme::BG_DARK)
                                    .inner_margin(16.0)
                                    .show(ui, |ui| {
                                        ui.set_width(right_width - 32.0);

                                        if has_recent_files {
                                            let title = if has_project_folder {
                                                "Recent Files in Project"
                                            } else {
                                                "Recent Files"
                                            };
                                            ui.label(
                                                egui::RichText::new(title)
                                                    .size(16.0)
                                                    .color(CherryBlossomTheme::TEXT_PRIMARY),
                                            );
                                            ui.add_space(10.0);

                                            let mut clicked_file: Option<std::path::PathBuf> = None;
                                            for (path, name) in &recent_files_data {
                                                let file_path_str = path.display().to_string();
                                                let response = ui.add(
                                                    egui::Button::new(
                                                        egui::RichText::new(format!("📄  {}", file_path_str))
                                                            .color(CherryBlossomTheme::TEXT_PRIMARY)
                                                            .size(12.0),
                                                    )
                                                    .fill(CherryBlossomTheme::BG_MID)
                                                    .min_size(egui::vec2(right_width - 50.0, 30.0)),
                                                );

                                                if response.clicked() {
                                                    clicked_file = Some(path.clone());
                                                }
                                                response.on_hover_text(name.clone());
                                            }

                                            if let Some(path) = clicked_file {
                                                if let Ok(content) = std::fs::read_to_string(&path) {
                                                    self.tabs.open_file(path.clone(), content);
                                                    self.add_recent_file(path);
                                                }
                                            }

                                            if has_recent_projects {
                                                ui.add_space(20.0);
                                                ui.separator();
                                                ui.add_space(10.0);
                                            }
                                        }

                                        if has_recent_projects {
                                            ui.label(
                                                egui::RichText::new("Recent Projects")
                                                    .size(16.0)
                                                    .color(CherryBlossomTheme::TEXT_PRIMARY),
                                            );
                                            ui.add_space(10.0);

                                            let mut clicked_project: Option<std::path::PathBuf> = None;
                                            for (path, name) in &recent_projects_data {
                                                let project_path_str = path.display().to_string();
                                                let response = ui.add(
                                                    egui::Button::new(
                                                        egui::RichText::new(format!("📁  {}", project_path_str))
                                                            .color(CherryBlossomTheme::TEXT_PRIMARY)
                                                            .size(12.0),
                                                    )
                                                    .fill(CherryBlossomTheme::BG_MID)
                                                    .min_size(egui::vec2(right_width - 50.0, 30.0)),
                                                );

                                                if response.clicked() {
                                                    clicked_project = Some(path.clone());
                                                }
                                                response.on_hover_text(name.clone());
                                            }

                                            if let Some(project) = clicked_project {
                                                self.open_recent_project(&project);
                                            }
                                        }
                                    });
                            },
                        );
                    }
                });
            });
    }

    fn show_editor(&mut self, ctx: &egui::Context) {
        if self.tabs.is_empty() {
            self.show_welcome_screen(ctx);
            return;
        }

        let active_tab_type = self
            .tabs
            .active_tab()
            .map(|t| t.tab_type)
            .unwrap_or(TabType::File);

        if active_tab_type == TabType::Settings {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::central_panel(&ctx.style()).fill(CherryBlossomTheme::BG_DARKEST),
                )
                .show(ctx, |ui| {
                    ui.set_height(ui.available_height());
                    self.settings.show_content(ui);
                });
            self.settings.save();

            if self.settings.edit_as_json_clicked {
                self.settings.edit_as_json_clicked = false;
                if let Some(path) = settings::get_settings_file_path() {
                    self.settings.save();
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        self.tabs.open_file(path.clone(), content);
                        self.add_recent_file(path);
                    }
                }
            }
            return;
        }

        if active_tab_type == TabType::SearchResults {
            egui::CentralPanel::default()
                .frame(
                    egui::Frame::central_panel(&ctx.style()).fill(CherryBlossomTheme::BG_DARKEST),
                )
                .show(ctx, |ui| {
                    let mut state: search::SearchState = ui.ctx().data_mut(|d| {
                        d.get_temp(egui::Id::new("search_state"))
                            .unwrap_or_default()
                    });
                    search::show_search_tab(ui, &mut state, self.settings.search_min_chars);
                    ui.ctx().data_mut(|d| {
                        d.insert_temp(egui::Id::new("search_state"), state);
                    });
                });
            return;
        }

        let (content, line_count) = if let Some(editor) = self.tabs.current_editor() {
            let content = editor.buffer.content().to_string();
            let line_count = content.lines().count().max(1);
            let line_count = if content.ends_with('\n') {
                line_count + 1
            } else {
                line_count
            };
            (content, line_count)
        } else {
            (String::new(), 1)
        };

        let _show_line_numbers = self.settings.show_line_numbers;
        let font_size = self.settings.font_size;
        let _line_number_width = if _show_line_numbers { 50.0 } else { 0.0 };

        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(CherryBlossomTheme::BG_DARKEST))
            .show(ctx, |ui| {
                let mut text_changed = false;
                let mut new_text = content.clone();


                let mut editor = egui_code_editor::CodeEditor::default()
                    .id_source("code_editor")
                    .with_fontsize(font_size)
                    .with_theme(egui_code_editor::ColorTheme::SONOKAI)
                    .vscroll(true);

                let response = editor.show(ui, &mut new_text);

                if response.response.has_focus() {
                    self.editor_had_focus = true;
                }

                if new_text != content {
                    text_changed = true;
                }

                if text_changed {
                    if let Some(editor) = self.tabs.current_editor_mut() {
                        editor.buffer = core::buffer::Buffer::from_str(&new_text);
                    }
                    if let Some(tab) = self.tabs.active_tab_mut() {
                        tab.is_modified = true;
                    }
                }
            });
    }

    fn show_status_bar(&mut self, ctx: &egui::Context) {
        if !self.settings.status_bar_visible {
            return;
        }

        egui::TopBottomPanel::bottom("status_bar")
            .exact_height(22.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.set_width(ui.available_width());

                    ui.label(
                        egui::RichText::new(&self.status_message)
                            .size(11.0)
                            .color(CherryBlossomTheme::TEXT_SECONDARY),
                    );

                    ui.add_space(ui.available_width() - 250.0);

                    if let Some(tab) = self.tabs.active_tab() {
                        ui.label(
                            egui::RichText::new(format!("{} Ln, Col {}", 1, 1))
                                .size(11.0)
                                .color(CherryBlossomTheme::TEXT_MUTED),
                        );

                        ui.add_space(15.0);

                        let indent_text = if self.settings.use_spaces {
                            format!("Spaces: {}", self.settings.tab_size)
                        } else {
                            "Tab Size".to_string()
                        };
                        ui.label(
                            egui::RichText::new(indent_text)
                                .size(11.0)
                                .color(CherryBlossomTheme::TEXT_MUTED),
                        );

                        ui.add_space(15.0);

                        ui.label(
                            egui::RichText::new("UTF-8")
                                .size(11.0)
                                .color(CherryBlossomTheme::TEXT_MUTED),
                        );

                        ui.add_space(15.0);

                        if tab.is_modified {
                            ui.label(
                                egui::RichText::new("● Modified")
                                    .size(11.0)
                                    .color(CherryBlossomTheme::ACCENT_HOT),
                            );
                        }
                    }
                });
            });
    }
}

impl eframe::App for AsterIDE {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.set_height(ui.available_height());

        let ctx = ui.ctx();
        CherryBlossomTheme::apply(ctx);

        if self.editor_had_focus {
            let tab_pressed = ctx.input(|i| {
                i.key_pressed(egui::Key::Tab)
                    && !i.modifiers.shift
                    && !i.modifiers.alt
                    && !i.modifiers.ctrl
                    && !i.modifiers.command
            });
            if tab_pressed {
                if let Some(id) = self.editor_id {
                    ctx.memory_mut(|mem| mem.request_focus(id));
                }
            }
        }

        self.editor_had_focus = false;

        ctx.input(|i| {
            if i.modifiers.command && i.key_pressed(egui::Key::P) {
                self.command_palette.toggle();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::Comma) {
                self.tabs.open_settings_tab();
            }
            if i.modifiers.command && i.modifiers.shift && i.key_pressed(egui::Key::F) {
                self.tabs.open_search_tab();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                self.save_current_file(ctx);
            }
            // I don't use Windows so I'll probably need to use a VM just to ensure this does work
            if !cfg!(target_os = "macos") && i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                self.save_current_file(ctx);
            }
            if i.modifiers.command && i.key_pressed(egui::Key::O) {
                if i.modifiers.shift {
                    self.open_folder_dialog();
                } else {
                    self.open_file(ctx);
                }
            }
            if i.modifiers.command && i.key_pressed(egui::Key::T) {
                self.tabs.new_tab();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::W) {
                self.tabs.close_active_tab();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::B) {
                self.settings.sidebar_visible = !self.settings.sidebar_visible;
                self.settings.save();
            }
        });

        let global_search_triggered = ctx.data_mut(|d| {
            d.get_temp::<bool>(egui::Id::new("global_search_triggered"))
                .unwrap_or(false)
        });
        if global_search_triggered {
            ctx.data_mut(|d| {
                if let Some(mut state) =
                    d.get_temp::<search::SearchState>(egui::Id::new("search_state"))
                {
                    state.results.clear();

                    let mut searched_files: std::collections::HashSet<std::path::PathBuf> =
                        std::collections::HashSet::new();

                    for tab in self.tabs.iter() {
                        if tab.tab_type == TabType::File {
                            if let Some(ref path) = tab.path {
                                searched_files.insert(path.clone());
                                let content = tab.editor.buffer.content().to_string();
                                let lines: Vec<String> =
                                    content.lines().map(|s: &str| s.to_string()).collect();
                                state.find_in_file(&path.display().to_string(), &lines);
                            }
                        }
                    }

                    if let Some(ref folder) = self.opened_folder {
                        let mut walker = walkdir::WalkDir::new(folder).into_iter();

                        while let Some(entry) = walker.next() {
                            let Ok(entry) = entry else { continue };
                            let path = entry.path();

                            if entry.file_type().is_dir() {
                                if self.should_ignore_dir(path) {
                                    walker.skip_current_dir();
                                }
                                continue;
                            }

                            if !entry.file_type().is_file() {
                                continue;
                            }

                            if searched_files.contains(path) {
                                continue;
                            }

                            if let Some(ext) = path.extension() {
                                let ext = ext.to_string_lossy().to_lowercase();
                                if ![
                                    "txt", "rs", "md", "toml", "json", "js", "ts", "html", "css",
                                    "py", "c", "cpp", "h", "hpp", "go", "java", "rb", "sh", "yml",
                                    "yaml",
                                ]
                                .contains(&ext.as_str())
                                {
                                    continue;
                                }
                            }

                            if let Ok(content) = std::fs::read_to_string(path) {
                                searched_files.insert(path.to_path_buf());
                                let lines: Vec<String> =
                                    content.lines().map(|s: &str| s.to_string()).collect();
                                let rel_path = path
                                    .strip_prefix(folder)
                                    .map(|p| p.display().to_string())
                                    .unwrap_or_else(|_| path.display().to_string());
                                state.find_in_file(&rel_path, &lines);
                            }
                        }
                    }

                    d.insert_temp(egui::Id::new("search_state"), state);
                }
                d.insert_temp(egui::Id::new("global_search_triggered"), false);
            });
        }

        let global_replace_triggered = ctx.data_mut(|d| {
            d.get_temp::<bool>(egui::Id::new("global_replace_all_triggered"))
                .unwrap_or(false)
        });
        if global_replace_triggered {
            if let Some(state) =
                ctx.data_mut(|d| d.get_temp::<search::SearchState>(egui::Id::new("search_state")))
            {
                for tab in self.tabs.iter_mut() {
                    if tab.tab_type == TabType::File {
                        let content = tab.editor.buffer.content().to_string();
                        let new_content = state.replace_all_in_text(&content);
                        if new_content != content {
                            tab.editor.buffer = core::buffer::Buffer::from_str(&new_content);
                            tab.is_modified = true;
                        }
                    }
                }
            }
            ctx.data_mut(|d| {
                d.insert_temp(egui::Id::new("global_replace_all_triggered"), false);
            });
        }

        self.show_menu_bar(ctx);
        self.show_activity_bar(ctx);
        self.show_sidebar(ctx);
        self.show_tab_bar(ctx);
        self.show_status_bar(ctx);
        self.show_editor(ctx);

        self.command_palette.show(ctx);
    }
}

fn main() -> eframe::Result<()> {
    let icon = load_icon();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "AsterIDE",
        options,
        Box::new(|_cc| Ok(Box::new(AsterIDE::default()))),
    )
}

fn load_icon() -> egui::IconData {
    // All these appIcons were made using Apples Icon Composer.
    static ICON_DARK: &[u8] =
        include_bytes!("../../../assets/appIcon/asteride-macOS-Dark-1024x1024@1x.png");
    static ICON_DEFAULT: &[u8] =
        include_bytes!("../../../assets/appIcon/asteride-macOS-Default-1024x1024@1x.png");
    static ICON_TINTED_DARK: &[u8] =
        include_bytes!("../../../assets/appIcon/asteride-macOS-TintedDark-1024x1024@1x.png");
    static ICON_TINTED_LIGHT: &[u8] =
        include_bytes!("../../../assets/appIcon/asteride-macOS-TintedLight-1024x1024@1x.png");
    static ICON_CLEAR_DARK: &[u8] =
        include_bytes!("../../../assets/appIcon/asteride-macOS-ClearDark-1024x1024@1x.png");
    static ICON_CLEAR_LIGHT: &[u8] =
        include_bytes!("../../../assets/appIcon/asteride-macOS-ClearLight-1024x1024@1x.png");

    // System Appearance detection needs to be worked on because it does detect light and dark correctly, however...
    let is_dark = is_system_dark_mode();
    let is_tinted = is_system_tinted();
    let is_clear = is_system_clear();

    // The same can't be said for this, this doesn't seem to detect whether icons are Clear, or tinted.
    let icon_bytes = if is_tinted {
        if is_dark {
            ICON_TINTED_DARK
        } else {
            ICON_TINTED_LIGHT
        }
    } else if is_clear {
        if is_dark {
            ICON_CLEAR_DARK
        } else {
            ICON_CLEAR_LIGHT
        }
    } else {
        if is_dark { ICON_DARK } else { ICON_DEFAULT }
    };

    // This means that the icon doesn't currently change properly.
    //
    // TODO: Fix the System Appearance Detection for macOS, might make a seperate folder for this, probably in
    // Objective-C or Swift, if Swift works well will this project anyways. We will have to see.
    //  ----------------------------------------------------------------------------------------------------- //
    // Default Icons:
    //  assets/appIcon/asteride-macOS-Default-1024x1024@1x.png: Light Mode enabled with the Default Icons.
    //  assets/appIcon/asteride-macOS-Dark-1024x1024@1x.png: Dark Mode enabled with the Default Icons.
    // Clear Icons:
    //  assets/appIcon/asteride-macOS-ClearLight-1024x1024@1x.png: Light Mode enabled with Clear Icons.
    //  assets/appIcon/asteride-macOS-ClearDark-1024x1024@1x.png: Dark Mode enabled with Clear Icons.
    // Tinted Icons:
    //  assets/appIcon/asteride-macOS-TintedLight-1024x1024@1x.png: Light Mode enabled with the Tinted Icons.
    //  assets/appIcon/asteride-macOS-TintedDark-1024x1024@1x.png: Dark Mode enabled with the Tinted Icons.

    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .into_rgba8();
    let (width, height) = image.dimensions();

    egui::IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

#[cfg(target_os = "macos")]
fn is_system_dark_mode() -> bool {
    use std::process::Command;

    match Command::new("defaults")
        .args(&["read", "-g", "AppleInterfaceStyle"])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.trim() == "Dark"
        }
        Err(_) => false,
    }
}

#[cfg(target_os = "macos")]
fn is_system_tinted() -> bool {
    // Checks if the System is using the 'Tinted' Appearance, currently doesn't work
    // Not sure if I need to do this in ObjC or Swift, or if I can do it in rust.
    // This should check for accent colors as well.. but the icon isn't dynamic to do that.
    use std::process::Command;

    match Command::new("defaults")
        .args(&["read", "-g", "AppleAccentColor"])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            // If a specific accent color is set, just consider it 'tinted', if it was Swift, I'm sure this would be easier
            !stdout.trim().is_empty() && stdout.trim() != "null"
        }
        Err(_) => false,
    }
}

#[cfg(target_os = "macos")]
fn is_system_clear() -> bool {
    // Check for reduced transparency / clear appearance.
    // This actually should work, I'll probably rewrite this logic either way though
    // for when I fix the actual system and make it work properly.
    use std::process::Command;

    match Command::new("defaults")
        .args(&["read", "com.apple.universalaccess", "reduceTransparency"])
        .output()
    {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            stdout.trim() == "1"
        }
        Err(_) => false,
    }
}

// On non-macOS platforms, these functions are defined but do not perform any logic.
// honestly it'd be funny if I made these do something, specifically with a
// custom wayland compositor (the one I'm making in rust.
// anyways, for now it just returns false because that makes sense.
#[cfg(not(target_os = "macos"))]
fn is_system_dark_mode() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
fn is_system_tinted() -> bool {
    false
}

#[cfg(not(target_os = "macos"))]
fn is_system_clear() -> bool {
    false
}
