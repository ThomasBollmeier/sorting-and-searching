use std::time::Instant;

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

fn dump_board(board: &[[i32; NUM_COLS]; NUM_ROWS]) {
    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
            print!("{:02} ", board[row][col]);
        }
        println!();
    }
}

// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(
    board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &mut [[i32; 2]; 8], // 8 possible moves, 2 coordinates each.
    cur_row: i32,
    cur_col: i32,
    num_visited: i32,
) -> bool {
    if num_visited == INUM_ROWS * INUM_COLS {
        return if REQUIRE_CLOSED_TOUR {
            can_tour_be_closed(cur_row, cur_col, offsets)
        } else {
            true
        };
    }

    let next_positions = get_next_positions(cur_row, cur_col, board, offsets);

    if next_positions.is_empty() {
        return false;
    }

    for [next_row, next_col] in next_positions {
        board[next_row as usize][next_col as usize] = num_visited;
        let found = find_tour(board, offsets, next_row, next_col, num_visited + 1);
        if found {
            return true;
        }
        board[next_row as usize][next_col as usize] = UNVISITED;
    }

    false
}

fn get_next_positions(
    cur_row: i32,
    cur_col: i32,
    board: &[[i32; NUM_COLS]; NUM_ROWS],
    offsets: &[[i32; 2]; 8],
) -> Vec<[i32; 2]> {
    let mut result = vec![];

    for [delta_row, delta_col] in offsets {
        let row = cur_row + delta_row;
        if row < 0 || row >= INUM_ROWS {
            continue;
        }
        let col = cur_col + delta_col;
        if col < 0 || col >= INUM_COLS {
            continue;
        }
        if board[row as usize][col as usize] != UNVISITED {
            continue;
        }
        result.push([row, col]);
    }

    result
}

fn can_tour_be_closed(cur_row: i32, cur_col: i32, offsets: &[[i32; 2]; 8]) -> bool {
    for [delta_row, delta_col] in offsets {
        let row = cur_row + delta_row;
        let col = cur_col + delta_col;
        if row == 0 && col == 0 {
            return true;
        }
    }

    false
}

fn main() {
    // Initialize the vector of move offsets.
    let mut offsets = [
        [-2, -1],
        [-1, -2],
        [2, -1],
        [1, -2],
        [-2, 1],
        [-1, 2],
        [2, 1],
        [1, 2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &mut offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&mut board);
}
