use crate::models::{AnalysisReport, DecisionNode, Pattern};
use std::collections::HashMap;

/// Renders analysis reports as SVG diagrams
pub struct SvgRenderer {
    width: u32,
    height: u32,
    padding: u32,
}

impl SvgRenderer {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            padding: 40,
        }
    }

    pub fn with_dimensions(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            padding: 40,
        }
    }

    /// Render an analysis report to SVG string
    pub fn render(&self, report: &AnalysisReport) -> String {
        let mut svg = String::new();
        
        // SVG header
        svg.push_str(&format!(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?>
<svg width="{}" height="{}" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
<defs>
    <style type="text/css">
        .pattern-box {{ fill: #e3f2fd; stroke: #1976d2; stroke-width: 2; rx: 5; }}
        .decision-box {{ fill: #fff3e0; stroke: #f57c00; stroke-width: 2; rx: 5; }}
        .import-box {{ fill: #f3e5f5; stroke: #7b1fa2; stroke-width: 2; rx: 5; }}
        .high-confidence {{ fill: #c8e6c9; }}
        .medium-confidence {{ fill: #fff9c4; }}
        .low-confidence {{ fill: #ffccbc; }}
        .title {{ font-family: 'Arial', sans-serif; font-size: 18px; font-weight: bold; }}
        .label {{ font-family: 'Arial', sans-serif; font-size: 14px; }}
        .small {{ font-family: 'Arial', sans-serif; font-size: 12px; fill: #666; }}
        .confidence {{ font-family: 'Arial', sans-serif; font-size: 11px; font-weight: bold; }}
        .connector {{ stroke: #666; stroke-width: 2; fill: none; marker-end: url(#arrowhead); }}
    </style>
    <marker id="arrowhead" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto">
        <polygon points="0 0, 10 3, 0 6" fill="#666" />
    </marker>
</defs>
"#,
            self.width, self.height
        ));

        // Background
        svg.push_str(&format!(
            r#"<rect width="{}" height="{}" fill="#fafafa"/>"#,
            self.width, self.height
        ));

        // Title
        let filename = report.file_path.split('/').last().unwrap_or(&report.file_path);
        svg.push_str(&format!(
            r#"<text x="{}" y="30" class="title">Rust Pattern Analysis: {}</text>"#,
            self.padding, Self::escape_xml(filename)
        ));

        svg.push_str(&format!(
            r#"<text x="{}" y="50" class="small">Generated: {} | Confidence: {:.0}%</text>"#,
            self.padding, report.timestamp, report.overall_confidence * 100.0
        ));

        let mut y_offset = 80;

        // Render patterns section
        if !report.patterns.is_empty() {
            svg.push_str(&self.render_patterns_section(&report.patterns, self.padding, y_offset));
            y_offset += self.calculate_patterns_height(&report.patterns) + 30;
        }

        // Render decision nodes section
        if !report.decision_nodes.is_empty() {
            svg.push_str(&self.render_decisions_section(&report.decision_nodes, self.padding, y_offset));
            y_offset += self.calculate_decisions_height(&report.decision_nodes) + 30;
        }

        // Render imports section
        if !report.import_suggestions.is_empty() {
            svg.push_str(&self.render_imports_section(&report.import_suggestions, self.padding, y_offset));
        }

        // Calculate actual height needed
        let actual_height = y_offset + self.calculate_imports_height(&report.import_suggestions) + self.padding;
        
        svg.push_str("</svg>");

        // Update height in SVG if needed
        if actual_height > self.height {
            svg = svg.replace(
                &format!(r#"height="{}""#, self.height),
                &format!(r#"height="{}""#, actual_height)
            );
        }

        svg
    }

    fn render_patterns_section(&self, patterns: &[Pattern], x: u32, y: u32) -> String {
        let mut svg = String::new();
        
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" class="label">Detected Patterns ({})</text>"#,
            x, y, patterns.len()
        ));

        let mut current_y = y + 25;
        let box_width = (self.width - 2 * self.padding) as usize;

        for pattern in patterns {
            let confidence_class = Self::confidence_class(pattern.confidence);
            let box_height = self.calculate_pattern_box_height(pattern);

            // Pattern box
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" class="pattern-box {}"/>"#,
                x, current_y, box_width, box_height, confidence_class
            ));

            // Pattern type and line range
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" class="label">{} (lines {}-{})</text>"#,
                x + 10, current_y + 20,
                Self::escape_xml(&pattern.pattern_type),
                pattern.start_line, pattern.end_line
            ));

            // Confidence badge
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" class="confidence">{:.0}%</text>"#,
                x + box_width as u32 - 60, current_y + 20,
                pattern.confidence * 100.0
            ));

            // Reasoning if present
            if let Some(reasoning) = &pattern.reasoning {
                let wrapped_text = Self::wrap_text(reasoning, 80);
                let mut text_y = current_y + 40;
                for line in wrapped_text {
                    svg.push_str(&format!(
                        r#"<text x="{}" y="{}" class="small">{}</text>"#,
                        x + 10, text_y, Self::escape_xml(&line)
                    ));
                    text_y += 15;
                }
            }

            current_y += box_height + 15;
        }

        svg
    }

    fn render_decisions_section(&self, decisions: &[DecisionNode], x: u32, y: u32) -> String {
        let mut svg = String::new();
        
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" class="label">Decision Points ({})</text>"#,
            x, y, decisions.len()
        ));

        let mut current_y = y + 25;
        let box_width = (self.width - 2 * self.padding) as usize;

        for decision in decisions {
            let confidence_class = Self::confidence_class(decision.confidence);
            let box_height = 80;

            // Decision box
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" class="decision-box {}"/>"#,
                x, current_y, box_width, box_height, confidence_class
            ));

            // Decision type
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" class="label">{:?}</text>"#,
                x + 10, current_y + 20,
                decision.decision_type
            ));

            // Confidence
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" class="confidence">{:.0}%</text>"#,
                x + box_width as u32 - 60, current_y + 20,
                decision.confidence * 100.0
            ));

            // Description
            let wrapped_desc = Self::wrap_text(&decision.description, 80);
            let mut text_y = current_y + 40;
            for line in wrapped_desc.iter().take(2) {
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" class="small">{}</text>"#,
                    x + 10, text_y, Self::escape_xml(line)
                ));
                text_y += 15;
            }

            current_y += box_height + 15;
        }

        svg
    }

    fn render_imports_section(&self, imports: &[crate::models::Import], x: u32, y: u32) -> String {
        let mut svg = String::new();
        
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" class="label">Import Analysis ({})</text>"#,
            x, y, imports.len()
        ));

        let mut current_y = y + 25;
        let box_width = (self.width - 2 * self.padding) as usize;

        for import in imports.iter().take(5) { // Limit to 5 for space
            let box_height = 50;

            // Import box
            svg.push_str(&format!(
                r#"<rect x="{}" y="{}" width="{}" height="{}" class="import-box"/>"#,
                x, current_y, box_width, box_height
            ));

            // Import path
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" class="label">{}</text>"#,
                x + 10, current_y + 20,
                Self::escape_xml(&import.path)
            ));

            // Category
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" class="small">{}</text>"#,
                x + 10, current_y + 38,
                Self::escape_xml(&import.category)
            ));

            current_y += box_height + 10;
        }

        if imports.len() > 5 {
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" class="small">... and {} more imports</text>"#,
                x + 10, current_y + 10, imports.len() - 5
            ));
        }

        svg
    }

    fn calculate_patterns_height(&self, patterns: &[Pattern]) -> u32 {
        patterns.iter()
            .map(|p| self.calculate_pattern_box_height(p) + 15)
            .sum::<u32>()
            + 25
    }

    fn calculate_pattern_box_height(&self, pattern: &Pattern) -> u32 {
        let base_height = 40;
        let reasoning_lines = pattern.reasoning.as_ref()
            .map(|r| Self::wrap_text(r, 80).len() as u32)
            .unwrap_or(0);
        base_height + reasoning_lines * 15
    }

    fn calculate_decisions_height(&self, decisions: &[DecisionNode]) -> u32 {
        (decisions.len() as u32 * 95) + 25
    }

    fn calculate_imports_height(&self, imports: &[crate::models::Import]) -> u32 {
        let visible_imports = imports.len().min(5);
        (visible_imports as u32 * 60) + 35
    }

    fn confidence_class(confidence: f64) -> &'static str {
        if confidence >= 0.8 {
            "high-confidence"
        } else if confidence >= 0.5 {
            "medium-confidence"
        } else {
            "low-confidence"
        }
    }

    fn escape_xml(text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
    }

    fn wrap_text(text: &str, max_chars: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > max_chars {
                if !current_line.is_empty() {
                    lines.push(current_line.clone());
                    current_line.clear();
                }
            }
            if !current_line.is_empty() {
                current_line.push(' ');
            }
            current_line.push_str(word);
        }

        if !current_line.is_empty() {
            lines.push(current_line);
        }

        lines
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::*;

    #[test]
    fn test_render_empty_report() {
        let report = AnalysisReport {
            file_path: "test.rs".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            patterns: vec![],
            import_suggestions: vec![],
            decision_nodes: vec![],
            overall_confidence: 0.0,
            metadata: ReportMetadata {
                analyzer_version: "1.0.0".to_string(),
                total_lines: 0,
                analysis_duration_ms: 0,
            },
        };

        let renderer = SvgRenderer::new();
        let svg = renderer.render(&report);

        assert!(svg.contains("<?xml"));
        assert!(svg.contains("test.rs"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn test_render_with_patterns() {
        let report = AnalysisReport {
            file_path: "test.rs".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            patterns: vec![
                Pattern {
                    pattern_type: "Error Handling".to_string(),
                    start_line: 10,
                    end_line: 15,
                    confidence: 0.9,
                    reasoning: Some("Uses Result<T,E> pattern".to_string()),
                    code_snippet: "".to_string(),
                }
            ],
            import_suggestions: vec![],
            decision_nodes: vec![],
            overall_confidence: 0.9,
            metadata: ReportMetadata {
                analyzer_version: "1.0.0".to_string(),
                total_lines: 100,
                analysis_duration_ms: 50,
            },
        };

        let renderer = SvgRenderer::new();
        let svg = renderer.render(&report);

        assert!(svg.contains("Error Handling"));
        assert!(svg.contains("90%"));
        assert!(svg.contains("high-confidence"));
    }

    #[test]
    fn test_xml_escaping() {
        assert_eq!(
            SvgRenderer::escape_xml("<script>alert('xss')</script>"),
            "&lt;script&gt;alert(&apos;xss&apos;)&lt;/script&gt;"
        );
    }

    #[test]
    fn test_text_wrapping() {
        let text = "This is a very long line that should be wrapped into multiple shorter lines";
        let wrapped = SvgRenderer::wrap_text(text, 20);
        
        assert!(wrapped.len() > 1);
        for line in wrapped {
            assert!(line.len() <= 25); // Some tolerance for word boundaries
        }
    }
}
