use anyhow::Context;
use std::{cmp::Ordering, collections::HashMap};

fn main() {
    let coordinates = match parse_coordinates() {
        Ok(coordinates) => coordinates,
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    println!("{}", problem(&coordinates, Some(1000)).0);
    println!("{}", problem(&coordinates, None).1);
}

fn problem(coordinates: &[Coordinate], pairs_to_consider: Option<usize>) -> (i64, i64) {
    let mut circuits = HashMap::new();
    for (circuit_id, coordinate) in coordinates.iter().enumerate() {
        circuits.insert(coordinate, circuit_id);
    }

    let mut pairs = Vec::new();
    for i in 0..coordinates.len() {
        for j in i + 1..coordinates.len() {
            pairs.push((
                distance(&coordinates[i], &coordinates[j]),
                &coordinates[i],
                &coordinates[j],
            ));
        }
    }

    // Cubic complexity loop (could use Union-Find to reduce to effective
    // quadratic complexity).
    let mut last_joined_x_product = 0;
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
    for (pairs_considered, (_, coordinate_first, coordinate_second)) in
        pairs.into_iter().enumerate()
    {
        if let Some(pairs_to_consider) = pairs_to_consider
            && pairs_considered >= pairs_to_consider
        {
            break;
        }

        let circuit_id_first = circuits[coordinate_first];
        let circuit_id_second = circuits[coordinate_second];
        if circuit_id_first == circuit_id_second {
            continue;
        }

        // Make all coordinates with `circuit_id_second` have
        // `circuit_id_first`.
        last_joined_x_product = coordinate_first.x * coordinate_second.x;
        let mut should_break = true;
        for coordinate in coordinates {
            if circuits[&coordinate] != circuit_id_first {
                should_break = false;
            }

            if circuits[&coordinate] == circuit_id_second {
                circuits.insert(coordinate, circuit_id_first);
            }
        }

        // Break early if no coordinates were joined.
        if should_break {
            break;
        }
    }

    let mut counts = HashMap::new();
    for circuit_id in circuits.values() {
        *(counts.entry(*circuit_id).or_insert(0)) += 1;
    }

    let mut top_counts = counts.values().collect::<Vec<_>>();
    top_counts.sort();
    top_counts.reverse();
    (
        if top_counts.len() >= 3 {
            top_counts[0] * top_counts[1] * top_counts[2]
        } else {
            0
        },
        last_joined_x_product,
    )
}

fn distance(coordinate_first: &Coordinate, coordinate_second: &Coordinate) -> f64 {
    (((coordinate_first.x - coordinate_second.x).pow(2)
        + (coordinate_first.y - coordinate_second.y).pow(2)
        + (coordinate_first.z - coordinate_second.z).pow(2)) as f64)
        .sqrt()
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}

impl Coordinate {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

fn parse_coordinates() -> anyhow::Result<Vec<Coordinate>> {
    let lines = shared::read_lines("./8.txt").context("failed to read lines")?;
    let mut coordinates = Vec::new();
    for line in lines {
        let line = line.context("failed to read line")?;
        let parts = line.split(",").collect::<Vec<&str>>();
        let [x, y, z] = parts
            .into_iter()
            .map(|part| part.parse().map_err(anyhow::Error::from))
            .collect::<anyhow::Result<Vec<i64>>>()?
            .try_into()
            .map_err(|_| anyhow::anyhow!("failed to parse coordinates"))?;
        coordinates.push(Coordinate::new(x, y, z));
    }

    Ok(coordinates)
}
