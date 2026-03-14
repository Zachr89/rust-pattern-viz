//! Rust Pattern Viz - Core Library
//! 
//! This library provides Rust code analysis and pattern detection capabilities.
//! It can be used as a library, CLI tool, LSP server, or compiled to WebAssembly.

pub mod analyzer;
pub mod models;

pub use analyzer::CodeAnalyzer;
pub use models::{AnalysisReport, Pattern, DecisionNode, Import, Alternative, DecisionType};

#[cfg(feature = "wasm")]
pub mod wasm;

#[cfg(feature = "wasm")]
pub use wasm::*;
