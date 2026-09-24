use eframe::egui;
use ml_project::tic_tac_engine as tte;

fn main() -> eframe::Result {
    let mut my_board = tte::Board::new();
    let mut text_to_show_up: Option<String> = None;
    let options = eframe::NativeOptions::default();

    eframe::run_ui_native("My egui App", options, move |ui, _frame| {
        egui::CentralPanel::default().show(ui, |ui| {
            let mut clicked_idx: Option<usize> = None;
            ui.heading("Tic-Tac-Toe");
            egui::Grid::new("111").show(ui, |ui| {
                // print current board as buttons
                for (idx, elem) in my_board.cells().enumerate() {
                    let label = match elem {
                        tte::Cell::Empty => " ",
                        tte::Cell::Taken(tte::Player::O) => "O",
                        tte::Cell::Taken(tte::Player::X) => "X",
                    };

                    if ui.add(egui::Button::new(label)).clicked() {
                        clicked_idx = Some(idx + 1);
                    }
                    if (idx + 1) % 3 == 0 {
                        ui.end_row();
                    }
                }
            });
            if let Some(idx) = clicked_idx {
                match tte::apply_move(my_board, idx, tte::Player::X) {
                    Ok(new_board) => {
                        my_board = new_board;
                        text_to_show_up = None;
                    }
                    Err(tte::MoveError::CellError(msg)) => text_to_show_up = Some(msg),
                }
            }
            ui.label(text_to_show_up.as_deref().unwrap_or(""));
        });
    })
}
