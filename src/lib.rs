//! # riops
//!
//! **R**ust **I**nput/**O**utput **P**arallel **S**earching — a command-line grep utility
//! that leverages multi-threading via [`rayon`] to search for matches across files.
//!
//! ## Features
//!
//! - Single-file or recursive directory search
//! - Case-insensitive matching (`--ignore-case`)
//! - Whole-word matching (`--whole-word`)
//! - File extension filtering (`--extension`)
//! - Parallel directory search powered by Rayon
//! - Summary output mode (`--simple-search`)
//!
//! ## Usage
//!
//! ```bash
//! # Search a single file
//! rps --query "hello" --file-path ./notes.txt
//!
//! # Recursively search a directory (parallel, .txt files by default)
//! rps --query "hello" --directory ./docs
//!
//! # Search only Rust source files
//! rps -q "fn main" -d ./src -e rs
//!
//! # Case-insensitive whole-word match with summary output
//! rps -q "hello" -d ./docs --ignore-case --whole-word --simple-search
//! ```

/// Data models for search parameters and match results.
pub mod models;

/// Core search logic for single files and parallel directory traversal.
pub mod search_engine;
