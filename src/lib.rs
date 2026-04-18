//! # riops
//!
//! **R**ust **I**nput/**O**utput **P**arallel **S**earching — a command-line grep utility
//! that leverages multi-threading via [`rayon`] to search for matches across files.
//!
//! ## Features
//!
//! - Single-file or recursive directory search (`.txt` files)
//! - Case-insensitive matching (`--ignore-case`)
//! - Whole-word matching (`--whole-word`)
//! - Parallel directory search powered by Rayon
//! - Summary output mode (`--simple-search`)
//!
//! ## Usage
//!
//! ```bash
//! # Search a single file
//! riops --query "hello" --file-path ./notes.txt
//!
//! # Recursively search a directory (parallel)
//! riops --query "hello" --directory ./docs
//!
//! # Case-insensitive whole-word match with summary output
//! riops -q "hello" -d ./docs --ignore-case --whole-word --simple-search
//! ```

/// Data models for search parameters and match results.
pub mod models;

/// Core search logic for single files and parallel directory traversal.
pub mod search_engine;