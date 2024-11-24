use crate::{
    solvers,
    solvers::{Solution, SolutionExecution},
    Execute,
};
use log::{debug, trace};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Solves both parts of day 05's puzzle
///
/// # Arguments
///
/// * `execute` - Execution mode configuration
/// * `input_path` - Path to the input file containing the schematic
///
/// # Returns
///
/// Array containing solutions for both parts of the puzzle
pub fn solve(execute: Execute, input_path: &Path) -> [Solution; 2] {
    trace!("Running solver for day 05 with Execute {}...", execute);
    solvers::common_solve(execute, input_path, solve_part1, solve_part2)
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
struct AlmanacEntry(u64, u64, u64);

#[derive(Debug, Eq, PartialEq, Clone)]
struct FileData {
    seeds: Vec<u64>,
    almanac: HashMap<String, Vec<AlmanacEntry>>,
    almanac_keys: HashMap<String, String>,
    destination: HashMap<String, String>,
    entities: HashSet<String>,
}

impl FileData {
    fn new() -> Self {
        FileData {
            seeds: Vec::new(),
            almanac: HashMap::new(),
            almanac_keys: HashMap::new(),
            destination: HashMap::new(),
            entities: HashSet::new(),
        }
    }

    fn add_seed(&mut self, seed: u64) {
        self.seeds.push(seed);
    }

    fn add_almanac_entry(&mut self, target: String, entry: AlmanacEntry) {
        self.almanac
            .entry(target)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    fn add_entity(&mut self, entity: String) {
        self.entities.insert(entity);
    }

    fn add_almanac_key(&mut self, entity: String, transformation: String) {
        self.almanac_keys.insert(entity, transformation);
    }

    fn add_destination(&mut self, entity: String, destination: String) {
        self.destination.insert(entity, destination);
    }
}

fn parse_file(input_path: &Path) -> Result<FileData, String> {
    let mut result = FileData::new();
    let file = File::open(input_path).map_err(|e| e.to_string())?;
    let buffer = BufReader::new(file);

    let mut current_target = None;

    for line in buffer.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds: ") {
            line[7..]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .for_each(|s| result.add_seed(s));

            continue;
        }

        if line.contains("map:") {
            let line = line
                .split_once(" ")
                .ok_or(format!("Failed to split line: {line}"))?;
            current_target = Some(line.0.to_string());

            let entities = line
                .0
                .split_once("-to-")
                .ok_or(format!("Invalid format: {}", line.0))?;

            result.add_entity(entities.0.to_string());
            result.add_entity(entities.1.to_string());
            result.add_almanac_key(entities.0.to_string(), line.0.to_owned());
            result.add_destination(line.0.to_owned(), entities.1.to_string());

            continue;
        }

        match current_target {
            None => {
                return Err(format!("Expected target before {line}"));
            }
            Some(ref target) => {
                let values: Vec<u64> = line
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                if values.len() != 3 {
                    return Err(format!("Expected three values from: {line}"));
                }

                let destination_start = values[0];
                let source_start = values[1];
                let range_length = values[2];

                result.add_almanac_entry(
                    target.to_owned(),
                    AlmanacEntry(source_start, destination_start, range_length),
                );
            }
        }
    }

    Ok(result)
}

fn solve_part1(input_path: &Path) -> Result<SolutionExecution, String> {
    let file_data = parse_file(input_path).map_err(|e| e.to_string())?;

    let mut data = file_data.seeds.clone();
    let mut current = "seed";

    while current != "location" {
        let current_key = file_data
            .almanac_keys
            .get(current)
            .ok_or(format!("No key for {current}"))?;
        let current_entries = file_data
            .almanac
            .get(current_key)
            .ok_or(format!("No almanac entries for {current}"))?;
        let current_destination = file_data
            .destination
            .get(current_key)
            .ok_or(format!("No destination for {current}"))?;

        data = data
            .iter()
            .map(|&v| {
                for entry in current_entries {
                    let source = entry.0;
                    let destination = entry.1;
                    let range_length = entry.2;

                    if v >= source && v <= source + range_length {
                        return destination
                            .wrapping_sub(source)
                            .wrapping_add(v);
                    }
                }

                v
            })
            .collect();

        current = current_destination;
    }

    let result = data
        .iter()
        .min()
        .expect("At least one location has to exist");

    Ok(SolutionExecution::Value(*result))
}

fn solve_part2(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 2...");

    Ok(SolutionExecution::NotImplemented)
}
