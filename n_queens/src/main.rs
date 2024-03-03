use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 20;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

fn dump_board(board: &[[char; NUM_COLS]; NUM_ROWS]) {
    for row in board {
        for cell in row {
            print!("{cell} ");
        }
        println!();
    }
}

// Return true if this series of squares contains at most one queen.
fn series_is_legal(
    board: &[[char; NUM_COLS]; NUM_ROWS],
    r0: i32,
    c0: i32,
    dr: i32,
    dc: i32,
) -> bool {
    let mut cnt_queens = 0;
    let mut r = r0;
    let mut c = c0;

    while r >= 0 && r < INUM_ROWS && c >= 0 && c < INUM_COLS {
        if board[r as usize][c as usize] == 'Q' {
            cnt_queens += 1;
        }
        if cnt_queens > 1 {
            return false;
        }
        r += dr;
        c += dc;
    }

    true
}

// Return true if the board is legal.
fn board_is_legal(board: &[[char; NUM_COLS]; NUM_ROWS]) -> bool {
    let num_rows = board.len() as i32;
    let num_cols = board[0].len() as i32;

    for r in 0..num_rows {
        if !series_is_legal(board, r, 0, 0, 1) {
            return false;
        }
    }
    for c in 0..num_cols {
        if !series_is_legal(board, 0, c, 1, 0) {
            return false;
        }
    }
    for d in 0..num_rows - 1 {
        if !series_is_legal(board, d, 0, 1, 1) {
            return false;
        }
    }
    for d in 1..num_cols - 1 {
        if !series_is_legal(board, 0, d, 1, 1) {
            return false;
        }
    }
    for d in 0..num_rows - 1 {
        if !series_is_legal(board, d, num_cols - 1, 1, -1) {
            return false;
        }
    }
    for d in 1..num_cols - 1 {
        if !series_is_legal(board, 0, d, 1, -1) {
            return false;
        }
    }
    true
}

// Return true if the board is legal and a solution.
fn _board_is_a_solution(board: &[[char; NUM_COLS]; NUM_ROWS]) -> bool {
    if !board_is_legal(board) {
        return false;
    }

    let mut num_queens = 0;
    for row in board {
        for col in row {
            if col == &'Q' {
                num_queens += 1;
            }
        }
    }

    num_queens == board.len()
}

fn _next_location(r: i32, c: i32) -> (i32, i32) {
    let mut next_r = r;
    let mut next_c = c + 1;

    if next_c >= INUM_COLS {
        next_c = 0;
        next_r += 1;
    }

    (next_r, next_c)
}

fn _place_queens_1(board: &mut [[char; NUM_COLS]; NUM_ROWS], r: i32, c: i32) -> bool {
    if r > INUM_ROWS - 1 {
        return _board_is_a_solution(board);
    }

    let (next_r, next_c) = _next_location(r, c);

    // no placement:
    let mut ok = _place_queens_1(board, next_r, next_c);
    if ok {
        return true;
    }

    // placement:
    board[r as usize][c as usize] = 'Q';
    ok = _place_queens_1(board, next_r, next_c);
    if !ok {
        board[r as usize][c as usize] = '.';
    }

    ok
}

fn _place_queens_2(
    board: &mut [[char; NUM_COLS]; NUM_ROWS],
    r: i32,
    c: i32,
    num_placed: i32,
) -> bool {
    if r > INUM_ROWS - 1 || num_placed == INUM_ROWS {
        return _board_is_a_solution(board);
    }

    let (next_r, next_c) = _next_location(r, c);

    // no placement:
    let mut ok = _place_queens_2(board, next_r, next_c, num_placed);
    if ok {
        return true;
    }

    // placement:
    board[r as usize][c as usize] = 'Q';
    ok = _place_queens_2(board, next_r, next_c, num_placed + 1);
    if !ok {
        board[r as usize][c as usize] = '.';
    }

    ok
}

// Try to place a queen in this column.
// Return true if we find a legal board.
fn place_queens_4(board: &mut [[char; NUM_COLS]; NUM_ROWS], c: i32) -> bool {
    if c == INUM_COLS {
        return board_is_legal(board);
    }

    if !board_is_legal(board) {
        return false;
    }

    for r in 0..INUM_ROWS {
        board[r as usize][c as usize] = 'Q';
        if place_queens_4(board, c + 1) {
            return true;
        }
        board[r as usize][c as usize] = '.';
    }

    false
}

fn main() {
    // Create a NUM_ROWS x NUM_COLS array with all entries Initialized to UNVISITED.
    let mut board = [['.'; NUM_COLS]; NUM_ROWS];

    let start = Instant::now();
    //let success = _place_queens_1(&mut board, 0, 0);
    //let success = place_queens_2(&mut board, 0, 0, 0);
    //let success = place_queens_3(& mut board);
    let success = place_queens_4(&mut board, 0);
    let duration = start.elapsed();

    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}
