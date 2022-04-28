use std::borrow::Cow;
use std::collections::BTreeMap;

use crate::app::egui::Pos2;
use crate::app::egui::Stroke;
use eframe::egui::FontDefinitions;
use eframe::egui::Painter;
use eframe::egui::RichText;
use eframe::egui::Ui;
use eframe::epaint::FontFamily;
use eframe::epaint::FontId;
use eframe::{egui, epaint::Color32, epi};

use simulator::IPoint;
use simulator::Simulator;
use simulator::CENTER;
use simulator::MAP_LIMIT;
use simulator::MAX_X;
use simulator::MAX_Y;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    sim: simulator::Simulator,
    solver1: solver::Solver,
    solver2: solver::Solver,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            sim: simulator::Simulator::new(0),
            solver1: solver::Solver::new(&simulator::IPoint::new(), 3),
            // simulater 側で solver2 の方は点対称に回して渡すので、IPoint(x: MAX_X, y: MAX_Y) ではない
            solver2: solver::Solver::new(&simulator::IPoint::new(), 3),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Spring Challenge 2022 Visualizer"
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
        // setup fonts

        // setup simulator
        let Self { sim, solver1, solver2 } = self;

        let scale = (simulator::MAX_X as f32) / 1080.0;
        let offset = simulator::MAP_LIMIT as f32 / scale + 50.0;

        let convert = |v| v / scale + offset;

        let draw_line = |painter: &Painter, p1: &IPoint, p2: &IPoint, color: Color32| {
            let pos1 = Pos2 {
                y: convert(p1.y as f32),
                x: convert(p1.x as f32),
            };
            let pos2 = Pos2 {
                y: convert(p2.y as f32),
                x: convert(p2.x as f32),
            };
            painter.line_segment([pos1, pos2], Stroke { width: 2.0, color });
        };

        let draw_circle = |painter: &Painter, circle: &IPoint, radius: f32, color: Color32| {
            let center = Pos2 {
                y: convert(circle.y as f32),
                x: convert(circle.x as f32),
            };
            painter.circle(center, radius, Color32::WHITE, Stroke { width: 5.0, color });
        };

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            // 外枠
            let points = [
                IPoint { x: 0, y: 0 },
                IPoint { x: MAX_X, y: 0 },
                IPoint { x: MAX_X, y: MAX_Y },
                IPoint { x: 0, y: MAX_Y },
            ];

            for i in 0..4 {
                draw_line(painter, &points[i], &points[(i + 1) % 4], Color32::BLACK);
            }

            // 真の外枠
            let points = [
                IPoint {
                    x: -MAP_LIMIT + 1,
                    y: -MAP_LIMIT + 1,
                },
                IPoint {
                    x: MAX_X + MAP_LIMIT - 1,
                    y: -MAP_LIMIT + 1,
                },
                IPoint {
                    x: MAX_X + MAP_LIMIT - 1,
                    y: MAX_Y + MAP_LIMIT - 1,
                },
                IPoint {
                    x: -MAP_LIMIT + 1,
                    y: MAX_Y + MAP_LIMIT - 1,
                },
            ];
            for i in 0..4 {
                draw_line(painter, &points[i], &points[(i + 1) % 4], Color32::GRAY);
            }

            let spawn_list = [
                IPoint {
                    x: MAX_X / 2,
                    y: -MAP_LIMIT + 1,
                },
                IPoint {
                    x: MAX_X / 2 + 4000,
                    y: -MAP_LIMIT + 1,
                },
                IPoint {
                    x: MAX_X / 2,
                    y: MAX_Y + MAP_LIMIT - 1,
                },
                IPoint {
                    x: MAX_X / 2 - 4000,
                    y: MAX_Y + MAP_LIMIT - 1,
                },
            ];
            for point in spawn_list.iter() {
                draw_circle(painter, point, 5.0, Color32::GRAY);
            }

            // hero
            for (player, is_self) in sim.components.player_list.iter().zip([true, false]) {
                let color = if is_self { Color32::RED } else { Color32::BLUE };
                for hero in player.hero_list.iter() {
                    // 本体
                    draw_circle(painter, &hero.component.position, 5.0, color);
                }
            }

            // monster
            for monster in sim.components.monster_list.iter() {
                draw_circle(painter, &monster.component.position, 5.0, Color32::BLACK);
            }
        });

        // let rich_text = |text: String| text;
        let fontsize = 14.0;
        let rich_text = |text: String| RichText::new(text).font(FontId::proportional(fontsize));

        macro_rules! label {
            ( $ui:ident, $name:expr, $e:expr ) => {
                $ui.label(rich_text(format!("{}: {:?}", $name, $e)));
            };
        }

        // turn 数と次の状態遷移
        egui::TopBottomPanel::bottom("config").show(ctx, |ui| {
            // button
            if ui.button(rich_text("next turn".to_string())).clicked() {
                let player1_board = sim.to_board(0);
                player1_board.dump();
                let player1_action = solver1.solve(&player1_board);

                let mut player2_board = sim.to_board(1);
                player2_board.point_symmetry();
                player2_board.dump();
                let mut player2_action = solver2.solve(&player2_board);
                // action 反転
                player2_action = player2_action
                    .into_iter()
                    .map(|action| -> simulator::Action {
                        match action {
                            simulator::Action::Wait { message } => simulator::Action::Wait { message },
                            simulator::Action::Move { point, message } => simulator::Action::Move {
                                point: point.point_symmetry(&CENTER),
                                message,
                            },
                            simulator::Action::Wind { point, message } => simulator::Action::Wind {
                                point: point.point_symmetry(&CENTER),
                                message,
                            },
                            simulator::Action::Shield { entity_id, message } => {
                                simulator::Action::Shield { entity_id, message }
                            }
                            simulator::Action::Control {
                                entity_id,
                                point,
                                message,
                            } => simulator::Action::Control {
                                entity_id,
                                point: point.point_symmetry(&CENTER),
                                message,
                            },
                        }
                    })
                    .collect();

                eprintln!("return action:");
                eprintln!("player 1");
                for action in player1_action.iter() {
                    eprintln!("  {:?}", action);
                }
                eprintln!("player 2");
                for action in player2_action.iter() {
                    eprintln!("  {:?}", action);
                }

                sim.next_state(player1_action, player2_action);
            }
            ui.label(rich_text(format!("turn: {}", sim.turn)));
        });

        egui::SidePanel::right("info").min_width(300.0).show(ctx, |ui| {
            for (text, player) in ["player1".to_string(), "player2".to_string()]
                .iter()
                .zip(sim.components.player_list.iter())
            {
                // hero
                egui::CollapsingHeader::new(text).default_open(true).show(ui, |ui| {
                    label!(ui, "health", player.health);
                    label!(ui, "mana", player.mana);
                    label!(ui, "base", player.base);
                    for (hero_id, hero) in player.hero_list.iter().enumerate() {
                        egui::CollapsingHeader::new(format!("hero {}", hero_id))
                            .default_open(true)
                            .show(ui, |ui| {
                                label!(ui, "Action", hero.action);
                                label!(ui, "Position", hero.component.position);
                                label!(ui, "Velocity", hero.component.velocity);
                                label!(ui, "Id", hero.component.id);
                                label!(ui, "Shield_life", hero.component.shield_life);
                                label!(ui, "IsControled", hero.component.is_controlled);
                            });
                    }
                });
            }

            // monster
            egui::CollapsingHeader::new("monster list")
                .default_open(true)
                .show(ui, |ui| {
                    for (monster_id, monster) in sim.components.monster_list.iter().enumerate() {
                        egui::CollapsingHeader::new(format!("monster {}", monster_id))
                            .default_open(true)
                            .show(ui, |ui| {
                                label!(ui, "Health", monster.health);
                                label!(ui, "Position", monster.component.position);
                                label!(ui, "Velocity", monster.component.velocity);
                                label!(ui, "Id", monster.component.id);
                                label!(ui, "Shield_life", monster.component.shield_life);
                                label!(ui, "IsControled", monster.component.is_controlled);
                            });
                    }
                });
        });

        // 情報表示
    }
}
