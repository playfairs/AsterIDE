use egui::{Color32, FontId, TextStyle, Visuals};

fn apply_fonts(ctx: &egui::Context) {
    let mut style = (*ctx.global_style()).clone();
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
    ctx.set_global_style(style);
}

pub struct RosePine;

impl RosePine {
    pub const BASE: Color32 = Color32::from_rgb(250, 244, 237);
    pub const SURFACE: Color32 = Color32::from_rgb(255, 250, 243);
    pub const OVERLAY: Color32 = Color32::from_rgb(242, 233, 222);
    pub const MUTED: Color32 = Color32::from_rgb(152, 147, 165);
    pub const SUBTLE: Color32 = Color32::from_rgb(121, 112, 122);
    pub const TEXT: Color32 = Color32::from_rgb(87, 82, 91);
    pub const LOVE: Color32 = Color32::from_rgb(180, 99, 122);
    pub const GOLD: Color32 = Color32::from_rgb(234, 157, 52);
    pub const ROSE: Color32 = Color32::from_rgb(215, 130, 126);
    pub const PINE: Color32 = Color32::from_rgb(40, 105, 131);
    pub const FOAM: Color32 = Color32::from_rgb(86, 148, 159);
    pub const IRIS: Color32 = Color32::from_rgb(144, 122, 169);
    pub const HIGHLIGHT_LOW: Color32 = Color32::from_rgb(244, 237, 232);
    pub const HIGHLIGHT_MED: Color32 = Color32::from_rgb(223, 218, 211);
    pub const HIGHLIGHT_HIGH: Color32 = Color32::from_rgb(206, 202, 195);

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let mut visuals = Visuals::light();
        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = Self::BASE;
        visuals.panel_fill = Self::BASE;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::HIGHLIGHT_MED);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = Self::SURFACE;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = Self::OVERLAY;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = Self::HIGHLIGHT_MED;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = Self::ROSE;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = Self::LOVE;

        visuals.selection.bg_fill = Self::ROSE;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::LOVE);

        visuals.override_text_color = Some(Self::TEXT);
        visuals.hyperlink_color = Self::IRIS;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::TEXT);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::PINE);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::BASE);

        ctx.set_visuals(visuals);
        apply_fonts(ctx);
    }
}

pub struct RosePineMoon;

impl RosePineMoon {
    pub const BASE: Color32 = Color32::from_rgb(35, 33, 54);
    pub const SURFACE: Color32 = Color32::from_rgb(42, 40, 62);
    pub const OVERLAY: Color32 = Color32::from_rgb(57, 53, 82);
    pub const MUTED: Color32 = Color32::from_rgb(110, 106, 134);
    pub const SUBTLE: Color32 = Color32::from_rgb(144, 140, 170);
    pub const TEXT: Color32 = Color32::from_rgb(224, 222, 244);
    pub const LOVE: Color32 = Color32::from_rgb(235, 111, 146);
    pub const GOLD: Color32 = Color32::from_rgb(246, 193, 119);
    pub const ROSE: Color32 = Color32::from_rgb(235, 188, 186);
    pub const PINE: Color32 = Color32::from_rgb(62, 143, 176);
    pub const FOAM: Color32 = Color32::from_rgb(156, 207, 216);
    pub const IRIS: Color32 = Color32::from_rgb(196, 167, 231);
    pub const HIGHLIGHT_LOW: Color32 = Color32::from_rgb(42, 40, 62);
    pub const HIGHLIGHT_MED: Color32 = Color32::from_rgb(68, 65, 90);
    pub const HIGHLIGHT_HIGH: Color32 = Color32::from_rgb(82, 79, 103);

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let mut visuals = Visuals::dark();
        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = Self::BASE;
        visuals.panel_fill = Self::BASE;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::OVERLAY);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = Self::SURFACE;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = Self::OVERLAY;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = Self::HIGHLIGHT_MED;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = Self::LOVE;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = Self::IRIS;

        visuals.selection.bg_fill = Self::LOVE;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::ROSE);

        visuals.override_text_color = Some(Self::TEXT);
        visuals.hyperlink_color = Self::FOAM;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::TEXT);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::ROSE);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::BASE);

        ctx.set_visuals(visuals);
        apply_fonts(ctx);
    }
}

pub struct RosePineDawn;

impl RosePineDawn {
    pub const BASE: Color32 = Color32::from_rgb(250, 245, 238);
    pub const SURFACE: Color32 = Color32::from_rgb(255, 251, 245);
    pub const OVERLAY: Color32 = Color32::from_rgb(242, 237, 232);
    pub const MUTED: Color32 = Color32::from_rgb(152, 147, 165);
    pub const SUBTLE: Color32 = Color32::from_rgb(121, 112, 122);
    pub const TEXT: Color32 = Color32::from_rgb(87, 82, 91);
    pub const LOVE: Color32 = Color32::from_rgb(180, 99, 122);
    pub const GOLD: Color32 = Color32::from_rgb(234, 157, 52);
    pub const ROSE: Color32 = Color32::from_rgb(215, 130, 126);
    pub const PINE: Color32 = Color32::from_rgb(40, 105, 131);
    pub const FOAM: Color32 = Color32::from_rgb(86, 148, 159);
    pub const IRIS: Color32 = Color32::from_rgb(144, 122, 169);
    pub const HIGHLIGHT_LOW: Color32 = Color32::from_rgb(244, 239, 234);
    pub const HIGHLIGHT_MED: Color32 = Color32::from_rgb(223, 218, 213);
    pub const HIGHLIGHT_HIGH: Color32 = Color32::from_rgb(206, 202, 197);

    pub fn apply(ctx: &egui::Context, corner_roundness: f32) {
        let mut visuals = Visuals::light();
        let radius = egui::CornerRadius::same(corner_roundness.clamp(0.0, 255.0) as u8);

        visuals.window_fill = Self::BASE;
        visuals.panel_fill = Self::BASE;
        visuals.window_stroke = egui::Stroke::new(1.0, Self::HIGHLIGHT_MED);
        visuals.window_corner_radius = radius;
        visuals.menu_corner_radius = radius;

        visuals.widgets.noninteractive.corner_radius = radius;
        visuals.widgets.noninteractive.bg_fill = Self::SURFACE;
        visuals.widgets.inactive.corner_radius = radius;
        visuals.widgets.inactive.bg_fill = Self::OVERLAY;
        visuals.widgets.hovered.corner_radius = radius;
        visuals.widgets.hovered.bg_fill = Self::HIGHLIGHT_MED;
        visuals.widgets.active.corner_radius = radius;
        visuals.widgets.active.bg_fill = Self::ROSE;
        visuals.widgets.open.corner_radius = radius;
        visuals.widgets.open.bg_fill = Self::LOVE;

        visuals.selection.bg_fill = Self::ROSE;
        visuals.selection.stroke = egui::Stroke::new(1.0, Self::LOVE);

        visuals.override_text_color = Some(Self::TEXT);
        visuals.hyperlink_color = Self::IRIS;

        visuals.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, Self::TEXT);
        visuals.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, Self::PINE);
        visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0, Self::BASE);

        ctx.set_visuals(visuals);
        apply_fonts(ctx);
    }
}
