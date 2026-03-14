use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub file_path: String,
    pub timestamp: String,
    pub patterns: Vec<Pattern>,
    pub import_suggestions: Vec<Import>,
    pub decision_nodes: Vec<DecisionNode>,
    pub overall_confidence: f64,
    pub metadata: ReportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub pattern_type: String,
    pub start_line: usize,
    pub end_line: usize,
    pub confidence: f64,
    pub reasoning: Option<String>,
    pub code_snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub category: String,
    pub confidence: f64,
    pub reasoning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionNode {
    pub id: String,
    pub decision_type: DecisionType,
    pub description: String,
    pub alternatives: Vec<String>,
    pub chosen: String,
    pub confidence: f64,
    pub reasoning: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DecisionType {
    ImportChoice,
    PatternSelection,
    ErrorHandling,
    TypeInference,
    ControlFlow,  // New: for if let / while let patterns
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub analyzer_version: String,
    pub rust_version: String,
}
