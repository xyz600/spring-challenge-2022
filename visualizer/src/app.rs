use crate::app::egui::Pos2;
use crate::app::egui::Stroke;
use eframe::{egui, epaint::Color32, epi};
use simulator::Simulator;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    sim: simulator::Simulator,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            sim: simulator::Simulator::new(0),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "eframe template"
    }

    /// Called once before the first frame.
    fn setup(&mut self, _ctx: &egui::Context, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    /// Called by the frame work to save state before shutdown.
    /// Note that you must enable the `persistence` feature for this to work.
    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let Self { sim } = self;

        let scale = (simulator::MAX_X as f32) / 1080.0;
        let offset = simulator::MAP_LIMIT as f32 / scale;

        let convert = |v| v / scale + offset;

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            // self hero
            for (player, is_self) in sim.components.player_list.iter().zip([true, false]) {
                let color = if is_self { Color32::RED } else { Color32::BLUE };
                eprintln!("{:?}", color);
                for hero in player.hero_list.iter() {
                    eprintln!("{:?}", hero.component.position);
                    // 本体
                    painter.circle(
                        Pos2 {
                            x: convert(hero.component.position.x as f32),
                            y: convert(hero.component.position.y as f32),
                        },
                        10.0,
                        Color32::WHITE,
                        Stroke { width: 5.0, color },
                    )
                }
            }
        });

        egui::TopBottomPanel::bottom("config").show(ctx, |ui| {
            if ui.button("next turn").clicked() {
                let player_action = vec![simulator::Action::Wait; 3];
                let opponent_action = vec![simulator::Action::Wait; 3];

                sim.next_state(player_action, opponent_action);
            }
        });
    }
}
