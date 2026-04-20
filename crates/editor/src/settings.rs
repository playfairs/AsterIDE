#[derive(Clone, Debug)]
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
        }
    }
}

impl Settings {
    pub fn show_panel(&mut self, ctx: &egui::Context) {
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(true)
            .default_size([400.0, 500.0])
            .show(ctx, |ui| {
                self.show_content(ui);
            });
    }
    
    pub fn show_content(&mut self, ui: &mut egui::Ui) {
        ui.heading("Editor Settings");
        ui.separator();
        
        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.group(|ui| {
                ui.label("Display");
                ui.checkbox(&mut self.show_line_numbers, "Show line numbers");
                ui.checkbox(&mut self.word_wrap, "Word wrap");
                ui.checkbox(&mut self.show_whitespace, "Show whitespace");
            });
            
            ui.add_space(10.0);
            
            ui.group(|ui| {
                ui.label("Font & Indentation");
                ui.horizontal(|ui| {
                    ui.label("Font size:");
                    ui.add(egui::Slider::new(&mut self.font_size, 8.0..=32.0));
                });
                
                ui.horizontal(|ui| {
                    ui.label("Tab size:");
                    ui.add(egui::Slider::new(&mut self.tab_size, 2..=8));
                });
                
                ui.checkbox(&mut self.use_spaces, "Use spaces for indentation");
            });
            
            ui.add_space(10.0);
            
            ui.group(|ui| {
                ui.label("Behavior");
                ui.checkbox(&mut self.vim_mode, "Vim mode");
                ui.checkbox(&mut self.auto_save, "Auto save");
                
                if self.auto_save {
                    ui.horizontal(|ui| {
                        ui.label("Auto save interval (seconds):");
                        ui.add(egui::Slider::new(&mut self.auto_save_interval, 5..=300));
                    });
                }
            });
            
            ui.add_space(10.0);
            
            ui.group(|ui| {
                ui.label("Interface");
                ui.checkbox(&mut self.sidebar_visible, "Show sidebar");
                ui.checkbox(&mut self.status_bar_visible, "Show status bar");
            });
        });
    }
}
