use egui::Context;

pub struct CommandPalette {
    pub open: bool,
    pub query: String,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self {
            open: false,
            query: String::new(),
        }
    }
}

impl CommandPalette {
    pub fn show(&mut self, ctx: &Context) {
        if !self.open {
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
                            .color(crate::theme::CherryBlossomTheme::ACCENT_PINK)
                    );
                    
                    let response = ui.text_edit_singleline(&mut self.query);
                    response.request_focus();
                    
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.open = false;
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
                        if self.query.is_empty()
                            || name.to_lowercase().contains(&self.query.to_lowercase())
                            || desc.to_lowercase().contains(&self.query.to_lowercase())
                        {
                            ui.horizontal(|ui| {
                                ui.set_width(ui.available_width());
                                
                                ui.label(
                                    egui::RichText::new(name)
                                        .size(14.0)
                                        .color(crate::theme::CherryBlossomTheme::TEXT_PRIMARY)
                                );
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    ui.label(
                                        egui::RichText::new(desc)
                                            .size(12.0)
                                            .color(crate::theme::CherryBlossomTheme::TEXT_MUTED)
                                    );
                                });
                            });
                            
                            ui.separator();
                        }
                    }
                });
            });
    }
    
    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open {
            self.query.clear();
        }
    }
}
