use shared::read_lines;

// Modulus for the dial.
const MODULUS: i32 = 100;

fn main() {
    let result = get_rotations(MODULUS);
    let rotations = match result {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    println!("{}", problem_1(&rotations));
    println!("{}", problem_2(&rotations));
}

fn problem_1(rotations: &Vec<Rotation>) -> i32 {
    let mut state = 50;
    let mut returns = 0;
    for rotation in rotations {
        rotation.apply_and_count_passing_turns(&mut state);
        if state == 0 {
            returns += 1;
        }
    }
    returns
}

fn problem_2(rotations: &Vec<Rotation>) -> i32 {
    let mut state = 50;
    let mut returns = 0;
    for rotation in rotations {
        let passing_turns = rotation.apply_and_count_passing_turns(&mut state);
        returns += passing_turns;
    }
    returns
}

enum Rotation {
    R { modulus: i32, clicks: i32 },
    L { modulus: i32, clicks: i32 },
}

impl Rotation {
    fn apply_and_count_passing_turns(&self, state: &mut i32) -> i32 {
        match self {
            Rotation::R { modulus, clicks } => {
                let old_state = *state;
                *state = (*state + clicks).rem_euclid(*modulus);
                let full_turns = clicks / modulus;
                full_turns
                    + if *state != old_state && *state < old_state {
                        1
                    } else {
                        0
                    }
            }
            Rotation::L { modulus, clicks } => {
                let old_state = *state;
                *state = (*state - clicks).rem_euclid(*modulus);
                let full_turns = clicks / modulus;
                // Ugly; can we simplify this?
                full_turns
                    + if *state != old_state
                        && (*state == 0 || (*state > old_state && old_state != 0))
                    {
                        1
                    } else {
                        0
                    }
            }
        }
    }
}

fn get_rotations(modulus: i32) -> Result<Vec<Rotation>, std::string::String> {
    let mut rotations = Vec::new();
    let lines = read_lines("./1.txt").map_err(|error| error.to_string())?;
    for line in lines.map_while(Result::ok) {
        let (rotation, clicks_str) = line.split_at(1);
        let clicks: i32 = clicks_str.parse().expect("Invalid number");
        match rotation {
            "R" => rotations.push(Rotation::R { modulus, clicks }),
            "L" => rotations.push(Rotation::L { modulus, clicks }),
            _ => return Err(format!("Invalid rotation character: {}", rotation)),
        }
    }
    Ok(rotations)
}
