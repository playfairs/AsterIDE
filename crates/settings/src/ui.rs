use crate::Settings;
use theme::CherryBlossomTheme;

pub fn setting_card(
    settings: &mut Settings,
    ui: &mut egui::Ui,
    title: &str,
    content: impl FnOnce(&mut egui::Ui, &mut Settings),
) {
    let card_margin = 16.0;

    egui::Frame::group(ui.style())
        .fill(CherryBlossomTheme::BG_DARK())
        .corner_radius(settings.corner_roundness)
        .stroke(egui::Stroke::new(1.0, CherryBlossomTheme::BG_LIGHT()))
        .inner_margin(egui::Margin::same(card_margin as i8))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.add(
                egui::Label::new(
                    egui::RichText::new(title)
                        .size(14.0)
                        .strong()
                        .color(CherryBlossomTheme::TEXT_PRIMARY())
                ).selectable(false)
            );

            ui.add_space(12.0);

            ui.painter().line_segment(
                [
                    ui.cursor().left_center(),
                    ui.cursor().left_center() + egui::vec2(ui.available_width(), 0.0),
                ],
                egui::Stroke::new(1.0, CherryBlossomTheme::BG_LIGHT()),
            );
            ui.add_space(12.0);

            content(ui, settings);
        });
}

pub fn cozy_row(
    _settings: &mut Settings,
    ui: &mut egui::Ui,
    title: &str,
    description: &str,
    control: impl FnOnce(&mut egui::Ui, &mut Settings),
) {
    ui.horizontal(|ui| {
        ui.set_width(ui.available_width());

        ui.vertical(|ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(title)
                        .size(13.0)
                        .color(CherryBlossomTheme::TEXT_PRIMARY())
                ).selectable(false)
            );
            ui.add(
                egui::Label::new(
                    egui::RichText::new(description)
                        .size(11.0)
                        .color(CherryBlossomTheme::TEXT_MUTED())
                ).selectable(false)
            );
        });

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            control(ui, _settings);
        });
    });

    ui.add_space(12.0);
}

pub fn cozy_row_filtered(
    settings: &mut Settings,
    ui: &mut egui::Ui,
    has_search: bool,
    query: &str,
    title: &str,
    description: &str,
    control: impl FnOnce(&mut egui::Ui, &mut Settings),
) {
    if has_search {
        let search_text = format!(
            "{} {} {}",
            title,
            description,
            get_setting_keywords(settings, title)
        )
        .to_lowercase();
        if !search_text.contains(query) {
            return;
        }
    }
    cozy_row(settings, ui, title, description, control);
}

pub fn matches_search(_settings: &Settings, query: &str, keywords: &[&str]) -> bool {
    keywords.iter().any(|kw| kw.to_lowercase().contains(query))
}

pub fn get_setting_keywords(_settings: &Settings, title: &str) -> &'static str {
    match title {
        "Show line numbers" => "gutter numbers",
        "Word wrap" => "wrap line break",
        "Font size" => "text size typography",
        "Font family" => "typeface typography",
        "Tab size" => "indent width",
        "Use spaces" => "indentation tabs",
        "Vim mode" => "modal editing",
        "Auto save" => "autosave automatic save",
        "Theme" => "color scheme appearance",
        "Sidebar" => "explorer file tree panel",
        "Status bar" => "bottom panel information",
        "Minimap" => "code overview thumbnail",
        _ => "",
    }
}
