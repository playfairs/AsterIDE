mod theme;
mod tabs;
mod settings;

use eframe::egui;
use theme::CherryBlossomTheme;
use tabs::{TabManager, TabType};
use settings::Settings;


struct AsterIDE {
    tabs: TabManager,
    settings: Settings,
    sidebar_width: f32,
    active_sidebar_tab: SidebarTab,
    command_palette_open: bool,
    command_palette_query: String,
    status_message: String,
    status_message_time: f64,
    opened_folder: Option<std::path::PathBuf>,
    expanded_folders: std::collections::HashSet<std::path::PathBuf>,
    selected_folder: Option<std::path::PathBuf>,
    editor_had_focus: bool,
    editor_id: Option<egui::Id>,
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
        Self {
            tabs: TabManager::new(),
            settings: Settings::default(),
            sidebar_width: 250.0,
            active_sidebar_tab: SidebarTab::Explorer,
            command_palette_open: false,
            command_palette_query: String::new(),
            status_message: "Ready".to_string(),
            status_message_time: 0.0,
            opened_folder: None,
            expanded_folders: std::collections::HashSet::new(),
            selected_folder: None,
            editor_had_focus: false,
            editor_id: None,
        }
    }
}

impl AsterIDE {
    fn set_status(&mut self, msg: String, ctx: &egui::Context) {
        self.status_message = msg;
        self.status_message_time = ctx.input(|i| i.time);
    }

    fn open_file(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new().pick_file() {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    self.tabs.open_file(path, content);
                    self.set_status(format!("Opened: {}", self.tabs.active_tab().map(|t| t.name.clone()).unwrap_or_default()), ctx);
                }
                Err(e) => {
                    self.set_status(format!("Error opening file: {}", e), ctx);
                }
            }
        }
    }

    fn open_folder(&mut self, ctx: &egui::Context) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.opened_folder = Some(path.clone());
            self.expanded_folders.insert(path.clone());
            self.set_status(format!("Opened folder: {}", path.display()), ctx);
        }
    }
    
    fn open_folder_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
            self.opened_folder = Some(path.clone());
            self.expanded_folders.insert(path.clone());
        }
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
                        let name = path.file_name()
                            .map(|n: &std::ffi::OsStr| n.to_string_lossy().to_string())
                            .unwrap_or_else(|| "untitled".to_string());
                        
                        if let Some(tab) = self.tabs.active_tab_mut() {
                            tab.name = name;
                            tab.path = Some(path);
                            tab.is_modified = false;
                        }
                        self.set_status(format!("Saved as: {}", self.tabs.active_tab().map(|t| t.name.clone()).unwrap_or_default()), ctx);
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
                    ui.style_mut().visuals.widgets.inactive.weak_bg_fill = CherryBlossomTheme::BG_DARK;
                    
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
                            if ui.checkbox(&mut self.settings.sidebar_visible, "Sidebar").clicked() {
                                ui.close_menu();
                            }
                            if ui.checkbox(&mut self.settings.show_line_numbers, "Line Numbers").clicked() {
                                ui.close_menu();
                            }
                            if ui.checkbox(&mut self.settings.word_wrap, "Word Wrap").clicked() {
                                ui.close_menu();
                            }
                            if ui.checkbox(&mut self.settings.status_bar_visible, "Status Bar").clicked() {
                                ui.close_menu();
                            }
                            ui.separator();
                            if ui.button("Command Palette").clicked() {
                                self.command_palette_open = true;
                                ui.close_menu();
                            }
                            if ui.button("Settings").clicked() {
                                self.tabs.open_settings_tab();
                                ui.close_menu();
                            }
                        });

                        ui.menu_button("Help", |ui| {
                            if ui.button("About AsterIDE").clicked() {
                                self.set_status("AsterIDE - Cherry Blossom Edition v0.1.0".to_string(), ctx);
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
                        self.toggle_sidebar(SidebarTab::Search);
                    }
                    
                    let git_active = self.active_sidebar_tab == SidebarTab::Git;
                    if self.icon_button(ui, "🌸", "Git", git_active, button_size) {
                        self.toggle_sidebar(SidebarTab::Git);
                    }
                    
                    let ext_active = self.active_sidebar_tab == SidebarTab::Extensions;
                    if self.icon_button(ui, "📦", "Extensions", ext_active, button_size) {
                        self.toggle_sidebar(SidebarTab::Extensions);
                    }
                    
                    ui.add_space(ui.available_height() - 60.0);
                    
                    let settings_active = self.tabs.active_tab()
                        .map(|t| t.tab_type == TabType::Settings)
                        .unwrap_or(false);
                    if self.icon_button(ui, "⚙", "Settings", settings_active, button_size) {
                        self.tabs.open_settings_tab();
                    }
                });
            });
    }

    fn icon_button(&self, ui: &mut egui::Ui, icon: &str, _tooltip: &str, active: bool, size: egui::Vec2) -> bool {
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
        ui.painter().galley(text_pos, galley, CherryBlossomTheme::TEXT_PRIMARY);
        
        response.clicked()
    }

    fn toggle_sidebar(&mut self, tab: SidebarTab) {
        if self.active_sidebar_tab == tab && self.settings.sidebar_visible {
            self.settings.sidebar_visible = false;
        } else {
            self.active_sidebar_tab = tab;
            self.settings.sidebar_visible = true;
        }
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
                    SidebarTab::Search => self.show_search(ui),
                    SidebarTab::Git => self.show_git(ui),
                    SidebarTab::Extensions => self.show_extensions(ui),
                }
            });
    }

    fn show_explorer(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Explorer");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let button_size = egui::vec2(20.0, 20.0);
                
                if ui.add_sized(button_size, egui::Button::new("⬇")).clicked() {
                    self.expanded_folders.clear();
                }
                if ui.add_sized(button_size, egui::Button::new("🔄")).clicked() {
                }
                if ui.add_sized(button_size, egui::Button::new("📁+")).clicked() {
                    self.create_new_folder();
                }
                if ui.add_sized(button_size, egui::Button::new("📄+")).clicked() {
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
        
        let name = path.file_name()
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
                if let Ok(content) = std::fs::read_to_string(path) {
                    self.tabs.open_file(path.clone(), content);
                }
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

    fn show_search(&mut self, ui: &mut egui::Ui) {
        ui.heading("Search");
        ui.separator();
        
        let mut search_query = String::new();
        ui.text_edit_singleline(&mut search_query);
        
        if ui.button("Search").clicked() {
            self.set_status(format!("Searching for: {}", search_query), ui.ctx());
        }
        
        ui.add_space(10.0);
        ui.label("Replace");
        let mut replace_query = String::new();
        ui.text_edit_singleline(&mut replace_query);
        
        if ui.button("Replace All").clicked() {
            self.set_status("Replace functionality not yet implemented".to_string(), ui.ctx());
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
                        
                        let (rect, response) = ui.allocate_exact_size(
                            egui::vec2(120.0, 30.0),
                            egui::Sense::click(),
                        );
                        
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
                            if is_active { CherryBlossomTheme::TEXT_PRIMARY } else { CherryBlossomTheme::TEXT_SECONDARY },
                            100.0,
                        );
                        
                        let text_pos = rect.left_center() + egui::vec2(10.0, -galley.size().y / 2.0);
                        ui.painter().galley(text_pos, galley, CherryBlossomTheme::TEXT_PRIMARY);
                        
                        let close_rect = egui::Rect::from_min_size(
                            rect.right_top() - egui::vec2(25.0, 0.0),
                            egui::vec2(20.0, rect.height()),
                        );
                        let close_response = ui.interact(close_rect, egui::Id::new(("close", i)), egui::Sense::click());
                        
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
                    
                    if ui.button("+").clicked() {
                        self.tabs.new_tab();
                    }
                });
            });
    }

    fn show_editor(&mut self, ctx: &egui::Context) {
        let is_settings_tab = self.tabs.active_tab()
            .map(|t| t.tab_type == TabType::Settings)
            .unwrap_or(false);
        
        if is_settings_tab {
            egui::CentralPanel::default()
                .frame(egui::Frame::central_panel(&ctx.style()).fill(CherryBlossomTheme::BG_DARKEST))
                .show(ctx, |ui| {
                    ui.set_height(ui.available_height());
                    self.settings.show_content(ui);
                });
            return;
        }
        
        let (content, line_count) = if let Some(editor) = self.tabs.current_editor() {
            let content = editor.buffer.content().to_string();
            let line_count = content.lines().count().max(1);
            let line_count = if content.ends_with('\n') { line_count + 1 } else { line_count };
            (content, line_count)
        } else {
            (String::new(), 1)
        };
        
        let show_line_numbers = self.settings.show_line_numbers;
        let font_size = self.settings.font_size;
        let line_number_width = if show_line_numbers { 50.0 } else { 0.0 };
        
        egui::CentralPanel::default()
            .frame(egui::Frame::central_panel(&ctx.style()).fill(CherryBlossomTheme::BG_DARKEST))
            .show(ctx, |ui| {
                let mut text_changed = false;
                let mut new_text = content.clone();
                
                let available_height = ui.available_height();
                let available_width = ui.available_width();
                
                let line_height = font_size * 1.2;
                let content_height = (line_count as f32 * line_height).max(available_height);
                let editor_width = available_width - line_number_width;
                
                egui::ScrollArea::vertical()
                    .id_salt("editor_scroll")
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        ui.set_height(content_height);
                        ui.set_width(available_width);
                        
                        ui.horizontal(|ui| {
                            ui.set_height(content_height);
                            
                            if show_line_numbers {
                                ui.vertical(|ui| {
                                    ui.set_width(line_number_width);
                                    ui.set_height(content_height);
                                    
                                    for i in 1..=line_count.max(1) {
                                        ui.with_layout(
                                            egui::Layout::right_to_left(egui::Align::Min),
                                            |ui| {
                                                ui.set_height(line_height);
                                                ui.vertical_centered(|ui| {
                                                    ui.label(
                                                        egui::RichText::new(format!("{}", i))
                                                            .monospace()
                                                            .color(CherryBlossomTheme::TEXT_MUTED)
                                                            .size(font_size)
                                                    );
                                                });
                                            },
                                        );
                                    }
                                });
                            }
                            
                            ui.vertical(|ui| {
                                ui.set_width(editor_width);
                                ui.set_height(content_height);
                                
                                let editor_id = ui.id().with("editor_text");
                                let text_edit = egui::TextEdit::multiline(&mut new_text)
                                    .id(editor_id)
                                    .font(egui::FontId::monospace(font_size))
                                    .text_color(CherryBlossomTheme::TEXT_PRIMARY)
                                    .desired_width(editor_width);
                                
                                let response = ui.add_sized(
                                    egui::vec2(editor_width, content_height),
                                    text_edit
                                );
                                
                                if response.has_focus() {
                                    self.editor_had_focus = true;
                                    self.editor_id = Some(editor_id);
                                }
                            });
                        });
                    });
                
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
                            .color(CherryBlossomTheme::TEXT_SECONDARY)
                    );
                    
                    ui.add_space(ui.available_width() - 250.0);
                    
                    if let Some(tab) = self.tabs.active_tab() {
                        ui.label(
                            egui::RichText::new(format!("{} Ln, Col {}", 1, 1))
                                .size(11.0)
                                .color(CherryBlossomTheme::TEXT_MUTED)
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
                                .color(CherryBlossomTheme::TEXT_MUTED)
                        );
                        
                        ui.add_space(15.0);
                        
                        ui.label(
                            egui::RichText::new("UTF-8")
                                .size(11.0)
                                .color(CherryBlossomTheme::TEXT_MUTED)
                        );
                        
                        ui.add_space(15.0);
                        
                        if tab.is_modified {
                            ui.label(
                                egui::RichText::new("● Modified")
                                    .size(11.0)
                                    .color(CherryBlossomTheme::ACCENT_HOT)
                            );
                        }
                    }
                });
            });
    }

    fn show_command_palette(&mut self, ctx: &egui::Context) {
        if !self.command_palette_open {
            return;
        }

        egui::Window::new("")
            .anchor(egui::Align2::CENTER_TOP, [0.0, 100.0])
            .fixed_size([500.0, 400.0])
            .collapsible(false)
            .title_bar(false)
            .show(ctx, |ui| {
                ui.set_height(ui.available_height());
                
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(">")
                            .size(20.0)
                            .color(CherryBlossomTheme::ACCENT_PINK)
                    );
                    
                    let response = ui.text_edit_singleline(&mut self.command_palette_query);
                    response.request_focus();
                    
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.command_palette_open = false;
                    }
                });
                
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let commands = vec![
                        ("New File", "Create a new file"),
                        ("Open File", "Open an existing file"),
                        ("Save", "Save current file"),
                        ("Save As", "Save file with new name"),
                        ("Close Tab", "Close current tab"),
                        ("Toggle Sidebar", "Show/hide sidebar"),
                        ("Toggle Line Numbers", "Show/hide line numbers"),
                        ("Toggle Word Wrap", "Enable/disable word wrap"),
                        ("Settings", "Open settings panel"),
                        ("Command Palette", "Open command palette"),
                    ];
                    
                    for (name, desc) in commands {
                        if self.command_palette_query.is_empty()
                            || name.to_lowercase().contains(&self.command_palette_query.to_lowercase())
                            || desc.to_lowercase().contains(&self.command_palette_query.to_lowercase())
                        {
                            ui.horizontal(|ui| {
                                ui.set_width(ui.available_width());
                                
                                ui.label(
                                    egui::RichText::new(name)
                                        .size(14.0)
                                        .color(CherryBlossomTheme::TEXT_PRIMARY)
                                );
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(
                                        egui::RichText::new(desc)
                                            .size(12.0)
                                            .color(CherryBlossomTheme::TEXT_MUTED)
                                    );
                                });
                            });
                            
                            ui.separator();
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
                i.key_pressed(egui::Key::Tab) && !i.modifiers.shift && !i.modifiers.alt && !i.modifiers.ctrl && !i.modifiers.command
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
                self.command_palette_open = !self.command_palette_open;
                if self.command_palette_open {
                    self.command_palette_query.clear();
                }
            }
            if i.modifiers.command && i.key_pressed(egui::Key::Comma) {
                self.tabs.open_settings_tab();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::S) {
                self.save_current_file(ctx);
            }
            // I don't use Windows so I'll probably need to use a VM just to ensure this does work
            if !cfg!(target_os = "macos") && i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                self.save_current_file(ctx);
            }
            if i.modifiers.command && i.key_pressed(egui::Key::T) {
                self.tabs.new_tab();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::W) {
                self.tabs.close_active_tab();
            }
            if i.modifiers.command && i.key_pressed(egui::Key::B) {
                self.settings.sidebar_visible = !self.settings.sidebar_visible;
            }
        });
        
        self.show_menu_bar(ctx);
        self.show_activity_bar(ctx);
        self.show_sidebar(ctx);
        self.show_tab_bar(ctx);
        self.show_status_bar(ctx);
        self.show_editor(ctx);
        
        self.show_command_palette(ctx);
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "AsterIDE",
        options,
        Box::new(|_cc| Ok(Box::new(AsterIDE::default()))),
    )
}