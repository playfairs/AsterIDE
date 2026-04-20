//! Cherry Blossom Theme for AsterIDE
// This'll be the default theme for now,
// I need to actually redo the whole editor directory to sort the editor logic
// from the themes and settings logic, and probably other stuff as well

use egui::{Color32, Visuals, TextStyle, FontId};

pub struct CherryBlossomTheme;

impl CherryBlossomTheme {
    pub const PINK_50: Color32 = Color32::from_rgb(255, 250, 252);
    pub const PINK_100: Color32 = Color32::from_rgb(255, 235, 245);
    pub const PINK_200: Color32 = Color32::from_rgb(255, 210, 230);
    pub const PINK_300: Color32 = Color32::from_rgb(255, 180, 210);
    pub const PINK_400: Color32 = Color32::from_rgb(255, 145, 185);
    pub const PINK_500: Color32 = Color32::from_rgb(255, 110, 155);
    pub const PINK_600: Color32 = Color32::from_rgb(230, 80, 130);
    pub const PINK_700: Color32 = Color32::from_rgb(195, 55, 105);
    pub const PINK_800: Color32 = Color32::from_rgb(160, 40, 85);
    pub const PINK_900: Color32 = Color32::from_rgb(130, 30, 65);

    pub const BG_DARKEST: Color32 = Color32::from_rgb(35, 20, 28);
    pub const BG_DARK: Color32 = Color32::from_rgb(45, 28, 38);
    pub const BG_MID: Color32 = Color32::from_rgb(55, 35, 45);
    pub const BG_LIGHT: Color32 = Color32::from_rgb(70, 45, 58);
    pub const BG_LIGHTER: Color32 = Color32::from_rgb(85, 55, 70);
    
    pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(255, 235, 245);
    pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(200, 160, 180);
    pub const TEXT_MUTED: Color32 = Color32::from_rgb(150, 110, 130);
    
    pub const ACCENT_PINK: Color32 = Color32::from_rgb(255, 130, 180);
    pub const ACCENT_HOT: Color32 = Color32::from_rgb(255, 90, 150);
    pub const ACCENT_LIGHT: Color32 = Color32::from_rgb(255, 200, 220);

    pub fn apply(ctx: &egui::Context) {
        let mut visuals = Visuals::dark();
        
        visuals.window_fill = Self::BG_DARK;
        visuals.panel_fill = Self::BG_DARK;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::BG_LIGHT);
        visuals.widgets.noninteractive.bg_fill = Self::BG_MID;
        visuals.widgets.inactive.bg_fill = Self::BG_LIGHT;
        visuals.widgets.hovered.bg_fill = Self::BG_LIGHTER;
        visuals.widgets.active.bg_fill = Self::PINK_700;
        visuals.widgets.open.bg_fill = Self::PINK_600;
        
        visuals.selection.bg_fill = Self::PINK_600;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::ACCENT_LIGHT);
        
        visuals.override_text_color = Some(Self::TEXT_PRIMARY);
        
        visuals.hyperlink_color = Self::ACCENT_PINK;
        
        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::TEXT_PRIMARY);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::ACCENT_LIGHT);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::PINK_50);
        
        ctx.set_visuals(visuals);
        
        let mut style = (*ctx.style()).clone();
        style.text_styles.insert(
            TextStyle::Heading,
            FontId::new(20.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Body,
            FontId::new(14.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Monospace,
            FontId::new(13.0, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            TextStyle::Button,
            FontId::new(13.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            TextStyle::Small,
            FontId::new(11.0, egui::FontFamily::Proportional),
        );
        
        ctx.set_style(style);
    }
}
