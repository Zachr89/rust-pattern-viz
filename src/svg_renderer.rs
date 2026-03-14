use crate::models::{AnalysisReport, DecisionNode, DecisionType, Pattern};

pub struct SvgRenderer {
    width: u32,
    height: u32,
}

impl SvgRenderer {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
        }
    }

    pub fn render(&self, report: &AnalysisReport) -> String {
        let mut svg = String::new();
        let mut y_offset = 20;

        // Calculate dynamic height based on content
        let patterns_height = report.patterns.len() * 120;
        let decisions_height = report.decision_nodes.len() * 180;
        let imports_height = report.import_suggestions.len() * 60;
        let total_height = 100 + patterns_height + decisions_height + imports_height + 100;
        let actual_height = total_height.max(self.height as usize);

        // SVG header
        svg.push_str(&format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" width="{}" height="{}" viewBox="0 0 {} {}">"#,
            self.width, actual_height, self.width, actual_height
        ));

        // Background
        svg.push_str(&format!(
            r#"<rect width="{}" height="{}" fill="#ffffff"/>"#,
            self.width, actual_height
        ));

        // Title
        svg.push_str(&format!(
            r#"<text x="20" y="{}" font-family="monospace" font-size="24" font-weight="bold" fill="#333">Pattern Analysis: {}</text>"#,
            y_offset + 30,
            self.escape_xml(&report.file_path)
        ));
        y_offset += 60;

        // Metadata
        svg.push_str(&format!(
            r#"<text x="20" y="{}" font-family="monospace" font-size="12" fill="#666">Confidence: {:.1}% | Patterns: {} | Time: {}</text>"#,
            y_offset,
            report.overall_confidence * 100.0,
            report.patterns.len(),
            &report.timestamp[..19]
        ));
        y_offset += 40;

        // Patterns section
        if !report.patterns.is_empty() {
            svg.push_str(&format!(
                r#"<text x="20" y="{}" font-family="monospace" font-size="18" font-weight="bold" fill="#1976d2">Detected Patterns</text>"#,
                y_offset
            ));
            y_offset += 30;

            for pattern in &report.patterns {
                let box_height = self.render_pattern(&mut svg, pattern, 20, y_offset);
                y_offset += box_height + 20;
            }

            y_offset += 20;
        }

        // Decision nodes section with flow diagrams
        if !report.decision_nodes.is_empty() {
            svg.push_str(&format!(
                r#"<text x="20" y="{}" font-family="monospace" font-size="18" font-weight="bold" fill="#f57c00">Decision Flow</text>"#,
                y_offset
            ));
            y_offset += 30;

            for node in &report.decision_nodes {
                let box_height = if node.decision_type == DecisionType::ControlFlow {
                    self.render_control_flow_diagram(&mut svg, node, 20, y_offset)
                } else {
                    self.render_decision_node(&mut svg, node, 20, y_offset)
                };
                y_offset += box_height + 30;
            }

            y_offset += 20;
        }

        // Imports section
        if !report.import_suggestions.is_empty() {
            svg.push_str(&format!(
                r#"<text x="20" y="{}" font-family="monospace" font-size="18" font-weight="bold" fill="#7b1fa2">Imports</text>"#,
                y_offset
            ));
            y_offset += 30;

            for import in &report.import_suggestions {
                self.render_import(&mut svg, import, 20, y_offset);
                y_offset += 70;
            }
        }

        svg.push_str("</svg>");
        svg
    }

    fn render_pattern(&self, svg: &mut String, pattern: &Pattern, x: u32, y: usize) -> usize {
        let confidence_color = self.confidence_color(pattern.confidence);
        let reasoning_lines = pattern.reasoning.as_ref().map_or(0, |r| (r.len() / 80) + 1);
        let box_height = 80 + reasoning_lines * 15;

        // Pattern box
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="760" height="{}" fill="{}" stroke="#1976d2" stroke-width="2" rx="5"/>"#,
            x, y, box_height, confidence_color
        ));

        // Pattern type
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="14" font-weight="bold" fill="#1976d2">{}</text>"#,
            x + 10,
            y + 20,
            self.escape_xml(&pattern.pattern_type)
        ));

        // Line range and confidence
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="12" fill="#666">Lines {}-{} | Confidence: {:.0}%</text>"#,
            x + 10,
            y + 40,
            pattern.start_line,
            pattern.end_line,
            pattern.confidence * 100.0
        ));

        // Reasoning
        if let Some(reasoning) = &pattern.reasoning {
            let wrapped = self.wrap_text(reasoning, 80);
            for (i, line) in wrapped.iter().enumerate() {
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" font-family="monospace" font-size="11" fill="#444">{}</text>"#,
                    x + 10,
                    y + 60 + (i * 15),
                    self.escape_xml(line)
                ));
            }
        }

        box_height
    }

    fn render_decision_node(&self, svg: &mut String, node: &DecisionNode, x: u32, y: usize) -> usize {
        let confidence_color = self.confidence_color(node.confidence);

        // Decision box
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="760" height="140" fill="{}" stroke="#f57c00" stroke-width="2" rx="5"/>"#,
            x, y, confidence_color
        ));

        // Decision type badge
        let type_str = format!("{:?}", node.decision_type);
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="120" height="20" fill="#f57c00" rx="3"/>"#,
            x + 10,
            y + 10
        ));
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="10" font-weight="bold" fill="white">{}</text>"#,
            x + 15,
            y + 23,
            type_str
        ));

        // Description
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="13" font-weight="bold" fill="#333">{}</text>"#,
            x + 10,
            y + 50,
            self.escape_xml(&node.description)
        ));

        // Chosen option
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="12" fill="#2e7d32">✓ Chosen: {}</text>"#,
            x + 10,
            y + 75,
            self.escape_xml(&node.chosen)
        ));

        // Alternatives
        if !node.alternatives.is_empty() {
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="monospace" font-size="11" fill="#666">Alternatives: {}</text>"#,
                x + 10,
                y + 95,
                self.escape_xml(&node.alternatives.join(", "))
            ));
        }

        // Reasoning
        if let Some(reasoning) = &node.reasoning {
            let wrapped = self.wrap_text(reasoning, 80);
            for (i, line) in wrapped.iter().take(1).enumerate() {
                svg.push_str(&format!(
                    r#"<text x="{}" y="{}" font-family="monospace" font-size="10" fill="#555">{}</text>"#,
                    x + 10,
                    y + 115 + (i * 15),
                    self.escape_xml(line)
                ));
            }
        }

        140
    }

    fn render_control_flow_diagram(&self, svg: &mut String, node: &DecisionNode, x: u32, y: usize) -> usize {
        // This renders if let / while let as a flow diagram with branches
        let is_while = node.description.contains("while let");
        let box_height = 160;
        
        let confidence_color = self.confidence_color(node.confidence);

        // Main container
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="760" height="{}" fill="{}" stroke="#f57c00" stroke-width="2" rx="5"/>"#,
            x, y, box_height, confidence_color
        ));

        // Type badge
        let type_label = if is_while { "WHILE LET" } else { "IF LET" };
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="100" height="20" fill="#f57c00" rx="3"/>"#,
            x + 10,
            y + 10
        ));
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="10" font-weight="bold" fill="white">{}</text>"#,
            x + 15,
            y + 23,
            type_label
        ));

        // Description
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="13" font-weight="bold" fill="#333">{}</text>"#,
            x + 10,
            y + 50,
            self.escape_xml(&node.description)
        ));

        // Flow diagram elements
        let center_x = x + 380;
        let flow_y = y + 75;

        // Condition diamond
        svg.push_str(&format!(
            r#"<polygon points="{},{} {},{} {},{} {},{}" fill="#fff3e0" stroke="#f57c00" stroke-width="2"/>"#,
            center_x, flow_y,           // top
            center_x + 50, flow_y + 25, // right
            center_x, flow_y + 50,      // bottom
            center_x - 50, flow_y + 25  // left
        ));
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="11" text-anchor="middle" fill="#333">Pattern</text>"#,
            center_x,
            flow_y + 25
        ));
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="11" text-anchor="middle" fill="#333">Match?</text>"#,
            center_x,
            flow_y + 38
        ));

        // Success branch (right)
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#2e7d32" stroke-width="2" marker-end="url(#arrowgreen)"/>"#,
            center_x + 50, flow_y + 25,
            center_x + 150, flow_y + 25
        ));
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="10" fill="#2e7d32">Yes</text>"#,
            center_x + 80,
            flow_y + 20
        ));
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="120" height="40" fill="#c8e6c9" stroke="#2e7d32" stroke-width="2" rx="3"/>"#,
            center_x + 150, flow_y + 5
        ));
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="11" text-anchor="middle" fill="#2e7d32">Execute</text>"#,
            center_x + 210,
            flow_y + 30
        ));

        // Failure branch (left)
        svg.push_str(&format!(
            r#"<line x1="{}" y1="{}" x2="{}" y2="{}" stroke="#c62828" stroke-width="2" marker-end="url(#arrowred)"/>"#,
            center_x - 50, flow_y + 25,
            center_x - 150, flow_y + 25
        ));
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="10" fill="#c62828">No</text>"#,
            center_x - 90,
            flow_y + 20
        ));
        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="120" height="40" fill="#ffcdd2" stroke="#c62828" stroke-width="2" rx="3"/>"#,
            center_x - 270, flow_y + 5
        ));
        
        let failure_text = if is_while { "Exit Loop" } else { "Skip/Else" };
        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="11" text-anchor="middle" fill="#c62828">{}</text>"#,
            center_x - 210,
            flow_y + 30,
            failure_text
        ));

        // Loop back arrow for while let
        if is_while {
            svg.push_str(&format!(
                r#"<path d="M {} {} Q {} {} {} {}" fill="none" stroke="#2e7d32" stroke-width="2" stroke-dasharray="5,5" marker-end="url(#arrowgreen)"/>"#,
                center_x + 270, flow_y + 25,
                center_x + 300, flow_y - 20,
                center_x, flow_y - 20
            ));
            svg.push_str(&format!(
                r#"<text x="{}" y="{}" font-family="monospace" font-size="9" fill="#2e7d32">Continue</text>"#,
                center_x + 280,
                flow_y - 25
            ));
        }

        // Arrow markers (defined once at SVG level, but included here for completeness)
        if !svg.contains("id=\"arrowgreen\"") {
            svg.push_str(r#"<defs>
                <marker id="arrowgreen" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto" markerUnits="strokeWidth">
                    <path d="M0,0 L0,6 L9,3 z" fill="#2e7d32" />
                </marker>
                <marker id="arrowred" markerWidth="10" markerHeight="10" refX="9" refY="3" orient="auto" markerUnits="strokeWidth">
                    <path d="M0,0 L0,6 L9,3 z" fill="#c62828" />
                </marker>
            </defs>"#);
        }

        box_height
    }

    fn render_import(&self, svg: &mut String, import: &crate::models::Import, x: u32, y: usize) {
        let confidence_color = self.confidence_color(import.confidence);

        svg.push_str(&format!(
            r#"<rect x="{}" y="{}" width="760" height="50" fill="{}" stroke="#7b1fa2" stroke-width="2" rx="5"/>"#,
            x, y, confidence_color
        ));

        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="12" font-weight="bold" fill="#7b1fa2">{}</text>"#,
            x + 10,
            y + 20,
            self.escape_xml(&import.category)
        ));

        svg.push_str(&format!(
            r#"<text x="{}" y="{}" font-family="monospace" font-size="11" fill="#333">{}</text>"#,
            x + 10,
            y + 38,
            self.escape_xml(&import.path)
        ));
    }

    fn confidence_color(&self, confidence: f64) -> &'static str {
        if confidence >= 0.8 {
            "#c8e6c9" // Light green
        } else if confidence >= 0.5 {
            "#fff9c4" // Light yellow
        } else {
            "#ffccbc" // Light orange
        }
    }

    fn wrap_text(&self, text: &str, max_chars: usize) -> Vec<String> {
        let mut lines = Vec::new();
        let mut current_line = String::new();

        for word in text.split_whitespace() {
            if current_line.len() + word.len() + 1 > max_chars {
                if !current_line.is_empty() {
                    lines.push(current_line);
                    current_line = String::new();
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

    fn escape_xml(&self, text: &str) -> String {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&apos;")
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
    fn test_svg_rendering_with_control_flow() {
        let report = AnalysisReport {
            file_path: "test.rs".to_string(),
            timestamp: "2024-01-01T00:00:00Z".to_string(),
            patterns: vec![],
            import_suggestions: vec![],
            decision_nodes: vec![
                DecisionNode {
                    id: "node_1".to_string(),
                    decision_type: DecisionType::ControlFlow,
                    description: "if let Some(x) = option".to_string(),
                    alternatives: vec!["match expression".to_string()],
                    chosen: "if let".to_string(),
                    confidence: 0.85,
                    reasoning: Some("Concise for single variant".to_string()),
                },
            ],
            overall_confidence: 0.85,
            metadata: ReportMetadata {
                analyzer_version: "1.0.0".to_string(),
                rust_version: "1.70.0".to_string(),
            },
        };

        let renderer = SvgRenderer::new();
        let svg = renderer.render(&report);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("if let Some(x) = option"));
        assert!(svg.contains("Pattern"));
        assert!(svg.contains("Match?"));
    }
}
