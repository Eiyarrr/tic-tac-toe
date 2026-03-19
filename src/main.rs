// 0, 1, 2 -> _, X, O
fn main() {
    let mut board: [char; 9] = [
        '_', '_', '_',
        '_', '_', '_',
        '_', '_', '_',
    ];

    let mut turn: i8 = 0;   // Evens -> X, Odds -> O
    loop {
        board = make_move(&turn, board);
        check_victory(&board);
        turn += 1;
    }
}

// 0 -> No victory
// 1 -> X wins
// 2 -> O wins
// 5 -> Draw
fn check_victory(board: &[char; 9]) -> i8 {
    0
}

fn make_move(turn: &i8, board: [char; 9]) -> [char; 9] {
    board
}
