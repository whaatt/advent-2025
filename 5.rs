use anyhow::Context;
use std::cmp::{max, min};

fn main() {
    let (ranges, ingredients) = match read_ranges_and_ingredients() {
        Ok((ranges, ingredients)) => (ranges, ingredients),
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    println!("{}", problem_1(&ranges, &ingredients));
    println!("{}", problem_2(&ranges));
}

fn problem_1(ranges: &[Range], ingredients: &[Ingredient]) -> i64 {
    let mut fresh_count = 0;
    for ingredient in ingredients {
        for range in ranges {
            if *ingredient >= range.0 && *ingredient <= range.1 {
                fresh_count += 1;
                break;
            }
        }
    }

    fresh_count
}

fn problem_2(ranges: &[Range]) -> i64 {
    let mut merged_ranges: Vec<Range> = Vec::new();
    for range in ranges {
        let mut new_merged_ranges = Vec::new();
        let mut i = 0;

        // Add all ranges that are completely before the new range.
        while i < merged_ranges.len() && merged_ranges[i].1 < range.0 {
            new_merged_ranges.push(merged_ranges[i]);
            i += 1;
        }

        // Add the new range after merging existing ranges that overlap.
        let mut new_range = *range;
        loop {
            if i == merged_ranges.len() {
                // Case: No more merged ranges to consider.
                break;
            }

            let new_range_before = new_range.1 < merged_ranges[i].0;
            if new_range_before {
                // Case: Current range is completely before the next merged
                // range.
                break;
            }

            // Case: Current range partially overlaps with the next merged
            // range.
            new_range = (
                min(new_range.0, merged_ranges[i].0),
                max(new_range.1, merged_ranges[i].1),
            );
            i += 1;
        }
        new_merged_ranges.push(new_range);

        // Add all ranges that are completely after the new range.
        while i < merged_ranges.len() {
            new_merged_ranges.push(merged_ranges[i]);
            i += 1;
        }

        merged_ranges = new_merged_ranges;
    }

    merged_ranges
        .iter()
        .map(|range| range.1 - range.0 + 1)
        .sum()
}

type Range = (i64, i64);
type Ingredient = i64;

fn read_ranges_and_ingredients() -> anyhow::Result<(Vec<Range>, Vec<Ingredient>)> {
    let input = shared::read_string("./5.txt").context("failed to read input")?;
    let (ranges_str, ingredients_str) =
        input.split_once("\n\n").context("failed to split input")?;
    let ranges = ranges_str
        .split("\n")
        .map(parse_range)
        .collect::<anyhow::Result<Vec<Range>>>()
        .context("failed to parse ranges")?;
    let ingredients = ingredients_str
        .split("\n")
        .map(|ingredient| {
            ingredient
                .parse::<Ingredient>()
                .map_err(anyhow::Error::from)
        })
        .collect::<anyhow::Result<Vec<Ingredient>>>()
        .context("failed to parse ingredients")?;
    Ok((ranges, ingredients))
}

fn parse_range(range: &str) -> anyhow::Result<Range> {
    let [start, end] = range
        .split('-')
        .map(str::parse::<i64>)
        .map(|result| result.map_err(anyhow::Error::from))
        .collect::<anyhow::Result<Vec<i64>>>()
        .context(format!("failed to parse range: {}", range))?
        .try_into()
        .map_err(|_| anyhow::anyhow!("range does not have exactly two parts: {}", range))?;
    Ok((start, end))
}
