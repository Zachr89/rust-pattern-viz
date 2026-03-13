use crate::models::*;
use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use syn::{visit::Visit, File, Item, UseTree};

pub struct CodeAnalyzer {
    version: String,
}

impl CodeAnalyzer {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    pub fn analyze(&self, source: &str, file_path: &str) -> Result<AnalysisReport> {
        let syntax_tree = syn::parse_file(source).context("Failed to parse Rust source")?;

        let mut visitor = PatternVisitor::new(source);
        visitor.visit_file(&syntax_tree);

        let patterns = visitor.patterns;
        let import_suggestions = self.analyze_imports(&syntax_tree);
        let decision_nodes = self.build_decision_tree(&patterns, &import_suggestions);

        let overall_confidence = if patterns.is_empty() {
            0.0
        } else {
            patterns.iter().map(|p| p.confidence).sum::<f64>() / patterns.len() as f64
        };

        let lines: Vec<&str> = source.lines().collect();
        let complexity_score = self.calculate_complexity(&syntax_tree);

        Ok(AnalysisReport {
            file_path: file_path.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            patterns,
            import_suggestions,
            decision_nodes,
            overall_confidence,
            metadata: ReportMetadata {
                analyzer_version: self.version.clone(),
                rust_edition: Self::detect_edition(source),
                total_lines: lines.len(),
                complexity_score,
            },
        })
    }

    fn analyze_imports(&self, syntax_tree: &File) -> Vec<ImportSuggestion> {
        let mut suggestions = Vec::new();

        for item in &syntax_tree.items {
            if let Item::Use(use_item) = item {
                self.process_use_tree(&use_item.tree, &mut suggestions);
            }
        }

        // Add synthetic AI reasoning for common patterns
        self.add_synthetic_suggestions(&mut suggestions);

        suggestions
    }

    fn process_use_tree(&self, tree: &UseTree, suggestions: &mut Vec<ImportSuggestion>) {
        match tree {
            UseTree::Path(path) => {
                let import_path = format!("{}", quote::quote!(#path));
                let confidence = self.estimate_import_confidence(&import_path);

                suggestions.push(ImportSuggestion {
                    import_path: import_path.clone(),
                    status: ImportStatus::Selected,
                    reason: Some(self.generate_import_reason(&import_path)),
                    alternatives: self.suggest_alternatives(&import_path),
                    confidence,
                });

                self.process_use_tree(&path.tree, suggestions);
            }
            UseTree::Name(name) => {
                let import_path = format!("{}", name.ident);
                suggestions.push(ImportSuggestion {
                    import_path,
                    status: ImportStatus::Selected,
                    reason: None,
                    alternatives: vec![],
                    confidence: 0.9,
                });
            }
            UseTree::Group(group) => {
                for tree in &group.items {
                    self.process_use_tree(tree, suggestions);
                }
            }
            _ => {}
        }
    }

    fn add_synthetic_suggestions(&self, suggestions: &mut Vec<ImportSuggestion>) {
        // Simulate AI considering but rejecting alternatives
        let common_alternatives = vec![
            ("std::collections::HashMap", vec!["hashbrown::HashMap", "indexmap::IndexMap"], "Standard library HashMap chosen for compatibility"),
            ("serde::Serialize", vec!["bincode::Encode", "rkyv::Archive"], "Serde chosen for ecosystem compatibility"),
            ("anyhow::Result", vec!["eyre::Result", "thiserror::Error"], "Anyhow provides better ergonomics for application code"),
        ];

        for (selected, alternatives, reason) in common_alternatives {
            if suggestions.iter().any(|s| s.import_path.contains(selected)) {
                for alt in alternatives {
                    suggestions.push(ImportSuggestion {
                        import_path: alt.to_string(),
                        status: ImportStatus::Rejected,
                        reason: Some(reason.to_string()),
                        alternatives: vec![selected.to_string()],
                        confidence: 0.4,
                    });
                }
            }
        }
    }

    fn estimate_import_confidence(&self, import_path: &str) -> f64 {
        // Heuristics for confidence scoring
        if import_path.starts_with("std::") {
            0.95
        } else if import_path.contains("::") {
            0.85
        } else {
            0.75
        }
    }

    fn generate_import_reason(&self, import_path: &str) -> String {
        if import_path.contains("std::") {
            "Standard library - guaranteed availability".to_string()
        } else if import_path.contains("serde") {
            "De-facto serialization standard".to_string()
        } else if import_path.contains("tokio") || import_path.contains("async") {
            "Async runtime required for concurrent operations".to_string()
        } else {
            "Common dependency in Rust ecosystem".to_string()
        }
    }

    fn suggest_alternatives(&self, import_path: &str) -> Vec<String> {
        let mut alternatives = Vec::new();

        if import_path.contains("HashMap") {
            alternatives.push("BTreeMap for ordered keys".to_string());
            alternatives.push("hashbrown::HashMap for performance".to_string());
        } else if import_path.contains("Vec") {
            alternatives.push("SmallVec for stack optimization".to_string());
            alternatives.push("ArrayVec for fixed capacity".to_string());
        }

        alternatives
    }

    fn build_decision_tree(
        &self,
        patterns: &[PatternMatch],
        imports: &[ImportSuggestion],
    ) -> Vec<DecisionNode> {
        let mut nodes = Vec::new();

        // Root decision: parsing strategy
        nodes.push(DecisionNode {
            id: "root".to_string(),
            description: "Code structure analysis initiated".to_string(),
            confidence: 1.0,
            children: vec!["imports".to_string(), "patterns".to_string()],
            metadata: None,
        });

        // Import analysis branch
        nodes.push(DecisionNode {
            id: "imports".to_string(),
            description: format!("Analyzed {} import statements", imports.len()),
            confidence: 0.9,
            children: imports
                .iter()
                .enumerate()
                .map(|(i, _)| format!("import_{}", i))
                .collect(),
            metadata: None,
        });

        // Pattern matching branch
        nodes.push(DecisionNode {
            id: "patterns".to_string(),
            description: format!("Identified {} code patterns", patterns.len()),
            confidence: 0.85,
            children: patterns
                .iter()
                .enumerate()
                .map(|(i, _)| format!("pattern_{}", i))
                .collect(),
            metadata: None,
        });

        // Individual pattern nodes
        for (i, pattern) in patterns.iter().enumerate() {
            nodes.push(DecisionNode {
                id: format!("pattern_{}", i),
                description: format!("{} at line {}", pattern.pattern_type, pattern.line_number),
                confidence: pattern.confidence,
                children: vec![],
                metadata: Some(serde_json::json!({
                    "reasoning": pattern.reasoning,
                    "alternatives": pattern.alternatives_considered,
                })),
            });
        }

        nodes
    }

    fn calculate_complexity(&self, syntax_tree: &File) -> f64 {
        let mut complexity = 0.0;

        for item in &syntax_tree.items {
            complexity += match item {
                Item::Fn(_) => 1.0,
                Item::Struct(_) => 0.5,
                Item::Enum(_) => 0.7,
                Item::Impl(_) => 1.2,
                Item::Trait(_) => 1.5,
                _ => 0.3,
            };
        }

        complexity
    }

    fn detect_edition(source: &str) -> Option<String> {
        if source.contains("edition = \"2021\"") {
            Some("2021".to_string())
        } else if source.contains("edition = \"2018\"") {
            Some("2018".to_string())
        } else {
            None
        }
    }
}

impl Default for CodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

struct PatternVisitor<'a> {
    source: &'a str,
    patterns: Vec<PatternMatch>,
    lines: Vec<&'a str>,
}

impl<'a> PatternVisitor<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            patterns: Vec::new(),
            lines: source.lines().collect(),
        }
    }

    fn get_line_number(&self, span: proc_macro2::Span) -> usize {
        span.start().line
    }

    fn get_code_snippet(&self, line_num: usize) -> String {
        self.lines
            .get(line_num.saturating_sub(1))
            .unwrap_or(&"")
            .trim()
            .to_string()
    }

    fn calculate_confidence(&self, pattern_type: &str) -> f64 {
        // Simulate AI confidence based on pattern complexity
        match pattern_type {
            t if t.contains("Error Handling") => 0.92,
            t if t.contains("Iterator Chain") => 0.88,
            t if t.contains("Option Handling") => 0.95,
            t if t.contains("Async") => 0.78,
            t if t.contains("Trait Implementation") => 0.85,
            _ => 0.75,
        }
    }
}

impl<'a> Visit<'a> for PatternVisitor<'a> {
    fn visit_item_fn(&mut self, node: &'a syn::ItemFn) {
        let line_number = self.get_line_number(node.sig.ident.span());
        let pattern_type = if node.sig.asyncness.is_some() {
            "Async Function Pattern"
        } else {
            "Function Definition"
        };

        let confidence = self.calculate_confidence(pattern_type);

        self.patterns.push(PatternMatch {
            pattern_type: pattern_type.to_string(),
            line_number,
            code_snippet: self.get_code_snippet(line_number),
            confidence,
            reasoning: format!(
                "Detected {} with {} parameters",
                if node.sig.asyncness.is_some() {
                    "async function"
                } else {
                    "function"
                },
                node.sig.inputs.len()
            ),
            alternatives_considered: vec![
                "Closure implementation".to_string(),
                "Macro-generated function".to_string(),
            ],
        });

        syn::visit::visit_item_fn(self, node);
    }

    fn visit_expr_match(&mut self, node: &'a syn::ExprMatch) {
        let line_number = self.get_line_number(node.match_token.span);
        let confidence = self.calculate_confidence("Pattern Matching");

        self.patterns.push(PatternMatch {
            pattern_type: "Pattern Matching".to_string(),
            line_number,
            code_snippet: self.get_code_snippet(line_number),
            confidence,
            reasoning: format!("Match expression with {} arms detected", node.arms.len()),
            alternatives_considered: vec![
                "If-let chain".to_string(),
                "Separate functions".to_string(),
            ],
        });

        syn::visit::visit_expr_match(self, node);
    }

    fn visit_expr_try(&mut self, node: &'a syn::ExprTry) {
        let line_number = self.get_line_number(node.question_token.span);
        let confidence = self.calculate_confidence("Error Handling");

        self.patterns.push(PatternMatch {
            pattern_type: "Error Handling Pattern (?)".to_string(),
            line_number,
            code_snippet: self.get_code_snippet(line_number),
            confidence,
            reasoning: "Question mark operator for error propagation".to_string(),
            alternatives_considered: vec![
                "Explicit match on Result".to_string(),
                "unwrap() with comment".to_string(),
            ],
        });

        syn::visit::visit_expr_try(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'a syn::ExprMethodCall) {
        let method_name = node.method.to_string();

        // Detect iterator chains
        if ["map", "filter", "fold", "collect", "flat_map"]
            .contains(&method_name.as_str())
        {
            let line_number = self.get_line_number(node.method.span());
            let confidence = self.calculate_confidence("Iterator Chain");

            self.patterns.push(PatternMatch {
                pattern_type: "Iterator Chain Pattern".to_string(),
                line_number,
                code_snippet: self.get_code_snippet(line_number),
                confidence,
                reasoning: format!("Functional iterator method '{}' detected", method_name),
                alternatives_considered: vec![
                    "Imperative loop".to_string(),
                    "Parallel iterator (rayon)".to_string(),
                ],
            });
        }

        syn::visit::visit_expr_method_call(self, node);
    }
}
