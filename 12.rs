use anyhow::Context;
use std::collections::HashMap;

// Note: This problem is a troll.
// My method actually works but only because the input is constructed in a way
// where solvable grids are trivial and all other grids can be bailed out on
// after a certain number of iterations. Happy holidays!
fn main() {
    let (pieces, mut grids) = match parse_grid() {
        Ok(grid) => grid,
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    println!("{}", problem_1(&pieces, &mut grids));
}

fn problem_1(pieces: &[Piece], grids: &mut [Grid]) -> usize {
    grids
        .iter_mut()
        .map(|grid| solve_grid(grid, pieces))
        .filter(|result| result.as_ref().is_ok_and(|result| *result))
        .count()
}

fn solve_grid(grid: &mut Grid, pieces: &[Piece]) -> anyhow::Result<bool> {
    // Hack to bail out on hard grids.
    if grid.trials > 10000000 {
        return Err(anyhow::anyhow!("trials limit reached"));
    }

    if grid.remaining.iter().all(|(_, count)| *count == 0) {
        return Ok(true);
    }

    // Backtracking packer algorithm.
    for index in 0..pieces.len() {
        for orientation in 0..pieces[index].grids.len() {
            for row in 0..grid.rows {
                for column in 0..grid.columns {
                    if grid.place(pieces, index, orientation, row, column) {
                        let result = solve_grid(grid, pieces);
                        match result {
                            Ok(true) => return Ok(true),
                            Ok(false) => {
                                let _ = grid.pop_last(pieces);
                                continue;
                            }
                            Err(error) => return Err(error),
                        }
                    }
                }
            }
        }
    }

    Ok(false)
}

#[derive(Debug)]
struct Piece {
    grids: Vec<Vec<Vec<char>>>,
}

#[derive(Debug)]
struct Grid {
    rows: usize,
    columns: usize,
    grid: Vec<Vec<char>>,
    trials: usize,
    placed: Vec<(usize, usize, usize, usize)>,
    remaining: HashMap<usize, usize>,
}

impl Grid {
    fn place(
        &mut self,
        pieces: &[Piece],
        index: usize,
        orientation: usize,
        row: usize,
        column: usize,
    ) -> bool {
        self.trials += 1;
        if *self.remaining.get(&index).unwrap_or(&0) == 0 {
            return false;
        }

        // Check if the piece can be placed.
        let piece = &pieces[index];
        let piece_grid = &piece.grids[orientation];
        for row_offset in 0..piece_grid.len() {
            #[allow(clippy::needless_range_loop)]
            for column_offset in 0..piece_grid[0].len() {
                // Non-empty piece cell out of bounds.
                if (row + row_offset >= self.rows || column + column_offset >= self.columns)
                    && piece_grid[row_offset][column_offset] != '.'
                {
                    return false;
                }

                // Non-empty piece cell overlaps with another piece.
                if piece_grid[row_offset][column_offset] != '.'
                    && self.grid[row + row_offset][column + column_offset] != '.'
                {
                    return false;
                }
            }
        }

        // Place the piece.
        for row_offset in 0..piece_grid.len() {
            #[allow(clippy::needless_range_loop)]
            for column_offset in 0..piece_grid[0].len() {
                if piece_grid[row_offset][column_offset] != '.' {
                    self.grid[row + row_offset][column + column_offset] =
                        piece_grid[row_offset][column_offset];
                }
            }
        }

        // Update internal grid state for piece tracking.
        self.placed.push((index, orientation, row, column));
        self.remaining.entry(index).and_modify(|count| *count -= 1);
        true
    }

    fn pop_last(&mut self, pieces: &[Piece]) -> anyhow::Result<()> {
        let (index, orientation, row, column) = self
            .placed
            .pop()
            .ok_or(anyhow::anyhow!("no last placed piece"))?;
        let piece = &pieces[index];
        let piece_grid = &piece.grids[orientation];
        for row_offset in 0..piece_grid.len() {
            #[allow(clippy::needless_range_loop)]
            for column_offset in 0..piece_grid[0].len() {
                // Cell out of bounds; nothing to do.
                if row + row_offset >= self.rows || column + column_offset >= self.columns {
                    continue;
                }

                // Cell was occupied by the piece; clear it.
                if piece_grid[row_offset][column_offset] != '.' {
                    self.grid[row + row_offset][column + column_offset] = '.';
                }
            }
        }

        // Update internal grid state for piece tracking.
        self.remaining.entry(index).and_modify(|count| *count += 1);
        Ok(())
    }
}

fn parse_grid() -> anyhow::Result<(Vec<Piece>, Vec<Grid>)> {
    let content = shared::read_string("./12.txt").context("failed to read file")?;
    let (pieces, grids) = content
        .rsplit_once("\n\n")
        .context("failed to split pieces and grids")?;
    let pieces = pieces
        .split("\n\n")
        .map(|piece| -> anyhow::Result<Piece> {
            let lines = piece.split("\n").collect::<Vec<&str>>()[1..].to_vec();
            let grid = lines
                .iter()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();
            Ok(Piece {
                grids: get_orientations(&grid),
            })
        })
        .collect::<anyhow::Result<Vec<Piece>>>()
        .context("failed to collect pieces")?;
    let grids = grids
        .split("\n")
        .map(|line| -> anyhow::Result<Grid> {
            let (dimensions, counts) = line
                .split_once(": ")
                .context("failed to split dimensions and counts")?;
            let (rows, columns) = dimensions
                .split_once("x")
                .context("failed to split rows and columns")?;
            let rows = rows.parse::<usize>().context("failed to parse rows")?;
            let columns = columns
                .parse::<usize>()
                .context("failed to parse columns")?;
            let counts = counts
                .split(" ")
                .map(|count| count.parse::<usize>().context("failed to parse count"))
                .collect::<anyhow::Result<Vec<usize>>>()
                .context("failed to collect counts")?;
            Ok(Grid {
                rows,
                columns,
                grid: vec![vec!['.'; columns]; rows],
                trials: 0,
                placed: Vec::new(),
                remaining: counts
                    .iter()
                    .enumerate()
                    .map(|(piece_index, count)| (piece_index, *count))
                    .collect(),
            })
        })
        .collect::<anyhow::Result<Vec<Grid>>>()
        .context("failed to collect grids")?;
    Ok((pieces, grids))
}

fn get_orientations(grid: &[Vec<char>]) -> Vec<Vec<Vec<char>>> {
    let mut orientations = Vec::new();
    let mut new_grid = grid.to_owned();
    for _ in 0..4 {
        orientations.push(new_grid.clone());
        new_grid = rotate_left(&new_grid);
    }

    orientations
}

fn rotate_left(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    if grid.is_empty() || grid[0].is_empty() {
        return vec![];
    }

    let mut new_grid = vec![vec![' '; grid.len()]; grid[0].len()];
    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            new_grid[grid[0].len() - column - 1][row] = grid[row][column];
        }
    }

    new_grid
}
