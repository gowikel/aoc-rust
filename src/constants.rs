//! Constants for AoC project

use std::ops::RangeInclusive;

/// Years when I have solutions for the AoC games
pub const VALID_YEARS: RangeInclusive<u32> = 2023..=2023;

/// December days that contain a puzzle
pub const VALID_DAYS: RangeInclusive<u32> = 1..=25;

/// The require AOC_COOKIE
pub const AOC_COOKIE: &str = "AOC_COOKIE";
