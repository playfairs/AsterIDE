use serde::{Deserialize, Serialize};
use theme;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppearanceSettings {
    pub sidebar_visible: bool,
    pub status_bar_visible: bool,
    pub activity_bar_visible: bool,
    pub corner_roundness: f32,
    pub theme_family: theme::ThemeFamily,
    pub theme_variant: theme::ThemeVariant,
    pub custom_background_color: Option<String>,
    pub custom_accent_color: Option<String>,
    pub window_opacity: f32,
    pub animations_enabled: bool,
    pub font_smoothing: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            sidebar_visible: true,
            status_bar_visible: true,
            activity_bar_visible: true,
            corner_roundness: 6.0,
            theme_family: theme::ThemeFamily::CherryBlossom,
            theme_variant: theme::ThemeVariant::CherryBlossomDark,
            custom_background_color: None,
            custom_accent_color: None,
            window_opacity: 1.0,
            animations_enabled: true,
            font_smoothing: true,
        }
    }
}

use crate::Settings;
use crate::ui::{setting_card, cozy_row_filtered, matches_search};

pub fn show_appearance_settings(settings: &mut Settings, ui: &mut egui::Ui, has_search: bool, query: &str) {
    let query = query.to_lowercase();

    if !has_search || matches_search(settings, &query, &["theme", "color scheme"]) {
        setting_card(settings, ui, "Theme", |ui, settings| {
            cozy_row_filtered(
                settings,
                ui,
                has_search,
                &query,
                "Theme Family",
                "Select the theme family",
                |ui, settings| {
                    egui::ComboBox::from_id_salt("settings_theme_family")
                        .selected_text(settings.theme_family.name())
                        .width(140.0)
                        .show_ui(ui, |ui| {
                            for family in theme::ThemeManager::all_families() {
                                if ui
                                    .selectable_label(
                                        settings.theme_family == *family,
                                        family.name(),
                                    )
                                    .clicked()
                                {
                                    settings.theme_family = *family;
                                    settings.theme_variant = family.default_variant();
                                }
                            }
                        });
                },
            );
            cozy_row_filtered(
                settings,
                ui,
                has_search,
                &query,
                "Theme Variant",
                "Select the theme variant",
                |ui, settings| {
                    egui::ComboBox::from_id_salt("settings_theme_variant")
                        .selected_text(settings.theme_variant.name())
                        .width(140.0)
                        .show_ui(ui, |ui| {
                            for &variant in settings.theme_family.variants() {
                                if ui
                                    .selectable_label(
                                        settings.theme_variant == variant,
                                        variant.name(),
                                    )
                                    .clicked()
                                {
                                    settings.theme_variant = variant;
                                }
                            }
                        });
                },
            );
        });
        ui.add_space(12.0);
    }

    setting_card(settings, ui, "UI Elements", |ui, settings| {
        cozy_row_filtered(
            settings,
            ui,
            has_search,
            &query,
            "Sidebar",
            "Show the left sidebar",
            |ui, settings| {
                ui.checkbox(&mut settings.sidebar_visible, "");
            },
        );
        cozy_row_filtered(
            settings,
            ui,
            has_search,
            &query,
            "Status bar",
            "Show the bottom status bar",
            |ui, settings| {
                ui.checkbox(&mut settings.status_bar_visible, "");
            },
        );
        cozy_row_filtered(
            settings,
            ui,
            has_search,
            &query,
            "Minimap",
            "Show code minimap on the right",
            |ui, settings| {
                ui.checkbox(&mut settings.minimap, "");
            },
        );
        cozy_row_filtered(
            settings,
            ui,
            has_search,
            &query,
            "Corner roundness",
            "UI element corner radius",
            |ui, settings| {
                ui.add(
                    egui::Slider::new(&mut settings.corner_roundness, 0.0..=20.0)
                        .show_value(true)
                        .text("px"),
                );
            },
        );
    });
}
