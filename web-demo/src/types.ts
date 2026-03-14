export interface AnalysisReport {
  file_path: string;
  timestamp: string;
  patterns: Pattern[];
  import_suggestions: Import[];
  decision_nodes: DecisionNode[];
  overall_confidence: number;
  metadata: ReportMetadata;
}

export interface Pattern {
  pattern_type: string;
  start_line: number;
  end_line: number;
  confidence: number;
  reasoning?: string;
  code_snippet: string;
}

export interface Import {
  module: string;
  items: string[];
  reasoning: string;
  confidence: number;
}

export interface DecisionNode {
  id: string;
  decision_type: string;
  description: string;
  alternatives: Alternative[];
  chosen: string;
  confidence: number;
}

export interface Alternative {
  name: string;
  description: string;
  score: number;
}

export interface ReportMetadata {
  analyzer_version: string;
  rust_edition: string;
  analysis_duration_ms: number;
}
