use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn dec(x: usize) -> usize {
    if x == 0 {
        0
    } else {
        x - 1
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src/aoc11.dat");
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut board = vec![];
    for line in reader.lines() {
        let line = line?;
        board.push(line.into_bytes());
    }
    println!("Part 1: {}", run(board.clone(), 4, nearby_occupied));
    println!("Part 1: {}", run(board, 5, visible_occupied));

    Ok(())
}

fn run<F>(mut board: Vec<Vec<u8>>, limit: usize, check: F) -> usize
where
    F: Fn(&Vec<Vec<u8>>, usize, usize) -> usize,
{
    let rows = board.len();
    let cols = board[0].len();

    loop {
        let mut changes = vec![];
        let mut total_occupied = 0;
        for row in 0..rows {
            for col in 0..cols {
                match board[row][col] {
                    b'L' if check(&board, row, col) == 0 => changes.push((b'#', row, col)),
                    b'#' if check(&board, row, col) >= limit => {
                        total_occupied += 1;
                        changes.push((b'L', row, col))
                    }
                    b'#' => total_occupied += 1,

                    _ => (),
                }
            }
        }

        if changes.is_empty() {
            return total_occupied;
        }
        for change in changes {
            board[change.1][change.2] = change.0;
        }
        // for row in &board {
        //     println!("{:?}", from_utf8(&row));
        // }
        // println!("");
    }
}

fn occupied(board: &Vec<Vec<u8>>, row: usize, col: usize, dir: (isize, isize)) -> bool {
    let mut row: isize = row as isize + dir.0;
    let mut col: isize = col as isize + dir.1;
    while 0 <= row
        && 0 <= col
        && (row as usize) < board.len()
        && (col as usize) < board[row as usize].len()
    {
        let ch = board[row as usize][col as usize];
        //        println!("row: {}, col: {}, char: {}", row, col, ch);
        match ch {
            b'#' => return true,
            b'L' => return false,
            _ => (),
        }
        row = row + dir.0;
        col = col + dir.1;
    }
    return false;
}

fn visible_occupied(board: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let dirs = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    assert!(board[row][col] != b'.');

    let mut result = 0;
    for dir in dirs {
        if occupied(board, row, col, dir) {
            //            println!("occupied - row: {}, col: {}, dir:{:?}", row, col, dir);
            result += 1;
            // } else {
            //     println!("not occupied - row: {}, col: {}, dir:{:?}", row, col, dir);
        }
    }
    return result;
}

fn nearby_occupied(board: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let mut result = 0;
    for i in dec(row)..(row + 2) {
        for j in dec(col)..(col + 2) {
            if i == row && j == col {
                continue;
            }
            if i < board.len() && j < board[i].len() && board[i][j] == b'#' {
                result += 1;
            }
        }
    }
    return result;
}
