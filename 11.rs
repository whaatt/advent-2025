use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let source_to_sinks = match parse_graph() {
        Ok(graph) => graph,
        Err(error) => {
            eprintln!("error: {}", error);
            return;
        }
    };

    println!("{}", problem_1(&source_to_sinks).unwrap_or(usize::MAX));
    println!("{}", problem_2(&source_to_sinks).unwrap_or(usize::MAX));
}

const YOU: &str = "you";
const OUT: &str = "out";
const SVR: &str = "svr";
const DAC: &str = "dac";
const FFT: &str = "fft";

fn problem_1(source_to_sinks: &HashMap<String, Vec<String>>) -> Option<usize> {
    let sorted_sources = topological_sort(
        source_to_sinks,
        &get_nodes_visited_from(source_to_sinks, YOU),
    );
    let mut paths_to: HashMap<String, usize> = HashMap::new();
    paths_to.insert(YOU.to_string(), 1);

    // Iterate over the sources in topological order, accumulating our target
    // metric at each node that has been fully-visited.
    for source in sorted_sources {
        let paths_to_source = *paths_to.get(&source).unwrap_or(&0);
        if let Some(sinks) = source_to_sinks.get(&source) {
            for sink in sinks {
                paths_to
                    .entry(sink.clone())
                    .and_modify(|paths| *paths += paths_to_source)
                    .or_insert(paths_to_source);
            }
        }
    }

    paths_to.get(OUT).copied()
}

fn problem_2(source_to_sinks: &HashMap<String, Vec<String>>) -> Option<usize> {
    let sorted_sources = topological_sort(
        source_to_sinks,
        &get_nodes_visited_from(source_to_sinks, SVR),
    );
    let mut paths_to: HashMap<String, (usize, usize, usize, usize)> = HashMap::new();
    paths_to.insert(SVR.to_string(), (1, 0, 0, 0));

    // Iterate over the sources in topological order, accumulating our target
    // metric at each node that has been fully-visited.
    for source in sorted_sources {
        // Unlike the first problem, we need to segment our metric by DAC/FFT
        // visited status. We use special logic to add these segmented metrics
        // when visiting each node.
        let (mut paths_none, mut paths_dac, mut paths_fft, mut paths_both) =
            *paths_to.get(&source).unwrap_or(&(0, 0, 0, 0));
        if source == DAC {
            paths_both += paths_fft;
            paths_dac += paths_none;
            paths_fft = 0;
            paths_none = 0;
        } else if source == FFT {
            paths_both += paths_dac;
            paths_fft += paths_none;
            paths_dac = 0;
            paths_none = 0;
        }

        let paths_after_visit = (paths_none, paths_dac, paths_fft, paths_both);
        if let Some(sinks) = source_to_sinks.get(&source) {
            for sink in sinks {
                paths_to
                    .entry(sink.clone())
                    .and_modify(|paths| {
                        paths.0 += paths_after_visit.0;
                        paths.1 += paths_after_visit.1;
                        paths.2 += paths_after_visit.2;
                        paths.3 += paths_after_visit.3;
                    })
                    .or_insert(paths_after_visit);
            }
        }
    }

    paths_to.get(OUT).map(|paths| paths.3)
}

fn get_nodes_visited_from(
    source_to_sinks: &HashMap<String, Vec<String>>,
    source: &str,
) -> HashSet<String> {
    let mut nodes_visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(source.to_string());
    while let Some(node) = queue.pop_front() {
        if nodes_visited.contains(&node) {
            continue;
        }

        nodes_visited.insert(node.clone());
        if let Some(sinks) = source_to_sinks.get(&node) {
            for sink in sinks {
                queue.push_back(sink.clone());
            }
        }
    }

    nodes_visited
}

fn topological_sort(
    source_to_sinks: &HashMap<String, Vec<String>>,
    // Restrict the topological sort to only include nodes in a filtered set.
    // Calculated via `get_nodes_visited_from` for a desired source.
    filtered_nodes: &HashSet<String>,
) -> Vec<String> {
    // Count the number of sources that lead to each sink.
    let mut source_count_by_sink = HashMap::new();
    for (source, sinks) in source_to_sinks.iter() {
        if !filtered_nodes.contains(source) {
            continue;
        }

        for sink in sinks {
            if !filtered_nodes.contains(sink) {
                continue;
            }

            source_count_by_sink
                .entry(sink)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    // Start the BFS-like traversal with nodes that have no inbound sources.
    let mut queue = VecDeque::new();
    for source in filtered_nodes {
        if source_count_by_sink.get(&source).unwrap_or(&0) == &0 {
            queue.push_back(source.to_string());
        }
    }

    let mut visited_nodes = HashSet::new();
    let mut result = Vec::new();
    while let Some(source) = queue.pop_front() {
        if visited_nodes.contains(&source) {
            continue;
        }

        visited_nodes.insert(source.clone());
        result.push(source.clone());

        // See which sinks have no more inbound sources and add them to the
        // traversal queue.
        if let Some(sinks) = source_to_sinks.get(&source) {
            for sink in sinks {
                if !filtered_nodes.contains(sink) {
                    continue;
                }

                source_count_by_sink
                    .entry(sink)
                    .and_modify(|count| *count -= 1);
                if source_count_by_sink.get(&sink).unwrap_or(&0) == &0 {
                    queue.push_back(sink.clone());
                }
            }
        }
    }

    result
}

fn parse_graph() -> anyhow::Result<HashMap<String, Vec<String>>> {
    let lines = shared::read_lines("./11.txt")?;
    let mut source_to_sinks = HashMap::new();
    for line in lines {
        let line = line?;
        let (source, sinks) = line
            .split_once(": ")
            .ok_or(anyhow::anyhow!("invalid line"))?;
        let sinks = sinks
            .split(" ")
            .map(|value| value.to_string())
            .collect::<Vec<String>>();
        source_to_sinks.insert(source.to_string(), sinks.clone());
    }

    Ok(source_to_sinks)
}
