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
    pub module: String,
    pub items: Vec<String>,
    pub reasoning: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionNode {
    pub id: String,
    pub decision_type: DecisionType,
    pub description: String,
    pub alternatives: Vec<Alternative>,
    pub chosen: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionType {
    ImportChoice,
    PatternSelection,
    ErrorHandling,
    TypeInference,
    LifetimeAnnotation,
    TraitBound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alternative {
    pub name: String,
    pub description: String,
    pub score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    pub analyzer_version: String,
    pub rust_edition: String,
    pub analysis_duration_ms: u64,
}

impl AnalysisReport {
    pub fn new(file_path: String) -> Self {
        Self {
            file_path,
            timestamp: chrono::Utc::now().to_rfc3339(),
            patterns: Vec::new(),
            import_suggestions: Vec::new(),
            decision_nodes: Vec::new(),
            overall_confidence: 0.0,
            metadata: ReportMetadata {
                analyzer_version: env!("CARGO_PKG_VERSION").to_string(),
                rust_edition: "2021".to_string(),
                analysis_duration_ms: 0,
            },
        }
    }
}
