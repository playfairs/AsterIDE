use serde::{Deserialize, Serialize};
use theme::CherryBlossomTheme;

use egui::{Color32, Stroke, Margin};

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum TerminalRightClickAction {
    Paste,
    ContextMenu,
    Nothing,
}

impl Default for TerminalRightClickAction {
    fn default() -> Self {
        TerminalRightClickAction::ContextMenu
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TerminalSettings {
    pub terminal_shell: String,
    pub terminal_font_family: String,
    pub terminal_font_size: f32,
    pub terminal_background_color: String,
    pub terminal_text_color: String,
    pub terminal_cursor_color: String,
    pub terminal_bell_enabled: bool,
    pub terminal_auto_scroll: bool,
    pub terminal_copy_on_selection: bool,
    pub terminal_right_click_action: TerminalRightClickAction,
}

impl Default for TerminalSettings {
    fn default() -> Self {
        Self {
            terminal_shell: std::env::var("SHELL").unwrap_or_else(|_| "/bin/bash".to_string()),
            terminal_font_family: "SF Mono".to_string(),
            terminal_font_size: 14.0,
            terminal_background_color: "#1e1e1e".to_string(),
            terminal_text_color: "#ffffff".to_string(),
            terminal_cursor_color: "#ffffff".to_string(),
            terminal_bell_enabled: true,
            terminal_auto_scroll: true,
            terminal_copy_on_selection: false,
            terminal_right_click_action: TerminalRightClickAction::default(),
        }
    }
}

use crate::Settings;
use crate::ui::{setting_card, cozy_row_filtered, matches_search};

pub fn show_terminal_settings(settings: &mut Settings, ui: &mut egui::Ui, has_search: bool, query: &str) {
    ui.add_space(16.0);
    ui.vertical_centered(|ui| {
        ui.add(
            egui::Label::new(
                egui::RichText::new("⚠️ Terminal Settings Disabled")
                    .size(16.0)
                    .strong()
                    .color(CherryBlossomTheme::ACCENT_PINK())
            ).selectable(false)
        );
        ui.add_space(8.0);
        ui.add(
            egui::Label::new(
                egui::RichText::new("The terminal feature has not yet been implemented.")
                    .size(13.0)
                    .color(CherryBlossomTheme::TEXT_SECONDARY())
            ).selectable(false)
        );
        ui.add(
            egui::Label::new(
                egui::RichText::new("These settings will be available in a future update.")
                    .size(12.0)
                    .color(CherryBlossomTheme::TEXT_MUTED())
            ).selectable(false)
        );
    });
    ui.add_space(24.0);
    ui.separator();
    ui.add_space(16.0);
    
    let query = query.to_lowercase();

    let dimmed_fill = ui.visuals().window_fill.linear_multiply(0.7);
    let dimmed_stroke = ui.visuals().widgets.noninteractive.bg_stroke;
    
    egui::Frame::group(ui.style())
        .fill(dimmed_fill)
        .corner_radius(settings.corner_roundness)
        .stroke(dimmed_stroke)
        .inner_margin(Margin::same(16))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            
            ui.vertical_centered(|ui| {
                ui.add(
                    egui::Label::new(
                        egui::RichText::new("SETTINGS DISABLED")
                            .size(11.0)
                            .strong()
                            .color(ui.visuals().widgets.noninteractive.fg_stroke.color)
                    ).selectable(false)
                );
            });
            ui.add_space(8.0);
            
            if !has_search || matches_search(settings, &query, &["terminal", "shell", "console"]) {
                disabled_card(ui, settings, "Shell", |ui| {
                    disabled_row(ui, "Shell Path", "Path to shell executable", |ui| {
                        ui.add_sized(
                            [120.0, 22.0],
                            egui::TextEdit::singleline(&mut settings.terminal_shell.clone())
                                .hint_text("/bin/bash")
                                .id_salt("disabled_terminal_shell")
                        );
                    });
                });
                ui.add_space(8.0);
            }

            if !has_search || matches_search(settings, &query, &["font", "terminal", "text"]) {
                disabled_card(ui, settings, "Font", |ui| {
                    disabled_row(ui, "Font Family", "Terminal font family", |ui| {
                        ui.add_sized(
                            [120.0, 22.0],
                            egui::TextEdit::singleline(&mut settings.terminal_font_family.clone())
                                .hint_text("SF Mono")
                                .id_salt("disabled_terminal_font_family")
                        );
                    });
                    disabled_row(ui, "Font Size", "Terminal font size in pixels", |ui| {
                        ui.label(
                            egui::RichText::new(format!("{:.0}px", settings.terminal_font_size))
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_MUTED())
                        );
                    });
                });
                ui.add_space(8.0);
            }

            if !has_search || matches_search(settings, &query, &["color", "theme", "terminal"]) {
                disabled_card(ui, settings, "Colors", |ui| {
                    disabled_row(ui, "Background Color", "Terminal background color", |ui| {
                        ui.label(
                            egui::RichText::new(&settings.terminal_background_color)
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_MUTED())
                        );
                    });
                    disabled_row(ui, "Text Color", "Terminal text color", |ui| {
                        ui.label(
                            egui::RichText::new(&settings.terminal_text_color)
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_MUTED())
                        );
                    });
                    disabled_row(ui, "Cursor Color", "Terminal cursor color", |ui| {
                        ui.label(
                            egui::RichText::new(&settings.terminal_cursor_color)
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_MUTED())
                        );
                    });
                });
                ui.add_space(8.0);
            }

            if !has_search || matches_search(settings, &query, &["behavior", "terminal", "bell", "scroll"]) {
                disabled_card(ui, settings, "Behavior", |ui| {
                    disabled_row(ui, "Terminal Bell", "Enable terminal bell sound", |ui| {
                        disabled_checkbox(ui, settings.terminal_bell_enabled);
                    });
                    disabled_row(ui, "Auto-scroll", "Automatically scroll to bottom on new output", |ui| {
                        disabled_checkbox(ui, settings.terminal_auto_scroll);
                    });
                    disabled_row(ui, "Copy on Selection", "Automatically copy selected text", |ui| {
                        disabled_checkbox(ui, settings.terminal_copy_on_selection);
                    });
                    disabled_row(ui, "Right-click Action", "Action when right-clicking in terminal", |ui| {
                        ui.label(
                            egui::RichText::new(format!("{:?}", settings.terminal_right_click_action))
                                .size(12.0)
                                .color(CherryBlossomTheme::TEXT_MUTED())
                        );
                    });
                });
            }
        });
}

fn disabled_card(ui: &mut egui::Ui, _settings: &Settings, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    let card_margin = 12.0;
    let dimmed_card_fill = ui.visuals().extreme_bg_color.linear_multiply(0.85);
    let dimmed_stroke = ui.visuals().widgets.noninteractive.bg_stroke;
    
    egui::Frame::group(ui.style())
        .fill(dimmed_card_fill)
        .corner_radius(4.0)
        .stroke(dimmed_stroke)
        .inner_margin(Margin::same(card_margin as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            
            let title_color = ui.visuals().text_color().linear_multiply(0.6);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(title)
                        .size(13.0)
                        .strong()
                        .color(title_color)
                ).selectable(false)
            );
            
            ui.add_space(8.0);
            
            content(ui);
        });
}

fn disabled_row(ui: &mut egui::Ui, title: &str, description: &str, control: impl FnOnce(&mut egui::Ui)) {
    ui.horizontal(|ui| {
        ui.set_width(ui.available_width());
        
        let text_color = ui.visuals().text_color();
        let muted_color = ui.visuals().widgets.noninteractive.fg_stroke.color;
        
        ui.vertical(|ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(title)
                        .size(12.0)
                        .color(text_color.linear_multiply(0.7))
                ).selectable(false)
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(description)
                        .size(10.0)
                        .color(muted_color)
                ).selectable(false)
            );
        });
        
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            control(ui);
        });
    });
    
    ui.add_space(8.0);
}

fn disabled_checkbox(ui: &mut egui::Ui, checked: bool) {
    let (rect, _response) = ui.allocate_exact_size(egui::vec2(16.0, 16.0), egui::Sense::hover());
    let bg_fill = ui.visuals().extreme_bg_color;
    let bg_color = if checked { 
        ui.visuals().text_color().linear_multiply(0.5)
    } else { 
        bg_fill.linear_multiply(0.8)
    };
    ui.painter().rect_filled(rect, 3.0, bg_color);
    if checked {
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "✓",
            egui::FontId::new(10.0, egui::FontFamily::Proportional),
            ui.visuals().window_fill,
        );
    }
}
