use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result as RpcResult;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

mod analyzer;
mod models;
mod visualizer;

use analyzer::CodeAnalyzer;

#[derive(Debug)]
struct Backend {
    client: Client,
    analyzer: Arc<CodeAnalyzer>,
    document_cache: Arc<Mutex<HashMap<Url, String>>>,
}

impl Backend {
    fn new(client: Client) -> Self {
        Self {
            client,
            analyzer: Arc::new(CodeAnalyzer::new()),
            document_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    async fn analyze_document(&self, uri: &Url) -> Option<String> {
        let cache = self.document_cache.lock().await;
        let source = cache.get(uri)?;
        
        let file_path = uri.to_file_path().ok()?.to_string_lossy().to_string();
        
        match self.analyzer.analyze(source, &file_path) {
            Ok(report) => {
                let summary = self.format_hover_content(&report);
                Some(summary)
            }
            Err(e) => {
                eprintln!("Analysis error: {}", e);
                None
            }
        }
    }

    fn format_hover_content(&self, report: &models::AnalysisReport) -> String {
        let mut content = String::new();
        
        content.push_str("## 🦀 Rust Pattern Analysis\n\n");
        
        if !report.patterns.is_empty() {
            content.push_str(&format!("**Patterns Found:** {}\n\n", report.patterns.len()));
            
            // Show top 3 patterns
            for (idx, pattern) in report.patterns.iter().take(3).enumerate() {
                content.push_str(&format!(
                    "{}. **{}** (Line {}-{}) - Confidence: {:.1}%\n",
                    idx + 1,
                    pattern.pattern_type,
                    pattern.start_line,
                    pattern.end_line,
                    pattern.confidence * 100.0
                ));
                
                if let Some(reason) = &pattern.reasoning {
                    content.push_str(&format!("   - {}\n", reason));
                }
            }
            
            if report.patterns.len() > 3 {
                content.push_str(&format!("\n   _...and {} more patterns_\n", report.patterns.len() - 3));
            }
            content.push_str("\n");
        }
        
        if !report.import_suggestions.is_empty() {
            content.push_str(&format!("**Imports Analyzed:** {}\n", report.import_suggestions.len()));
            
            let selected: Vec<_> = report.import_suggestions.iter()
                .filter(|s| matches!(s.status, models::ImportStatus::Selected))
                .collect();
            
            if !selected.is_empty() {
                content.push_str(&format!("- ✓ {} selected\n", selected.len()));
                
                for import in selected.iter().take(2) {
                    content.push_str(&format!("  - `{}` (confidence: {:.0}%)\n", 
                        import.import_path, import.confidence * 100.0));
                }
            }
            content.push_str("\n");
        }
        
        content.push_str(&format!("**Overall Confidence:** {:.1}%\n", report.overall_confidence * 100.0));
        content.push_str(&format!("**Complexity Score:** {:.1}\n\n", report.metadata.complexity_score));
        
        if !report.decision_nodes.is_empty() {
            content.push_str("### Decision Tree\n\n");
            
            for node in report.decision_nodes.iter().take(3) {
                let emoji = match node.decision_type {
                    models::DecisionType::ImportChoice => "📦",
                    models::DecisionType::PatternSelection => "🔧",
                    models::DecisionType::ErrorHandling => "⚠️",
                    models::DecisionType::TypeInference => "🎯",
                };
                
                content.push_str(&format!("{} **{}**\n", emoji, node.description));
                
                if !node.alternatives.is_empty() {
                    content.push_str(&format!("   Considered {} alternatives\n", node.alternatives.len()));
                }
            }
        }
        
        content.push_str("\n---\n");
        content.push_str("*Powered by rust-pattern-viz*");
        
        content
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> RpcResult<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "rust-pattern-viz-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Rust Pattern Viz LSP initialized")
            .await;
    }

    async fn shutdown(&self) -> RpcResult<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;
        
        let mut cache = self.document_cache.lock().await;
        cache.insert(uri, text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        
        if let Some(change) = params.content_changes.first() {
            let mut cache = self.document_cache.lock().await;
            cache.insert(uri, change.text.clone());
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let mut cache = self.document_cache.lock().await;
        cache.remove(&params.text_document.uri);
    }

    async fn hover(&self, params: HoverParams) -> RpcResult<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;
        
        // Check if this is a Rust file
        if !uri.as_str().ends_with(".rs") {
            return Ok(None);
        }
        
        // Get cached document
        let cache = self.document_cache.lock().await;
        let source = match cache.get(uri) {
            Some(s) => s.clone(),
            None => return Ok(None),
        };
        drop(cache);
        
        // Check if hovering over a function or important construct
        let lines: Vec<&str> = source.lines().collect();
        let line_idx = position.line as usize;
        
        if line_idx >= lines.len() {
            return Ok(None);
        }
        
        let line = lines[line_idx];
        
        // Only show hover on functions, impl blocks, or struct definitions
        let should_show = line.trim_start().starts_with("pub fn") 
            || line.trim_start().starts_with("fn ")
            || line.trim_start().starts_with("impl ")
            || line.trim_start().starts_with("pub struct")
            || line.trim_start().starts_with("struct ");
        
        if !should_show {
            return Ok(None);
        }
        
        // Perform analysis
        let content = match self.analyze_document(uri).await {
            Some(c) => c,
            None => return Ok(None),
        };
        
        Ok(Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: content,
            }),
            range: None,
        }))
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend::new(client));
    
    Server::new(stdin, stdout, socket).serve(service).await;
}
