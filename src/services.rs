//! The services module provides high-level abstractions for file system operations
//! and HTTP operations required for the Advent of Code puzzles.
//!
//! It re-exports the FSService and HTTPService structs from their respective modules.
//!
//! # Overview
//!
//! This module encapsulates two core services:
//! * FSService: Manages file system operations, such as extracting and writing puzzle
//! templates.
//! * HTTPService: Manages HTTP operations, such as downloading puzzle inputs from the
//!   Advent of Code website.

pub mod date;
pub mod fs;
pub mod http;

pub use date::DateService;
pub use fs::FSService;
pub use http::HTTPService;

pub use crate::providers::date::DateAdapter;
pub use crate::providers::file_system::LocalFSAdapter;
pub use crate::providers::http::HTTPAdapter;
