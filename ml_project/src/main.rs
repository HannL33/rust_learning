use ml_project::tic_tac_engine as tte;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([420.0, 520.0])
            .with_min_inner_size([420.0, 520.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Tic-Tac-Toe",
        options,
        Box::new(|_| Ok(Box::<MyApp>::default())),
    )
}

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    ManVsMan,
    ManVsMachine,
    MachineVsMachine,
}

struct MyApp {
    game: tte::Game,
    mode: Mode,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            game: tte::Game::new(),
            mode: Mode::ManVsMan,
        }
    }
}
impl MyApp {
    /// Draw the current board and returns the field that was clicked
    fn board_ui(&self, ui: &mut eframe::egui::Ui) -> Option<usize> {
        let mut clicked = None;
        let spacing = 8.0;
        let cell = ((ui.available_width() - 2.0 * spacing) / 3.0).clamp(48.0, 140.0);
        let enabled = self.game.result() == tte::GameResult::InProgress;

        ui.vertical_centered(|ui| {
            eframe::egui::Grid::new("board")
                .spacing([spacing, spacing])
                .show(ui, |ui| {
                    for (idx, elem) in self.game.board().cells().enumerate() {
                        let (label, fill) = match elem {
                            tte::Cell::Empty => (" ", eframe::egui::Color32::from_gray(28)),
                            tte::Cell::Taken(tte::Player::O) => {
                                ("O", eframe::egui::Color32::from_gray(60))
                            }
                            tte::Cell::Taken(tte::Player::X) => {
                                ("X", eframe::egui::Color32::from_gray(60))
                            }
                        };
                        let stroke = match elem {
                            tte::Cell::Empty => {
                                eframe::egui::Stroke::new(2.0, eframe::egui::Color32::from_gray(70))
                            }
                            tte::Cell::Taken(_) => {
                                eframe::egui::Stroke::new(5.0, eframe::egui::Color32::RED)
                            }
                        };
                        let btn = eframe::egui::Button::new(
                            eframe::egui::RichText::new(label).size(34.0).strong(),
                        )
                        .min_size(eframe::egui::vec2(cell, cell))
                        .fill(fill)
                        .stroke(stroke);

                        if ui.add_enabled(enabled, btn).clicked() {
                            clicked = Some(idx);
                        }
                        if (idx + 1) % 3 == 0 {
                            ui.end_row();
                        }
                    }
                });

            ui.add_space(12.0);
            ui.with_layout(
                eframe::egui::Layout::top_down(eframe::egui::Align::Center),
                |ui| {
                    if self.game.result() == tte::GameResult::InProgress {
                        ui.label(format!("The turn is for {:?}", self.game.turn()));
                    } else {
                        ui.label(format!("The result is: {:?}", self.game.result()));
                    }
                },
            );
        });

        clicked
    }
}
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Tic-Tac-Toe");
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.mode == Mode::ManVsMan, "ManVsMan")
                    .clicked()
                {
                    self.mode = Mode::ManVsMan;
                    self.game = tte::Game::new();
                }
                if ui
                    .selectable_label(self.mode == Mode::ManVsMachine, "ManVsMachine")
                    .clicked()
                {
                    self.mode = Mode::ManVsMachine;
                    self.game = tte::Game::new();
                };
                if ui
                    .selectable_label(self.mode == Mode::MachineVsMachine, "MachineVsMachine")
                    .clicked()
                {
                    self.mode = Mode::MachineVsMachine;
                    self.game = tte::Game::new();
                };
            });
            match self.mode {
                Mode::ManVsMan => {
                    if let Some(clicked_idx) = self.board_ui(ui) {
                        match self.game.play(clicked_idx) {
                            Ok(_) => {}
                            Err(error) => println!("Error: {:?}", error),
                        }
                    }
                }
                Mode::ManVsMachine => {
                    ui.label("ManVsMachine not implemented yet".to_string());
                }
                Mode::MachineVsMachine => {
                    ui.label("MachineVsMachine not implemented yet".to_string());
                }
            };

            ui.add_space(12.0);
            ui.separator();
            ui.with_layout(
                eframe::egui::Layout::top_down(eframe::egui::Align::Center),
                |ui| {
                    if ui.button("New Game").clicked() {
                        self.game = tte::Game::new();
                    }
                },
            );
        });
    }
}
