//! Rust Pattern Visualizer - Core Library
//!
//! This library provides tools for analyzing Rust code patterns and decision trees.

pub mod analyzer;
pub mod models;
pub mod share;
pub mod web_server;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub use analyzer::CodeAnalyzer;
pub use models::{AnalysisReport, DecisionNode, Pattern};
pub use share::{ShareService, SharedAnalysis};
pub use web_server::ShareServer;
