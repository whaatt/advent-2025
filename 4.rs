use anyhow::Context;
use std::collections::HashMap;

fn main() {
    let (grid, rows, columns) = match parse_grid() {
        Ok((grid, rows, columns)) => (grid, rows, columns),
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    println!("{}", problem(&mut grid.clone(), rows, columns, false));
    println!("{}", problem(&mut grid.clone(), rows, columns, true));
}

fn problem(
    grid: &mut HashMap<(i32, i32), char>,
    rows: i32,
    columns: i32,
    continue_until_stable: bool,
) -> i64 {
    let mut accessible_paper = 0;
    loop {
        let mut got_changes = false;
        for row in 0..rows {
            for column in 0..columns {
                if get_value(grid, row, column) != '@' {
                    continue;
                }

                let count_neighboring_paper = get_neighbors(row, column)
                    .iter()
                    .filter(|(neighbor_row, neighbor_column)| {
                        get_value(grid, *neighbor_row, *neighbor_column) == '@'
                    })
                    .count();
                if count_neighboring_paper < 4 {
                    // Make modifications to grid if `continue_until_stable`.
                    if continue_until_stable {
                        grid.insert((row, column), '.');
                        got_changes = true;
                    }
                    accessible_paper += 1;
                }
            }
        }

        // Break if `continue_until_stable` is not set or no changes were made.
        if !continue_until_stable || !got_changes {
            break;
        }
    }

    accessible_paper
}

fn get_value(grid: &HashMap<(i32, i32), char>, row: i32, column: i32) -> char {
    *grid.get(&(row, column)).unwrap_or(&'.')
}

fn get_neighbors(row: i32, column: i32) -> [(i32, i32); 8] {
    [
        (row - 1, column - 1),
        (row - 1, column),
        (row - 1, column + 1),
        (row, column - 1),
        (row, column + 1),
        (row + 1, column - 1),
        (row + 1, column),
        (row + 1, column + 1),
    ]
}

#[allow(clippy::type_complexity)]
fn parse_grid() -> anyhow::Result<(HashMap<(i32, i32), char>, i32, i32)> {
    let lines = shared::read_lines("./4.txt").context("failed to read lines")?;
    let mut grid = HashMap::new();
    let mut rows = 0;
    let mut columns = 0;
    for (row, line) in lines.map_while(Result::ok).enumerate() {
        for (column, char) in line.chars().enumerate() {
            grid.insert((row as i32, column as i32), char);
            if row == 0 {
                columns += 1;
            }
        }
        rows += 1;
    }

    Ok((grid, rows, columns))
}
