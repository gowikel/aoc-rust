use crate::{
    solvers,
    solvers::{Solution, SolutionExecution},
    Execute,
};
use log::trace;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Formatter};
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

#[derive(Eq, PartialOrd, Ord, PartialEq, Clone, Copy)]
struct SeedRange(u64, u64);

impl Debug for SeedRange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.0, self.0 + self.1 - 1)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct FileData {
    seeds: Vec<u64>,
    seed_ranges: Vec<SeedRange>,
    almanac: HashMap<String, Vec<AlmanacEntry>>,
    almanac_keys: HashMap<String, String>,
    destination: HashMap<String, String>,
    entities: HashSet<String>,
}

impl FileData {
    fn new() -> Self {
        FileData {
            seeds: Vec::new(),
            seed_ranges: Vec::new(),
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

    fn add_seed_range(&mut self, range: SeedRange) {
        self.seed_ranges.push(range);
    }

    fn apply(&self, seed: u64) -> Result<u64, String> {
        let mut current = "seed";
        let mut result = seed;

        while current != "location" {
            let current_key = self
                .almanac_keys
                .get(current)
                .ok_or(format!("No key for {current}"))?;

            let current_entries = self
                .almanac
                .get(current_key)
                .ok_or(format!("No almanac entries for {current}"))?;

            let current_destination = self
                .destination
                .get(current_key)
                .ok_or(format!("No destination for {current}"))?;

            current = current_destination;
            for entry in current_entries {
                let source = entry.0;
                let destination = entry.1;
                let range_length = entry.2;

                if result >= source && result <= source + range_length - 1 {
                    result =
                        destination.wrapping_sub(source).wrapping_add(result);
                    break;
                }
            }
        }

        Ok(result)
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
            let seeds: Vec<u64> = line[7..]
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            seeds.iter().for_each(|s| result.add_seed(*s));

            seeds.chunks_exact(2).for_each(|chunk| {
                let seed_start = chunk[0];
                let seed_length = chunk[1];

                result.add_seed_range(SeedRange(seed_start, seed_length));
            });

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
    let file_data = parse_file(input_path)?;

    let locations: Result<Vec<u64>, String> = file_data
        .seeds
        .iter()
        .map(|seed| file_data.apply(*seed))
        .collect();
    let locations = locations?;

    let result = locations
        .iter()
        .min()
        .expect("At least one location should exists");

    Ok(SolutionExecution::Value(*result))
}

fn solve_part2(input_path: &Path) -> Result<SolutionExecution, String> {
    trace!("Running part 2...");

    let file_data = parse_file(input_path)?;
    let mut queue = VecDeque::new();
    let mut result = u64::MAX;

    for seed_range in file_data.seed_ranges.iter() {
        queue.push_back(*seed_range);
    }

    while let Some(seed_range) = queue.pop_front() {
        let seed_start = seed_range.0;
        let seed_length = seed_range.1;
        let seed_end = seed_start + seed_length - 1;

        let v0 = file_data.apply(seed_start)?;
        let v1 = file_data.apply(seed_end)?;

        if seed_length > 2
            && v1.wrapping_sub(v0) != seed_length - 1
            && v0.wrapping_sub(v1) != seed_length - 1
        {
            let midpoint = (seed_start + seed_end) / 2;
            let offset = match seed_length % 2 {
                0 => 1,
                1 => 0,
                _ => unreachable!(),
            };

            queue.push_back(SeedRange(seed_start, seed_length / 2));
            queue.push_back(SeedRange(
                midpoint + offset,
                seed_length / 2 + seed_length % 2,
            ));
            continue;
        }

        if v0 < result {
            result = v0;
        }

        if v1 < result {
            result = v1;
        }
    }

    Ok(SolutionExecution::Value(result))
}
