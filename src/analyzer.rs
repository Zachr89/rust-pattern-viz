use crate::models::{AnalysisReport, DecisionNode, DecisionType, Import, Pattern, ReportMetadata};
use std::path::Path;
use syn::{visit::Visit, Expr, ExprIf, ExprWhile, File, Item, ItemFn, ItemImpl, Pat, Stmt};

pub struct CodeAnalyzer;

impl CodeAnalyzer {
    pub fn new() -> Self {
        Self
    }

    pub fn analyze(&self, source: &str, file_path: &Path) -> Result<AnalysisReport, String> {
        let syntax_tree = syn::parse_file(source).map_err(|e| format!("Parse error: {}", e))?;

        let mut visitor = PatternVisitor::new(source);
        visitor.visit_file(&syntax_tree);

        let overall_confidence = if visitor.patterns.is_empty() {
            0.0
        } else {
            visitor.patterns.iter().map(|p| p.confidence).sum::<f64>()
                / visitor.patterns.len() as f64
        };

        Ok(AnalysisReport {
            file_path: file_path.display().to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            patterns: visitor.patterns,
            import_suggestions: visitor.imports,
            decision_nodes: visitor.decision_nodes,
            overall_confidence,
            metadata: ReportMetadata {
                analyzer_version: env!("CARGO_PKG_VERSION").to_string(),
                rust_version: std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string()),
            },
        })
    }
}

struct PatternVisitor<'a> {
    source: &'a str,
    patterns: Vec<Pattern>,
    imports: Vec<Import>,
    decision_nodes: Vec<DecisionNode>,
    node_counter: usize,
}

impl<'a> PatternVisitor<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            patterns: Vec::new(),
            imports: Vec::new(),
            decision_nodes: Vec::new(),
            node_counter: 0,
        }
    }

    fn next_node_id(&mut self) -> String {
        self.node_counter += 1;
        format!("node_{}", self.node_counter)
    }

    fn get_line_number(&self, span_start: usize) -> usize {
        self.source[..span_start].lines().count()
    }

    fn extract_code_snippet(&self, start: usize, end: usize) -> String {
        self.source
            .get(start..end)
            .unwrap_or("")
            .lines()
            .take(10)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn analyze_if_let(&mut self, if_expr: &ExprIf) {
        // Check if this is an `if let` pattern
        if let Expr::Let(let_expr) = &*if_expr.cond {
            let start_line = self.get_line_number(let_expr.let_token.span.start().into());
            let end_line = start_line + 5; // Approximate

            // Extract pattern information
            let pattern_str = quote::quote!(#let_expr).to_string();
            
            self.patterns.push(Pattern {
                pattern_type: "Conditional Pattern Match (if let)".to_string(),
                start_line,
                end_line,
                confidence: 0.85,
                reasoning: Some(
                    "if let expression allows pattern matching with a success branch. \
                     Falls through to else branch if pattern doesn't match."
                        .to_string(),
                ),
                code_snippet: pattern_str.clone(),
            });

            // Create decision node for control flow
            let node_id = self.next_node_id();
            let pattern_name = self.extract_pattern_name(&let_expr.pat);
            
            self.decision_nodes.push(DecisionNode {
                id: node_id,
                decision_type: DecisionType::ControlFlow,
                description: format!(
                    "if let {} = <expr> - Pattern match with conditional branching",
                    pattern_name
                ),
                alternatives: vec![
                    "Use match expression for exhaustive handling".to_string(),
                    "Use if with manual destructuring".to_string(),
                    "Use pattern guards for additional conditions".to_string(),
                ],
                chosen: format!("if let {} (success branch only)", pattern_name),
                confidence: 0.85,
                reasoning: Some(
                    "if let is optimal when only the success case needs handling. \
                     More concise than match for single-variant extraction."
                        .to_string(),
                ),
            });
        }

        // Visit nested blocks
        self.visit_block(&if_expr.then_branch);
        if let Some((_, else_branch)) = &if_expr.else_branch {
            self.visit_expr(else_branch);
        }
    }

    fn analyze_while_let(&mut self, while_expr: &ExprWhile) {
        // Check if this is a `while let` pattern
        if let Expr::Let(let_expr) = &*while_expr.cond {
            let start_line = self.get_line_number(let_expr.let_token.span.start().into());
            let end_line = start_line + 8; // Approximate

            let pattern_str = quote::quote!(#let_expr).to_string();
            
            self.patterns.push(Pattern {
                pattern_type: "Conditional Loop (while let)".to_string(),
                start_line,
                end_line,
                confidence: 0.88,
                reasoning: Some(
                    "while let expression creates a loop that continues while pattern matches. \
                     Commonly used with iterators and Option/Result types."
                        .to_string(),
                ),
                code_snippet: pattern_str.clone(),
            });

            // Create decision node for loop control flow
            let node_id = self.next_node_id();
            let pattern_name = self.extract_pattern_name(&let_expr.pat);
            
            self.decision_nodes.push(DecisionNode {
                id: node_id,
                decision_type: DecisionType::ControlFlow,
                description: format!(
                    "while let {} = <expr> - Pattern-based loop continuation",
                    pattern_name
                ),
                alternatives: vec![
                    "Use loop with manual break on None/Err".to_string(),
                    "Use for loop if iterating a collection".to_string(),
                    "Use iterator combinators (take_while, filter_map)".to_string(),
                ],
                chosen: format!("while let {} (loops until pattern fails)", pattern_name),
                confidence: 0.88,
                reasoning: Some(
                    "while let is idiomatic for consuming iterators or processing until None/Err. \
                     More readable than manual loop/break patterns."
                        .to_string(),
                ),
            });
        }

        // Visit loop body
        self.visit_block(&while_expr.body);
    }

    fn extract_pattern_name(&self, pat: &Pat) -> String {
        match pat {
            Pat::Ident(ident) => ident.ident.to_string(),
            Pat::TupleStruct(tuple) => {
                let path = &tuple.path;
                quote::quote!(#path).to_string()
            }
            Pat::Struct(struct_pat) => {
                let path = &struct_pat.path;
                quote::quote!(#path).to_string()
            }
            Pat::Tuple(tuple) => format!("({})", 
                tuple.elems.iter().map(|_| "_").collect::<Vec<_>>().join(", ")
            ),
            _ => quote::quote!(#pat).to_string(),
        }
    }

    fn analyze_function(&mut self, func: &ItemFn) {
        let start_line = self.get_line_number(func.sig.fn_token.span.start().into());

        // Check for Result/Option return types (error handling pattern)
        let return_type = &func.sig.output;
        if let syn::ReturnType::Type(_, ty) = return_type {
            let type_str = quote::quote!(#ty).to_string();
            if type_str.contains("Result") || type_str.contains("Option") {
                let confidence = if type_str.contains("Result") { 0.85 } else { 0.75 };
                let pattern_type = if type_str.contains("Result") {
                    "Error Handling (Result)"
                } else {
                    "Optional Values (Option)"
                };

                self.patterns.push(Pattern {
                    pattern_type: pattern_type.to_string(),
                    start_line,
                    end_line: start_line + 1,
                    confidence,
                    reasoning: Some(format!(
                        "Function returns {}, enabling explicit error/absence handling",
                        if type_str.contains("Result") { "Result<T,E>" } else { "Option<T>" }
                    )),
                    code_snippet: format!("fn {}(...) -> {}", func.sig.ident, type_str),
                });

                let node_id = self.next_node_id();
                self.decision_nodes.push(DecisionNode {
                    id: node_id,
                    decision_type: DecisionType::ErrorHandling,
                    description: format!("Return type: {}", type_str),
                    alternatives: vec![
                        "panic! on error".to_string(),
                        "unwrap/expect".to_string(),
                        "custom error enum".to_string(),
                    ],
                    chosen: type_str.clone(),
                    confidence,
                    reasoning: Some("Using Result/Option for explicit error handling".to_string()),
                });
            }
        }

        // Visit function body to find if let / while let patterns
        for stmt in &func.block.stmts {
            self.visit_stmt(stmt);
        }
    }
}

impl<'a> Visit<'a> for PatternVisitor<'a> {
    fn visit_file(&mut self, file: &'a File) {
        for item in &file.items {
            match item {
                Item::Fn(func) => self.analyze_function(func),
                Item::Use(use_item) => {
                    let path = quote::quote!(#use_item).to_string();
                    let category = if path.contains("std::") {
                        "Standard Library"
                    } else if path.contains("::") {
                        "External Crate"
                    } else {
                        "Local Module"
                    };

                    self.imports.push(Import {
                        path: path.clone(),
                        category: category.to_string(),
                        confidence: 0.9,
                        reasoning: Some(format!("Import from {}", category)),
                    });

                    let node_id = self.next_node_id();
                    self.decision_nodes.push(DecisionNode {
                        id: node_id,
                        decision_type: DecisionType::ImportChoice,
                        description: format!("Import: {}", path),
                        alternatives: vec!["Manual implementation".to_string()],
                        chosen: path,
                        confidence: 0.9,
                        reasoning: Some("Using external dependency".to_string()),
                    });
                }
                Item::Impl(impl_item) => {
                    self.visit_item_impl(impl_item);
                }
                Item::Struct(_) | Item::Enum(_) => {
                    // Could add struct/enum pattern analysis here
                }
                _ => {}
            }
        }
    }

    fn visit_item_impl(&mut self, impl_item: &'a ItemImpl) {
        for item in &impl_item.items {
            if let syn::ImplItem::Fn(method) = item {
                self.analyze_function(&syn::ItemFn {
                    attrs: method.attrs.clone(),
                    vis: method.vis.clone(),
                    sig: method.sig.clone(),
                    block: Box::new(method.block.clone()),
                });
            }
        }
    }

    fn visit_expr(&mut self, expr: &'a Expr) {
        match expr {
            Expr::If(if_expr) => {
                self.analyze_if_let(if_expr);
            }
            Expr::While(while_expr) => {
                self.analyze_while_let(while_expr);
            }
            Expr::Match(match_expr) => {
                let start_line = self.get_line_number(match_expr.match_token.span.start().into());
                
                self.patterns.push(Pattern {
                    pattern_type: "Pattern Matching (match)".to_string(),
                    start_line,
                    end_line: start_line + match_expr.arms.len(),
                    confidence: 0.90,
                    reasoning: Some("Exhaustive pattern matching with match expression".to_string()),
                    code_snippet: quote::quote!(#match_expr).to_string().lines().take(10).collect::<Vec<_>>().join("\n"),
                });

                let node_id = self.next_node_id();
                self.decision_nodes.push(DecisionNode {
                    id: node_id,
                    decision_type: DecisionType::PatternSelection,
                    description: format!("match expression with {} arms", match_expr.arms.len()),
                    alternatives: vec![
                        "if let chain".to_string(),
                        "multiple if statements".to_string(),
                    ],
                    chosen: "match (exhaustive)".to_string(),
                    confidence: 0.90,
                    reasoning: Some("match provides exhaustive pattern matching".to_string()),
                });
            }
            _ => {}
        }

        // Continue visiting nested expressions
        syn::visit::visit_expr(self, expr);
    }

    fn visit_stmt(&mut self, stmt: &'a Stmt) {
        syn::visit::visit_stmt(self, stmt);
    }
}

impl Default for CodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_if_let_detection() {
        let source = r#"
fn main() {
    let x = Some(5);
    if let Some(value) = x {
        println!("Got: {}", value);
    }
}
        "#;

        let analyzer = CodeAnalyzer::new();
        let report = analyzer.analyze(source, Path::new("test.rs")).unwrap();

        assert!(report.patterns.iter().any(|p| p.pattern_type.contains("if let")));
        assert!(report.decision_nodes.iter().any(|n| n.description.contains("if let")));
    }

    #[test]
    fn test_while_let_detection() {
        let source = r#"
fn main() {
    let mut iter = vec![1, 2, 3].into_iter();
    while let Some(value) = iter.next() {
        println!("{}", value);
    }
}
        "#;

        let analyzer = CodeAnalyzer::new();
        let report = analyzer.analyze(source, Path::new("test.rs")).unwrap();

        assert!(report.patterns.iter().any(|p| p.pattern_type.contains("while let")));
        assert!(report.decision_nodes.iter().any(|n| n.description.contains("while let")));
    }
}
