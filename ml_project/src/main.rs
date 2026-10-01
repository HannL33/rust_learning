use ml_project::tic_tac_engine as tte;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default().with_inner_size([420.0, 520.0]),
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

        ui.heading("Tic-Tac-Toe");
        eframe::egui::Grid::new("board")
            .spacing([8.0, 8.0])
            .show(ui, |ui| {
                // print current board as buttons
                for (idx, elem) in self.game.board().cells().enumerate() {
                    let label = match elem {
                        tte::Cell::Empty => " ",
                        tte::Cell::Taken(tte::Player::O) => "O",
                        tte::Cell::Taken(tte::Player::X) => "X",
                    };
                    let btn = eframe::egui::Button::new(
                        eframe::egui::RichText::new(label).size(34.0).strong(),
                    )
                    .min_size(eframe::egui::vec2(96.0, 96.0))
                    .fill(eframe::egui::Color32::DARK_GRAY)
                    .stroke(eframe::egui::Stroke::new(5.0, eframe::egui::Color32::RED));

                    if ui
                        .add_enabled(self.game.result() == tte::GameResult::InProgress, btn)
                        .clicked()
                    {
                        clicked = Some(idx);
                    }
                    if (idx + 1) % 3 == 0 {
                        ui.end_row();
                    }
                }
            });
        ui.vertical(|ui| {
            if self.game.result() == tte::GameResult::InProgress {
                ui.label(format!("The turn is for {:?}", self.game.turn()))
            } else {
                ui.label(format!("The result is: {:?}", self.game.result()))
            }
        });
        return clicked;
    }
}
impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ui, |ui| {
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

            ui.centered_and_justified(|ui| {
                if ui.button("New Game").clicked() {
                    self.game = tte::Game::new();
                }
            });
        });
    }
}
