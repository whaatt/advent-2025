use anyhow::Context;
use std::cmp::{max, min};

fn main() {
    let coordinates = match parse_coordinates() {
        Ok(coordinates) => coordinates,
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    println!("{}", problem_1(&coordinates));
    println!("{}", problem_2(&coordinates));
}

fn problem_1(coordinates: &[Coordinate]) -> i64 {
    let mut max_area = 0;
    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            let area = area(&coordinates[i], &coordinates[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn problem_2(coordinates: &[Coordinate]) -> i64 {
    let mut max_area = 0;
    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            let first = &coordinates[i];
            let second = &coordinates[j];

            let min_row = min(first.row, second.row);
            let max_row = max(first.row, second.row);
            let min_column = min(first.column, second.column);
            let max_column = max(first.column, second.column);

            // See if any obstacles are within the rectangle.
            let mut should_skip = false;
            for m in 0..coordinates.len() {
                let n = (m + 1) % coordinates.len();
                // Row obstacle is within the row range; check if the column
                // range of the obstacle is within the strict column range
                // of the rectangle (i.e. excluding the border).
                if coordinates[m].row == coordinates[n].row
                    && min_row < coordinates[m].row
                    && coordinates[m].row < max_row
                    && range_overlaps(
                        min_column + 1,
                        max_column - 1,
                        min(coordinates[m].column, coordinates[n].column),
                        max(coordinates[m].column, coordinates[n].column),
                    )
                {
                    should_skip = true;
                    break;
                }

                // Column obstacle is within the column range; check if the row
                // range of the obstacle is within the strict row range of the
                // rectangle (i.e. excluding the border).
                if coordinates[m].column == coordinates[n].column
                    && min_column < coordinates[m].column
                    && coordinates[m].column < max_column
                    && range_overlaps(
                        min_row + 1,
                        max_row - 1,
                        min(coordinates[m].row, coordinates[n].row),
                        max(coordinates[m].row, coordinates[n].row),
                    )
                {
                    should_skip = true;
                    break;
                }
            }

            // Skip the pair if any obstacles are within the rectangle.
            if should_skip {
                continue;
            }

            let area = area(&coordinates[i], &coordinates[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn range_overlaps(a_start: i64, a_end: i64, b_start: i64, b_end: i64) -> bool {
    // Return false for degenerate ranges.
    if a_end < a_start || b_end < b_start {
        return false;
    }

    (a_start >= b_start && a_start <= b_end) || (a_end >= b_start && a_end <= b_end)
}

fn area(coordinate_first: &Coordinate, coordinate_second: &Coordinate) -> i64 {
    ((coordinate_first.row - coordinate_second.row).abs() + 1)
        * ((coordinate_first.column - coordinate_second.column).abs() + 1)
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct Coordinate {
    row: i64,
    column: i64,
}

impl Coordinate {
    fn new(row: i64, column: i64) -> Self {
        Self { row, column }
    }
}

fn parse_coordinates() -> anyhow::Result<Vec<Coordinate>> {
    let lines = shared::read_lines("./9.txt").context("failed to read lines")?;
    let mut coordinates = Vec::new();
    for line in lines {
        let line = line.context("failed to read line")?;
        let parts = line.split(",").collect::<Vec<&str>>();
        let [column, row] = parts
            .into_iter()
            .map(|part| part.parse().map_err(anyhow::Error::from))
            .collect::<anyhow::Result<Vec<i64>>>()?
            .try_into()
            .map_err(|_| anyhow::anyhow!("failed to parse coordinates"))?;
        coordinates.push(Coordinate::new(row, column));
    }

    Ok(coordinates)
}
