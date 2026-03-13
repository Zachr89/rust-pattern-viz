use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub file_path: String,
    pub timestamp: String,
    pub patterns: Vec<PatternMatch>,
    pub import_suggestions: Vec<ImportSuggestion>,
    pub decision_nodes: Vec<DecisionNode>,
    pub overall_confidence: f64,
    pub metadata: ReportMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMatch {
    pub pattern_type: String,
    pub line_number: usize,
    pub code_snippet: String,
    pub confidence: f64,
    pub reasoning: String,
    pub alternatives_considered: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSuggestion {
    pub import_path: String,
    pub status: ImportStatus,
    pub reason: Option<String>,
    pub alternatives: Vec<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImportStatus {
    Selected,
    Rejected,
    Considered,
}

impl std::fmt::Display for ImportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ImportStatus::Selected => write!(f, "✓ Selected"),
            ImportStatus::Rejected => write!(f, "✗ Rejected"),
            ImportStatus::Considered => write!(f, "○ Considered"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionNode {
    pub id: String,
    pub description: String,
    pub confidence: f64,
    pub children: Vec<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub analyzer_version: String,
    pub rust_edition: Option<String>,
    pub total_lines: usize,
    pub complexity_score: f64,
}

#[derive(Debug, Clone)]
pub enum VisualizationFormat {
    Json,
    Html,
    Svg,
}
