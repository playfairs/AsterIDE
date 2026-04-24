use egui::Color32;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

pub mod cherry_blossom;
pub mod rose_pine;

static CURRENT_THEME_COLORS: RwLock<ThemeColors> = RwLock::new(ThemeColors::cherry_blossom_dark());

#[derive(Debug, Clone, Copy)]
pub struct ThemeColors {
    pub bg_darkest: Color32,
    pub bg_dark: Color32,
    pub bg_mid: Color32,
    pub bg_light: Color32,
    pub bg_lighter: Color32,
    pub border: Color32,

    pub text_primary: Color32,
    pub text_secondary: Color32,
    pub text_muted: Color32,

    pub accent_primary: Color32,
    pub accent_hot: Color32,
    pub accent_light: Color32,
}

impl ThemeColors {
    pub const fn cherry_blossom_dark() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(35, 20, 28),
            bg_dark: Color32::from_rgb(45, 28, 38),
            bg_mid: Color32::from_rgb(55, 35, 45),
            bg_light: Color32::from_rgb(70, 45, 58),
            bg_lighter: Color32::from_rgb(85, 55, 70),
            border: Color32::from_rgb(85, 55, 70),
            text_primary: Color32::from_rgb(255, 235, 245),
            text_secondary: Color32::from_rgb(200, 160, 180),
            text_muted: Color32::from_rgb(150, 110, 130),
            accent_primary: Color32::from_rgb(255, 130, 180),
            accent_hot: Color32::from_rgb(255, 90, 150),
            accent_light: Color32::from_rgb(255, 200, 220),
        }
    }

    pub const fn cherry_blossom_light() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(245, 220, 230),
            bg_dark: Color32::from_rgb(255, 248, 252),
            bg_mid: Color32::from_rgb(255, 240, 248),
            bg_light: Color32::from_rgb(245, 230, 240),
            bg_lighter: Color32::from_rgb(235, 220, 230),
            border: Color32::from_rgb(235, 220, 230),
            text_primary: Color32::from_rgb(80, 40, 60),
            text_secondary: Color32::from_rgb(130, 80, 105),
            text_muted: Color32::from_rgb(170, 120, 145),
            accent_primary: Color32::from_rgb(220, 80, 140),
            accent_hot: Color32::from_rgb(255, 90, 150),
            accent_light: Color32::from_rgb(255, 180, 210),
        }
    }

    pub const fn rose_pine() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(31, 29, 46),    // Surface #1f1d2e (base color)
            bg_dark: Color32::from_rgb(38, 35, 58),       // Overlay #26233a (containers)
            bg_mid: Color32::from_rgb(33, 32, 46),        // Highlight Low #21202e (buttons)
            bg_light: Color32::from_rgb(49, 46, 73),       // Highlight Low blended
            bg_lighter: Color32::from_rgb(64, 61, 82),   // Highlight Med #403d52
            border: Color32::from_rgb(82, 79, 103),       // Highlight High #524f67
            text_primary: Color32::from_rgb(224, 222, 244),   // Text #e0def4
            text_secondary: Color32::from_rgb(144, 140, 170), // Subtle #908caa
            text_muted: Color32::from_rgb(110, 106, 134),     // Muted #6e6a86
            accent_primary: Color32::from_rgb(196, 167, 231),  // Iris #c4a7e7 (special text, buttons)
            accent_hot: Color32::from_rgb(196, 167, 231),      // Iris #c4a7e7
            accent_light: Color32::from_rgb(156, 207, 216),    // Foam #9ccfd8
        }
    }

    pub const fn rose_pine_moon() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(35, 33, 54),    // Base #232136
            bg_dark: Color32::from_rgb(42, 39, 63),      // Surface #2a273f
            bg_mid: Color32::from_rgb(57, 53, 82),       // Overlay #393552
            bg_light: Color32::from_rgb(42, 40, 62),      // Highlight Low #2a283e
            bg_lighter: Color32::from_rgb(68, 65, 90),    // Highlight Med #44415a
            border: Color32::from_rgb(86, 82, 110),       // Highlight High #56526e
            text_primary: Color32::from_rgb(224, 222, 244),   // Text #e0def4
            text_secondary: Color32::from_rgb(144, 140, 170), // Subtle #908caa
            text_muted: Color32::from_rgb(110, 106, 134),     // Muted #6e6a86
            accent_primary: Color32::from_rgb(196, 167, 231),  // Iris #c4a7e7
            accent_hot: Color32::from_rgb(196, 167, 231),      // Iris #c4a7e7
            accent_light: Color32::from_rgb(156, 207, 216),    // Foam #9ccfd8
        }
    }

    pub const fn rose_pine_dawn() -> Self {
        Self {
            bg_darkest: Color32::from_rgb(250, 244, 237),   // Base #faf4ed
            bg_dark: Color32::from_rgb(255, 250, 243),      // Surface #fffaf3
            bg_mid: Color32::from_rgb(242, 233, 222),       // Overlay #f2e9e1
            bg_light: Color32::from_rgb(223, 218, 211),     // Highlight Low #dfdad9
            bg_lighter: Color32::from_rgb(206, 202, 195),   // Highlight Med #cecacd
            border: Color32::from_rgb(189, 185, 177),       // Highlight High #bdb5b0
            text_primary: Color32::from_rgb(87, 82, 91),      // Text #575279
            text_secondary: Color32::from_rgb(121, 112, 122), // Subtle #797593
            text_muted: Color32::from_rgb(152, 147, 165),     // Muted #9893a5
            accent_primary: Color32::from_rgb(196, 167, 231),  // Iris #c4a7e7
            accent_hot: Color32::from_rgb(196, 167, 231),      // Iris #c4a7e7
            accent_light: Color32::from_rgb(156, 207, 216),    // Foam #9ccfd8
        }
    }

    pub fn for_variant(variant: ThemeVariant) -> Self {
        match variant {
            ThemeVariant::CherryBlossomDark => Self::cherry_blossom_dark(),
            ThemeVariant::CherryBlossomLight => Self::cherry_blossom_light(),
            ThemeVariant::RosePine => Self::rose_pine(),
            ThemeVariant::RosePineMoon => Self::rose_pine_moon(),
            ThemeVariant::RosePineDawn => Self::rose_pine_dawn(),
        }
    }
}

pub fn current_theme_colors() -> ThemeColors {
    CURRENT_THEME_COLORS.read().map(|c| *c).unwrap_or_else(|_| ThemeColors::cherry_blossom_dark())
}

pub fn set_current_theme_colors(colors: ThemeColors) {
    if let Ok(mut guard) = CURRENT_THEME_COLORS.write() {
        *guard = colors;
    }
}

pub fn update_current_theme(variant: ThemeVariant) {
    set_current_theme_colors(ThemeColors::for_variant(variant));
}

pub use cherry_blossom::{CherryBlossomDark, CherryBlossomLight};
pub use rose_pine::{RosePine, RosePineMoon, RosePineDawn};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeVariant {
    CherryBlossomDark,
    CherryBlossomLight,
    RosePine,
    RosePineMoon,
    RosePineDawn,
}

impl ThemeVariant {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeVariant::CherryBlossomDark => "Dark",
            ThemeVariant::CherryBlossomLight => "Light",
            ThemeVariant::RosePine => "Rose Pine",
            ThemeVariant::RosePineMoon => "Rose Pine Moon",
            ThemeVariant::RosePineDawn => "Rose Pine Dawn",
        }
    }

    pub fn apply(&self, ctx: &egui::Context, corner_roundness: f32) {
        match self {
            ThemeVariant::CherryBlossomDark => CherryBlossomDark::apply(ctx, corner_roundness),
            ThemeVariant::CherryBlossomLight => CherryBlossomLight::apply(ctx, corner_roundness),
            ThemeVariant::RosePine => RosePine::apply(ctx, corner_roundness),
            ThemeVariant::RosePineMoon => RosePineMoon::apply(ctx, corner_roundness),
            ThemeVariant::RosePineDawn => RosePineDawn::apply(ctx, corner_roundness),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeFamily {
    CherryBlossom,
    RosePine,
}

impl ThemeFamily {
    pub fn name(&self) -> &'static str {
        match self {
            ThemeFamily::CherryBlossom => "Cherry Blossom",
            ThemeFamily::RosePine => "Rose Pine",
        }
    }

    pub fn variants(&self) -> &'static [ThemeVariant] {
        match self {
            ThemeFamily::CherryBlossom => &[
                ThemeVariant::CherryBlossomDark,
                ThemeVariant::CherryBlossomLight,
            ],
            ThemeFamily::RosePine => &[
                ThemeVariant::RosePine,
                ThemeVariant::RosePineMoon,
                ThemeVariant::RosePineDawn,
            ],
        }
    }

    pub fn default_variant(&self) -> ThemeVariant {
        match self {
            ThemeFamily::CherryBlossom => ThemeVariant::CherryBlossomDark,
            ThemeFamily::RosePine => ThemeVariant::RosePine,
        }
    }
}

pub struct ThemeManager {
    pub current_family: ThemeFamily,
    pub current_variant: ThemeVariant,
    pub corner_roundness: f32,
    pub show_family_dropdown: bool,
    pub show_variant_dropdown: bool,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self {
            current_family: ThemeFamily::CherryBlossom,
            current_variant: ThemeVariant::CherryBlossomDark,
            corner_roundness: 8.0,
            show_family_dropdown: false,
            show_variant_dropdown: false,
        }
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&self, ctx: &egui::Context) {
        self.current_variant.apply(ctx, self.corner_roundness);
    }

    pub fn set_family(&mut self, family: ThemeFamily) {
        self.current_family = family;
        self.current_variant = family.default_variant();
    }

    pub fn set_variant(&mut self, variant: ThemeVariant) {
        self.current_variant = variant;
        self.current_family = match variant {
            ThemeVariant::CherryBlossomDark | ThemeVariant::CherryBlossomLight => {
                ThemeFamily::CherryBlossom
            }
            ThemeVariant::RosePine | ThemeVariant::RosePineMoon | ThemeVariant::RosePineDawn => {
                ThemeFamily::RosePine
            }
        };
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.label("Theme:");

        egui::ComboBox::from_id_salt("theme_family")
            .selected_text(self.current_family.name())
            .show_ui(ui, |ui| {
                for family in [ThemeFamily::CherryBlossom, ThemeFamily::RosePine] {
                    if ui
                        .selectable_label(self.current_family == family, family.name())
                        .clicked()
                    {
                        self.set_family(family);
                    }
                }
            });

        ui.add_space(8.0);

        egui::ComboBox::from_id_salt("theme_variant")
            .selected_text(self.current_variant.name())
            .show_ui(ui, |ui| {
                for &variant in self.current_family.variants() {
                    if ui
                        .selectable_label(self.current_variant == variant, variant.name())
                        .clicked()
                    {
                        self.current_variant = variant;
                    }
                }
            });

        ui.add_space(8.0);

        ui.label("Corner Roundness:");
        ui.add(egui::Slider::new(&mut self.corner_roundness, 0.0..=20.0));

        if ui.button("Apply Theme").clicked() {
            self.apply(ui.ctx());
        }
    }

    pub fn all_families() -> &'static [ThemeFamily] {
        &[ThemeFamily::CherryBlossom, ThemeFamily::RosePine]
    }
}

pub struct CherryBlossomTheme;

impl CherryBlossomTheme {
    pub fn BG_DARKEST() -> Color32 { current_theme_colors().bg_darkest }
    pub fn BG_DARK() -> Color32 { current_theme_colors().bg_dark }
    pub fn BG_MID() -> Color32 { current_theme_colors().bg_mid }
    pub fn BG_LIGHT() -> Color32 { current_theme_colors().bg_light }
    pub fn BG_LIGHTER() -> Color32 { current_theme_colors().bg_lighter }
    pub fn BORDER_PINK() -> Color32 { current_theme_colors().border }
    pub fn TEXT_PRIMARY() -> Color32 { current_theme_colors().text_primary }
    pub fn TEXT_SECONDARY() -> Color32 { current_theme_colors().text_secondary }
    pub fn TEXT_MUTED() -> Color32 { current_theme_colors().text_muted }
    pub fn ACCENT_PINK() -> Color32 { current_theme_colors().accent_primary }
    pub fn ACCENT_HOT() -> Color32 { current_theme_colors().accent_hot }
    pub fn ACCENT_LIGHT() -> Color32 { current_theme_colors().accent_light }

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let colors = current_theme_colors();

        let is_dark = colors.bg_darkest.r() < 128;
        let mut visuals = if is_dark {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = colors.bg_dark;
        visuals.panel_fill = colors.bg_dark;
        visuals.window_stroke = egui::Stroke::new(1.0, colors.bg_light);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = colors.bg_mid;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = colors.bg_light;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = colors.bg_lighter;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = colors.accent_hot;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = colors.accent_primary;

        visuals.selection.bg_fill = colors.accent_primary;
        visuals.selection.stroke = egui::Stroke::new(1.0, colors.accent_light);

        visuals.override_text_color = Some(colors.text_primary);
        visuals.hyperlink_color = colors.accent_primary;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, colors.text_primary);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, colors.accent_light);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, colors.bg_darkest);

        ctx.set_visuals(visuals);

        let mut style = (*ctx.global_style()).clone();
        style.text_styles.insert(
            egui::TextStyle::Heading,
            egui::FontId::new(20.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Body,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Monospace,
            egui::FontId::new(13.0, egui::FontFamily::Monospace),
        );
        style.text_styles.insert(
            egui::TextStyle::Button,
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
        );
        style.text_styles.insert(
            egui::TextStyle::Small,
            egui::FontId::new(11.0, egui::FontFamily::Proportional),
        );
        ctx.set_global_style(style);
    }
}

pub fn apply_theme_from_settings(ctx: &egui::Context, variant: ThemeVariant, corner_roundness: f32) {
    update_current_theme(variant);
    CherryBlossomTheme::apply(ctx, corner_roundness);
}
