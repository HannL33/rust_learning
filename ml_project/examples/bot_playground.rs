use ml_project::tic_tac_bot as ttb;
use ml_project::tic_tac_engine as tte;

fn main() -> Result<(), tte::MoveError> {
    // Indices: 0 1 2 / 3 4 5 / 6 7 8.
    // Each case starts on the bot's turn. Scores are from that bot's perspective.
    let cases = [
        ("Empty board", ".........", tte::Player::X, 0),
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
        let mut plain_count = 0;
        let mut pruned_count = 0;
        let plain = ttb::minimax(board, true, bot, &mut plain_count);
        let pruned =
            ttb::minimax_alpha_beta(board, true, bot, i32::MIN, i32::MAX, &mut pruned_count);
        assert_eq!(plain, expected, "minimax: {name}");
        assert_eq!(pruned, expected, "alpha-beta: {name}");
        assert!(pruned_count <= plain_count, "{name}");
        results.push((
            name,
            bot,
            expected,
            plain,
            pruned,
            plain_count,
            pruned_count,
        ));
    }

    println!("\n--- Minimax comparison (bot's perspective) ---");
    for (name, bot, expected, plain, pruned, plain_count, pruned_count) in results {
        println!(
            "{name} | bot: {bot:?} | expected: {expected} | minimax: {plain} ({plain_count} nodes) | alpha-beta: {pruned} ({pruned_count} nodes)"
        );
    }
    Ok(())
}
