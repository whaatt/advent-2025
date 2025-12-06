fn main() {
    let problems = match parse_problems() {
        Ok(problems) => problems,
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    println!("{}", problem_1(&problems));
    println!("{}", problem_2(&problems));
}

fn problem_1(problems: &[Problem]) -> i64 {
    let mut solution_total = 0;
    for problem in problems {
        solution_total += problem.solve();
    }

    solution_total
}

fn problem_2(problems: &[Problem]) -> i64 {
    let mut solution_total = 0;
    for problem in problems {
        solution_total += problem.solve_vertical();
    }

    solution_total
}

struct Problem {
    grid_width: usize,
    values: Vec<i64>,
    values_vertical: Vec<i64>,
    reducer: (i64, fn(i64, i64) -> i64),
}

impl Problem {
    fn solve(&self) -> i64 {
        let mut total = self.reducer.0;
        for value in self.values.iter() {
            total = self.reducer.1(total, *value);
        }

        total
    }

    fn solve_vertical(&self) -> i64 {
        let mut total = self.reducer.0;
        for value in self.values_vertical.iter() {
            total = self.reducer.1(total, *value);
        }

        total
    }
}

const ADD: (i64, fn(i64, i64) -> i64) = (0, |a, b| a + b);
const MULTIPLY: (i64, fn(i64, i64) -> i64) = (1, |a, b| a * b);

#[allow(clippy::type_complexity)]
fn parse_problems() -> anyhow::Result<Vec<Problem>> {
    let lines_buffer: std::io::Lines<std::io::BufReader<std::fs::File>> =
        shared::read_lines("6.txt")?;
    let lines: Vec<String> = lines_buffer.map_while(Result::ok).collect::<Vec<String>>();
    let mut problems: Vec<Problem> = Vec::new();
    let operations = lines
        .last()
        .ok_or_else(|| anyhow::anyhow!("failed to get operations"))?;

    // Operation spacing is used to constrain parsing of vertical values.
    let mut reducer: Option<(i64, fn(i64, i64) -> i64)> = None;
    let mut spaces_current = 0;
    for char in operations.chars() {
        if char == ' ' {
            spaces_current += 1;
        } else {
            if let Some(reducer) = reducer {
                let grid_width = spaces_current;
                problems.push(Problem {
                    grid_width,
                    values: vec![0; lines.len() - 1],
                    values_vertical: vec![0; grid_width],
                    reducer,
                });
            }
            if char == '*' {
                reducer = Some(MULTIPLY);
            } else if char == '+' {
                reducer = Some(ADD);
            }
            spaces_current = 0
        }
    }
    // Duplicate code to push the final problem.
    if let Some(reducer) = reducer {
        // Plus one to account for there not being another operation (and
        // therefore no extra space counted) after the last operation.
        let grid_width = spaces_current + 1;
        problems.push(Problem {
            grid_width,
            values: vec![0; lines.len() - 1],
            values_vertical: vec![0; grid_width],
            reducer,
        });
    }

    // Parse each of the value lines horizontally and vertically.
    for (i, line) in lines.iter().enumerate().take(lines.len() - 1) {
        // Parse the horizontal values (just based on a whitespace split).
        let values: Vec<i64> = line
            .split_whitespace()
            .map(|x| x.parse::<i64>().map_err(anyhow::Error::from))
            .collect::<anyhow::Result<Vec<i64>>>()?;
        for (j, value) in values.iter().enumerate() {
            problems[j].values[i] = *value;
        }

        let mut scan_index = 0;
        for problem in problems.iter_mut() {
            if scan_index >= line.len() {
                break;
            }

            // Parse the vertical values (skip spaces and multiply the
            // accumulated value by 10 before adding new digits on each value
            // line).
            for position in scan_index..(scan_index + problem.grid_width) {
                let problem_index = problem.values_vertical.len() - (position - scan_index) - 1;
                let byte = line.as_bytes()[position];
                if byte == b' ' {
                    continue;
                }

                problem.values_vertical[problem_index] *= 10;
                problem.values_vertical[problem_index] += (byte - b'0') as i64;
            }

            scan_index += problem.grid_width + 1;
        }
    }

    Ok(problems)
}
