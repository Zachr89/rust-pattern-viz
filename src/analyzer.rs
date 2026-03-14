use crate::models::*;
use anyhow::{Context, Result};
use syn::visit::{self, Visit};
use syn::{File, Item, ItemFn, ItemImpl, ItemStruct, ItemUse};

pub struct CodeAnalyzer;

impl CodeAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, source: &str, file_path: &str) -> Result<AnalysisReport> {
        let start = std::time::Instant::now();
        
        let syntax = syn::parse_file(source)
            .context("Failed to parse Rust source code")?;
        
        let mut visitor = PatternVisitor::new();
        visitor.visit_file(&syntax);
        
        let mut report = AnalysisReport::new(file_path.to_string());
        report.patterns = visitor.patterns;
        report.import_suggestions = visitor.imports;
        report.decision_nodes = visitor.decisions;
        
        // Calculate overall confidence
        if !report.patterns.is_empty() {
            report.overall_confidence = report.patterns.iter()
                .map(|p| p.confidence)
                .sum::<f64>() / report.patterns.len() as f64;
        }
        
        report.metadata.analysis_duration_ms = start.elapsed().as_millis() as u64;
        
        Ok(report)
    }
}

impl Default for CodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

struct PatternVisitor {
    patterns: Vec<Pattern>,
    imports: Vec<Import>,
    decisions: Vec<DecisionNode>,
    current_line: usize,
}

impl PatternVisitor {
    fn new() -> Self {
        Self {
            patterns: Vec::new(),
            imports: Vec::new(),
            decisions: Vec::new(),
            current_line: 1,
        }
    }

    fn detect_error_handling(&mut self, func: &ItemFn) {
        let func_str = quote::quote!(#func).to_string();
        
        if func_str.contains("Result<") {
            let start = func.sig.span().start().line;
            let end = func.block.span().end().line;
            
            self.patterns.push(Pattern {
                pattern_type: "Error Handling".to_string(),
                start_line: start,
                end_line: end,
                confidence: 0.85,
                reasoning: Some("Function returns Result type, implementing standard Rust error handling".to_string()),
                code_snippet: format!("fn {}(...) -> Result<...>", func.sig.ident),
            });
            
            self.decisions.push(DecisionNode {
                id: format!("decision_{}", self.decisions.len() + 1),
                decision_type: DecisionType::ErrorHandling,
                description: format!("Error handling strategy for function '{}'", func.sig.ident),
                alternatives: vec![
                    Alternative {
                        name: "Result<T, E>".to_string(),
                        description: "Standard Rust error handling with Result type".to_string(),
                        score: 0.9,
                    },
                    Alternative {
                        name: "Option<T>".to_string(),
                        description: "Simple success/failure with Option type".to_string(),
                        score: 0.6,
                    },
                    Alternative {
                        name: "Panic".to_string(),
                        description: "Unrecoverable error via panic!()".to_string(),
                        score: 0.3,
                    },
                ],
                chosen: "Result<T, E>".to_string(),
                confidence: 0.85,
            });
        }
        
        if func_str.contains("?") {
            self.patterns.push(Pattern {
                pattern_type: "Question Mark Operator".to_string(),
                start_line: func.sig.span().start().line,
                end_line: func.block.span().end().line,
                confidence: 0.9,
                reasoning: Some("Uses ? operator for clean error propagation".to_string()),
                code_snippet: "Uses ? for error propagation".to_string(),
            });
        }
    }

    fn detect_iterator_patterns(&mut self, func: &ItemFn) {
        let func_str = quote::quote!(#func).to_string();
        
        let iterator_methods = ["map", "filter", "fold", "collect", "for_each"];
        let found_methods: Vec<_> = iterator_methods.iter()
            .filter(|&&method| func_str.contains(method))
            .collect();
        
        if !found_methods.is_empty() {
            self.patterns.push(Pattern {
                pattern_type: "Iterator Chain".to_string(),
                start_line: func.sig.span().start().line,
                end_line: func.block.span().end().line,
                confidence: 0.8,
                reasoning: Some(format!(
                    "Uses functional iterator methods: {}",
                    found_methods.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(", ")
                )),
                code_snippet: format!("Iterator chain with: {}", found_methods.join(", ")),
            });
        }
    }

    fn detect_lifetime_patterns(&mut self, func: &ItemFn) {
        if !func.sig.generics.lifetimes().collect::<Vec<_>>().is_empty() {
            self.patterns.push(Pattern {
                pattern_type: "Lifetime Annotations".to_string(),
                start_line: func.sig.span().start().line,
                end_line: func.sig.span().end().line,
                confidence: 0.75,
                reasoning: Some("Explicit lifetime annotations for reference management".to_string()),
                code_snippet: format!("fn {}<'a>(...)", func.sig.ident),
            });
            
            self.decisions.push(DecisionNode {
                id: format!("decision_{}", self.decisions.len() + 1),
                decision_type: DecisionType::LifetimeAnnotation,
                description: format!("Lifetime annotation strategy for '{}'", func.sig.ident),
                alternatives: vec![
                    Alternative {
                        name: "Explicit lifetimes".to_string(),
                        description: "Manually specified lifetime parameters".to_string(),
                        score: 0.8,
                    },
                    Alternative {
                        name: "Elided lifetimes".to_string(),
                        description: "Let compiler infer lifetimes".to_string(),
                        score: 0.7,
                    },
                ],
                chosen: "Explicit lifetimes".to_string(),
                confidence: 0.75,
            });
        }
    }
}

impl<'ast> Visit<'ast> for PatternVisitor {
    fn visit_item_fn(&mut self, func: &'ast ItemFn) {
        self.detect_error_handling(func);
        self.detect_iterator_patterns(func);
        self.detect_lifetime_patterns(func);
        visit::visit_item_fn(self, func);
    }

    fn visit_item_impl(&mut self, impl_block: &'ast ItemImpl) {
        let impl_str = quote::quote!(#impl_block).to_string();
        
        if impl_str.contains("impl") {
            self.patterns.push(Pattern {
                pattern_type: "Trait Implementation".to_string(),
                start_line: impl_block.impl_token.span.start().line,
                end_line: impl_block.brace_token.span.close().line,
                confidence: 0.85,
                reasoning: Some("Custom trait implementation for type".to_string()),
                code_snippet: "impl Trait for Type { ... }".to_string(),
            });
        }
        
        visit::visit_item_impl(self, impl_block);
    }

    fn visit_item_struct(&mut self, struct_item: &'ast ItemStruct) {
        self.patterns.push(Pattern {
            pattern_type: "Data Structure".to_string(),
            start_line: struct_item.struct_token.span.start().line,
            end_line: struct_item.semi_token.map(|s| s.span.end().line)
                .unwrap_or_else(|| struct_item.struct_token.span.end().line),
            confidence: 0.9,
            reasoning: Some("Custom data structure definition".to_string()),
            code_snippet: format!("struct {}", struct_item.ident),
        });
        
        visit::visit_item_struct(self, struct_item);
    }

    fn visit_item_use(&mut self, use_item: &'ast ItemUse) {
        let use_str = quote::quote!(#use_item).to_string();
        
        let is_std = use_str.contains("std::");
        let confidence = if is_std { 0.9 } else { 0.7 };
        
        self.imports.push(Import {
            module: use_str.clone(),
            items: vec!["imported".to_string()],
            reasoning: if is_std {
                "Standard library import".to_string()
            } else {
                "External crate import".to_string()
            },
            confidence,
        });
        
        visit::visit_item_use(self, use_item);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_simple_function() {
        let source = r#"
            fn add(a: i32, b: i32) -> i32 {
                a + b
            }
        "#;
        
        let analyzer = CodeAnalyzer::new();
        let result = analyzer.analyze(source, "test.rs");
        assert!(result.is_ok());
    }

    #[test]
    fn test_detect_result_pattern() {
        let source = r#"
            fn divide(a: i32, b: i32) -> Result<i32, String> {
                if b == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(a / b)
                }
            }
        "#;
        
        let analyzer = CodeAnalyzer::new();
        let report = analyzer.analyze(source, "test.rs").unwrap();
        
        assert!(report.patterns.iter().any(|p| p.pattern_type == "Error Handling"));
    }

    #[test]
    fn test_detect_iterator_chain() {
        let source = r#"
            fn process_numbers(nums: Vec<i32>) -> Vec<i32> {
                nums.iter()
                    .filter(|&x| x > 0)
                    .map(|x| x * 2)
                    .collect()
            }
        "#;
        
        let analyzer = CodeAnalyzer::new();
        let report = analyzer.analyze(source, "test.rs").unwrap();
        
        assert!(report.patterns.iter().any(|p| p.pattern_type == "Iterator Chain"));
    }
}
