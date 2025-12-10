use anyhow::Context;
use good_lp::{
    Constraint, Expression, ProblemVariables, Solution, SolverModel, constraint, microlp, variable,
};
use std::collections::{HashSet, VecDeque};

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

fn problem_1(problems: &[Problem]) -> usize {
    problems
        .iter()
        .map(|problem| problem.solve_lights())
        .sum::<Option<usize>>()
        .unwrap_or(usize::MAX)
}

fn problem_2(problems: &[Problem]) -> usize {
    problems
        .iter()
        .map(|problem| problem.solve_joltages())
        .sum::<Option<usize>>()
        .unwrap_or(usize::MAX)
}

#[derive(Debug)]
struct Problem {
    lights: usize,
    buttons: Vec<usize>,
    joltages: Vec<usize>,
}

impl Problem {
    fn solve_lights(&self) -> Option<usize> {
        let mut seen = HashSet::new();
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        queue.push_back((0, 0, 0));
        while let Some((steps, state, used)) = queue.pop_front() {
            if state == self.lights {
                return Some(steps);
            }

            if seen.contains(&(steps, state, used)) {
                continue;
            }
            seen.insert((steps, state, used));

            for i in 0..self.buttons.len() {
                if used & (1 << i) == 0 {
                    queue.push_back((steps + 1, state ^ self.buttons[i], used | (1 << i)));
                }
            }
        }

        // Not expected.
        None
    }

    fn solve_joltages(&self) -> Option<usize> {
        let mut all_zeros = Vec::new();
        (0..self.joltages.len()).for_each(|_| all_zeros.push(0.0));

        // Build variables representing the number of times each button is
        // pressed; our minimization objective is simply the sum of these
        // variables.
        let mut problem = ProblemVariables::new();
        let variables = (0..self.buttons.len())
            .map(|_| problem.add(variable().integer().min(0)))
            .collect::<Vec<_>>();
        let mut objective: Expression = 0.into();
        for variable in &variables {
            objective += variable
        }

        // Build constraints for each joltage target, where presses of a given
        // button are included in the summation if the button would affect the
        // current target.
        let mut constraints: Vec<Constraint> = Vec::new();
        for (i, joltage) in self.joltages.iter().enumerate() {
            let mut summation: Expression = 0.into();
            for (j, button) in self.buttons.iter().enumerate() {
                let index = self.joltages.len() - 1 - i;
                if button & (1 << index) != 0 {
                    summation += variables[j]
                }
            }
            constraints.push(constraint!(summation == (*joltage as f64)));
        }

        // Solve the LP.
        match problem
            .minimise(&objective)
            .using(microlp)
            .with_all(constraints)
            .solve()
        {
            Ok(solution) => Some(solution.eval(objective).round() as usize),
            Err(_) => None,
        }
    }
}

fn parse_problems() -> anyhow::Result<Vec<Problem>> {
    let lines = shared::read_lines("./10.txt").context("failed to read lines")?;
    let mut problems: Vec<Problem> = Vec::new();
    for line in lines {
        let line = line?;
        let (lights_str, remaining) = line.split_once(" ").context("failed to split lights")?;
        let lights_bin = lights_str
            .chars()
            .filter(|char| *char != '[' && *char != ']')
            .map(|char| if char == '#' { '1' } else { '0' })
            .collect::<String>();
        let lights: usize = usize::from_str_radix(lights_bin.as_str(), 2)?;
        let (buttons_str, joltages_str) = remaining
            .split_once(" {")
            .context("failed to split buttons and joltages")?;
        let buttons: Vec<usize> = buttons_str
            .split(" ")
            .map(|button| {
                let positions = button
                    .chars()
                    .filter(|char| *char != '(' && *char != ')')
                    .collect::<String>()
                    .split(',')
                    .map(|position| {
                        position
                            .parse::<usize>()
                            .context("failed to parse position")
                    })
                    .collect::<anyhow::Result<Vec<usize>>>()?;

                let mut bitmask = 0;
                for position in positions {
                    bitmask |= 1 << (lights_bin.len() - 1 - position);
                }

                Ok(bitmask)
            })
            .collect::<anyhow::Result<Vec<usize>>>()?;
        let joltages: Vec<usize> = joltages_str
            .chars()
            .filter(|char| *char != '}')
            .collect::<String>()
            .split(',')
            .map(|joltage| joltage.parse::<usize>().context("failed to parse joltage"))
            .collect::<anyhow::Result<Vec<usize>>>()?;
        problems.push(Problem {
            lights,
            buttons,
            joltages,
        });
    }

    Ok(problems)
}
