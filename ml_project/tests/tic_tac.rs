use ml_project::{
    tic_tac_bot::{best_move, best_move_alpha_beta},
    tic_tac_engine as tte,
};

type MoveFinder = fn(tte::Board, tte::Player) -> Option<usize>;
const MOVE_FINDERS: [MoveFinder; 2] = [best_move, best_move_alpha_beta];

#[test]
fn two_optimal_bots_finish_in_a_draw() {
    for find_x in MOVE_FINDERS {
        for find_o in MOVE_FINDERS {
            let mut game = tte::Game::new();
            for _ in 0..9 {
                if game.result() != tte::GameResult::InProgress {
                    break;
                }
                let find_move = if game.turn() == tte::Player::X {
                    find_x
                } else {
                    find_o
                };
                let position = find_move(*game.board(), game.turn())
                    .expect("an unfinished game must have a legal move");
                game.play(position)
                    .expect("the bot must choose a legal move");
            }
            assert_eq!(game.result(), tte::GameResult::Draw);
        }
    }
}
