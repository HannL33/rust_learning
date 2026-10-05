use ml_project::tic_tac_bot as ttb;
use ml_project::tic_tac_engine as tte;

fn main() -> Result<(), tte::MoveError> {
    // let my_board = tte::Board::new();
    // let my_board = tte::apply_move(my_board, 4, tte::Player::X)?;
    // let my_board = tte::apply_move(my_board, 1, tte::Player::O)?;
    // let my_board = tte::apply_move(my_board, 0, tte::Player::X)?;
    // let my_board = tte::apply_move(my_board, 8, tte::Player::O)?;
    // println!("My starting board: {:?}", my_board);
    // println!(
    //     "The evaluation of the position: {}",
    //     ttb::minimax(my_board, true, tte::Player::X)
    // );

    // Indices: 0 1 2 / 3 4 5 / 6 7 8.
    // Each case starts on the bot's turn. Scores are from that bot's perspective.
    let cases = [
        (
            "X can win: XX. / OO. / XO.",
            "XX.OO.XO.",
            tte::Player::X,
            10,
        ),
        (
            "O can win: OO. / XXO / X.X",
            "OO.XXOX.X",
            tte::Player::O,
            10,
        ),
        (
            "One move left, draw: XOX / XOO / OX.",
            "XOXXOOOX.",
            tte::Player::X,
            0,
        ),
        (
            "X cannot avoid losing: OO. / OXX / .X.",
            "OO.OXX.X.",
            tte::Player::X,
            -10,
        ),
    ];

    let mut results = Vec::new();
    for (name, cells, bot, expected) in cases {
        let mut board = tte::Board::new();
        for (position, cell) in cells.chars().enumerate() {
            let player = match cell {
                'X' => tte::Player::X,
                'O' => tte::Player::O,
                '.' => continue,
                _ => unreachable!(),
            };
            board = tte::apply_move(board, position, player)?;
        }
        assert_eq!(tte::whos_turn(&board), bot);
        let actual = ttb::minimax_alpha_beta(
            board,
            true,
            bot,
            &mut i32::MIN.clone(),
            &mut i32::MAX.clone(),
        );
        assert_eq!(actual, expected, "{name}");
        results.push((name, bot, expected, actual));
    }

    // Print together after minimax's debug output for easy comparison.
    println!("\n--- Minimax comparison (bot's perspective) ---");
    for (name, bot, expected, actual) in results {
        println!("{name} | bot: {bot:?} | expected: {expected} | actual: {actual}");
    }
    Ok(())
}
