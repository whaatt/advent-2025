use anyhow::Context;
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let banks = match parse_banks() {
        Ok(banks) => banks,
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    println!("{}", problem_1(&banks));
    println!("{}", problem_2(&banks));
}

fn problem_1(banks: &Vec<Vec<i64>>) -> i64 {
    let mut total_joltage = 0;
    for bank in banks {
        let mut largest_seen_two_digits = -1;
        let mut largest_seen_digit = -1;
        for digit in bank {
            let largest_two_digits_here = largest_seen_digit * 10 + digit;
            if *digit > largest_seen_digit {
                largest_seen_digit = *digit;
            }
            if largest_two_digits_here > largest_seen_two_digits {
                largest_seen_two_digits = largest_two_digits_here;
            }
        }

        total_joltage += largest_seen_two_digits;
    }

    total_joltage
}

/// `T[i, k]`: After considering `i` elements of the bank and accepting `k`
/// proposals (out of some maximum `M`), this is the highest value we were able
/// to construct:
///     - `T[0, k] = 0`
///     - `T[i, 0] = 0`
///     - `T[i + 1, k] = max(T[i, k], T[i,  k - 1] * 10 + B[i])`
fn problem_2(banks: &Vec<Vec<i64>>) -> i64 {
    let max_proposals = 12;
    let mut total_joltage = 0;
    for bank in banks {
        let mut t: HashMap<(usize, u32), i64> = HashMap::new();
        for k in 0..=max_proposals {
            for (i, digit) in bank.iter().enumerate() {
                if k == 0 {
                    t.insert((i + 1, k), 0);
                } else {
                    let skip_position_value = *t.get(&(i, k)).unwrap_or(&0);
                    let take_position_value = t.get(&(i, k - 1)).unwrap_or(&0) * 10 + *digit;
                    t.insert((i + 1, k), max(skip_position_value, take_position_value));
                }
            }
        }

        total_joltage += *t.get(&(bank.len(), 12)).unwrap_or(&0);
    }

    total_joltage
}

fn parse_banks() -> anyhow::Result<Vec<Vec<i64>>> {
    let mut banks = Vec::new();
    let lines = shared::read_lines("./3.txt").context("failed to read lines")?;
    for line in lines.map_while(Result::ok) {
        let mut bank = Vec::new();
        for char in line.chars() {
            bank.push(
                char.to_digit(10)
                    .context(format!("failed to parse digit: {}", char))? as i64,
            );
        }
        banks.push(bank);
    }

    Ok(banks)
}
