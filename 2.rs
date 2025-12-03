use shared::read_lines;
use std::collections::HashSet;

fn main() {
    let ranges = match parse_ranges() {
        Ok(ranges) => ranges,
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    println!("{}", problem_1(&ranges));
    println!("{}", problem_2(&ranges));
}

fn problem_1(ranges: &[Range]) -> i64 {
    ranges.iter().map(sum_invalid).sum()
}

fn sum_invalid(range: &Range) -> i64 {
    let mut sum_invalid_total = 0;
    let digits_start = count_digits(range.0);
    let digits_end = count_digits(range.1);

    let mut current_start = range.0;
    let mut current_end: i64;
    // Iterate over each digit count in the range.
    for digit_count in digits_start..=digits_end {
        // This iteration either ends at the overall range end or at `99...9`
        // in the current digit count.
        if digit_count == digits_end {
            current_end = range.1
        } else {
            current_end = 10_i64.pow(digit_count as u32) - 1;
        }

        // No odd-digit numbers can be invalid.
        if digit_count % 2 != 0 {
            current_start = current_end + 1;
            continue;
        }

        // Sum invalid numbers in range (where the first half of number is
        // duplicated).
        let current_start_first_half = current_start / 10_i64.pow(digit_count as u32 / 2);
        let current_start_last_half = current_start % 10_i64.pow(digit_count as u32 / 2);
        let current_end_first_half = current_end / 10_i64.pow(digit_count as u32 / 2);
        let current_end_last_half = current_end % 10_i64.pow(digit_count as u32 / 2);
        for number in current_start_first_half..=current_end_first_half {
            // Handle cases where duplicating the first half is not in the
            // range as implied by the last half.
            if number == current_start_first_half
                && number == current_start_last_half
                && (number < current_start_last_half || number > current_end_last_half)
            {
                // Case 1: Start and end have same first half and number is not
                // in the range implied by the last halves.
                continue;
            }
            if number == current_start_first_half && number < current_start_last_half {
                // Case 2: Number has same first half as start but less than
                // its last half.
                continue;
            }
            if number == current_end_first_half && number > current_end_last_half {
                // Case 3: Number has same first half as end but greater than
                // its last half.
                continue;
            }
            sum_invalid_total += number * 10_i64.pow(digit_count as u32 / 2) + number;
        }

        current_start = current_end + 1;
    }

    sum_invalid_total
}

fn count_digits(number: i64) -> i64 {
    number.to_string().len() as i64
}

fn problem_2(ranges: &[Range]) -> i64 {
    let mut seen_numbers = HashSet::new();
    let mut sum_invalid_total = 0;
    for range in ranges {
        let digits_start = count_digits(range.0);
        let digits_end = count_digits(range.1);

        let mut current_start = range.0;
        let mut current_end: i64;
        // Iterate over each digit count in the range.
        for digit_count in digits_start..=digits_end {
            // This iteration either ends at the overall range end or at `99...9`
            // in the current digit count.
            if digit_count == digits_end {
                current_end = range.1
            } else {
                current_end = 10_i64.pow(digit_count as u32) - 1;
            }

            // Iterate over each possible split size.
            for split_size in 1..=digit_count / 2 {
                if digit_count % split_size != 0 {
                    continue;
                }

                let current_start_first_part =
                    current_start / 10_i64.pow(digit_count as u32 - split_size as u32);
                let current_end_first_part =
                    current_end / 10_i64.pow(digit_count as u32 - split_size as u32);

                // Iterate over each value to duplicate.
                for value in current_start_first_part..=current_end_first_part {
                    // Replicate `value` by `digit_count / split_size` times.
                    let replicated_value = value
                        .to_string()
                        .repeat(digit_count as usize / split_size as usize);
                    let replicated_value_i64 = replicated_value.parse::<i64>().unwrap();

                    // Brute-force check for being in range.
                    if replicated_value_i64 < current_start || replicated_value_i64 > current_end {
                        continue;
                    }

                    // Skip if we've already seen this number (can happen with
                    // overlapping splits as in 222222).
                    if seen_numbers.contains(&replicated_value_i64) {
                        continue;
                    }

                    seen_numbers.insert(replicated_value_i64);
                    sum_invalid_total += replicated_value_i64;
                }
            }

            current_start = current_end + 1;
        }
    }

    sum_invalid_total
}

type Range = (i64, i64);

fn parse_ranges() -> Result<Vec<Range>, std::string::String> {
    let lines: std::io::Lines<std::io::BufReader<std::fs::File>> =
        read_lines("./2.txt").map_err(|error| error.to_string())?;
    let mut ranges = Vec::new();
    for line in lines.map_while(Result::ok) {
        for range_str in line.split(',') {
            match parse_range(range_str) {
                Ok(range) => ranges.push(range),
                Err(error) => return Err(error),
            }
        }
    }

    Ok(ranges)
}

fn parse_range(range: &str) -> Result<Range, std::string::String> {
    let [start, end] = range
        .split('-')
        .map(str::parse::<i64>)
        .map(|result| result.map_err(|error| error.to_string()))
        .collect::<Result<Vec<i64>, std::string::String>>()?
        .try_into()
        .map_err(|_| format!("range does not have exactly two parts: {}", range))?;
    Ok((start, end))
}
