use anyhow::Context;
use std::collections::HashMap;

fn main() {
    let (rows, columns, grid) = match parse_grid() {
        Ok(grid) => grid,
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    let mut start: Option<(i64, i64)> = None;
    for (row, column) in grid.keys() {
        if *grid.get(&(*row, *column)).unwrap() == 'S' {
            start = Some((*row, *column));
            break;
        }
    }
    let start = match start {
        Some(start) => start,
        None => {
            eprintln!("start not found");
            return;
        }
    };

    println!("{}", problem_1(rows, columns, &mut grid.clone(), start));
    println!("{}", problem_2(rows, columns, &mut grid.clone(), start));
}

fn problem_1(
    rows: i64,
    columns: i64,
    grid: &mut HashMap<(i64, i64), char>,
    start: (i64, i64),
) -> i64 {
    let mut split_count = 0;
    for row_current in (start.0 + 1)..rows {
        for column in 0..columns {
            let current_char = *grid.get(&(row_current, column)).unwrap_or(&'.');
            let above_char = *grid.get(&(row_current - 1, column)).unwrap_or(&'.');

            if current_char == '^' && (above_char == 'S' || above_char == '|') {
                grid.insert((row_current, column - 1), '|');
                grid.insert((row_current, column + 1), '|');
                split_count += 1;
            } else if above_char == 'S' || above_char == '|' {
                grid.insert((row_current, column), '|');
            }
        }
    }

    split_count
}

fn problem_2(
    rows: i64,
    columns: i64,
    grid: &mut HashMap<(i64, i64), char>,
    start: (i64, i64),
) -> i64 {
    let mut path_counts = HashMap::new();
    for row_current in (start.0 + 1)..rows {
        for column in 0..columns {
            let current_char = *grid.get(&(row_current, column)).unwrap_or(&'.');
            let above_char = *grid.get(&(row_current - 1, column)).unwrap_or(&'.');
            let above_path_count = *path_counts.get(&(row_current - 1, column)).unwrap_or(&1);

            if current_char == '^' && (above_char == 'S' || above_char == '|') {
                grid.insert((row_current, column - 1), '|');
                grid.insert((row_current, column + 1), '|');
                *path_counts.entry((row_current, column - 1)).or_insert(0) += above_path_count;
                *path_counts.entry((row_current, column + 1)).or_insert(0) += above_path_count;
            } else if above_char == 'S' || above_char == '|' {
                grid.insert((row_current, column), '|');
                *path_counts.entry((row_current, column)).or_insert(0) += above_path_count;
            }
        }
    }

    let mut last_row_total = 0;
    for column in 0..columns {
        last_row_total += *path_counts.get(&(rows - 1, column)).unwrap_or(&0);
    }

    last_row_total
}

#[allow(clippy::type_complexity)]
fn parse_grid() -> anyhow::Result<(i64, i64, HashMap<(i64, i64), char>)> {
    let lines = shared::read_lines("./7.txt").context("failed to read lines")?;

    let mut rows = 0;
    let mut columns = 0;
    let mut grid = HashMap::new();
    for (row, line) in lines.map_while(Result::ok).enumerate() {
        for (column, char) in line.chars().enumerate() {
            grid.insert((row as i64, column as i64), char);
            if row == 0 {
                columns = (column + 1) as i64;
            }
            rows = (row + 1) as i64;
        }
    }

    Ok((rows, columns, grid))
}
