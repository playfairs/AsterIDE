use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Copy, Serialize, Deserialize)]
pub enum PanelPosition {
    Bottom,
    Left,
    Right,
}

impl Default for PanelPosition {
    fn default() -> Self {
        PanelPosition::Bottom
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WorkbenchSettings {
    pub panel_size: f32,
    pub panel_position: PanelPosition,
    pub auto_hide_panel: bool,
    pub show_open_editors: bool,
    pub show_explorer: bool,
    pub show_search: bool,
    pub show_git: bool,
    pub show_extensions: bool,
    pub compact_mode: bool,
}

impl Default for WorkbenchSettings {
    fn default() -> Self {
        Self {
            panel_size: 200.0,
            panel_position: PanelPosition::default(),
            auto_hide_panel: false,
            show_open_editors: true,
            show_explorer: true,
            show_search: true,
            show_git: true,
            show_extensions: true,
            compact_mode: false,
        }
    }
}

use crate::Settings;
use crate::ui::{setting_card, cozy_row_filtered, matches_search};

pub fn show_workbench_settings(settings: &mut Settings, ui: &mut egui::Ui, has_search: bool, query: &str) {
    let query = query.to_lowercase();

    if !has_search || matches_search(settings, &query, &["recent", "files", "projects", "history"]) {
        setting_card(settings, ui, "Recent Items", |ui, settings| {
            cozy_row_filtered(
                settings,
                ui,
                has_search,
                &query,
                "Recent files limit",
                "Number of recent files to show",
                |ui, settings| {
                    ui.add(
                        egui::Slider::new(&mut settings.recent_files_limit, 1..=20)
                            .show_value(true)
                            .text("files"),
                    );
                },
            );
            cozy_row_filtered(
                settings,
                ui,
                has_search,
                &query,
                "Recent projects limit",
                "Number of recent projects to show",
                |ui, settings| {
                    ui.add(
                        egui::Slider::new(&mut settings.recent_projects_limit, 1..=20)
                            .show_value(true)
                            .text("projects"),
                    );
                },
            );
        });
    }
}
