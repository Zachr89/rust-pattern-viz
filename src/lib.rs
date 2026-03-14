//! Rust Pattern Visualizer - Core library and WASM bindings
//! 
//! This library provides pattern analysis and visualization for Rust code.
//! It can be used as a Rust library, compiled to WASM for web/extension use,
//! or invoked via the CLI tool.

use wasm_bindgen::prelude::*;

mod analyzer;
mod models;
mod svg_renderer;

pub use analyzer::CodeAnalyzer;
pub use models::{AnalysisReport, Pattern, DecisionNode, Import};
pub use svg_renderer::SvgRenderer;

/// WASM entry point: Analyze Rust code and return SVG visualization
/// 
/// # Arguments
/// * `code` - Rust source code as string
/// * `show_confidence` - Whether to display confidence scores in output
/// 
/// # Returns
/// SVG string containing the visualization, or error message
#[wasm_bindgen]
pub fn analyze_code_to_svg(code: &str, show_confidence: bool) -> Result<String, JsValue> {
    // Set panic hook for better error messages in WASM
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    // Analyze the code
    let analyzer = CodeAnalyzer::new();
    let report = analyzer.analyze_code(code)
        .map_err(|e| JsValue::from_str(&format!("Analysis error: {}", e)))?;

    // Render as SVG
    let renderer = SvgRenderer::new();
    let svg = renderer.render(&report, show_confidence)
        .map_err(|e| JsValue::from_str(&format!("Rendering error: {}", e)))?;

    Ok(svg)
}

/// WASM entry point: Analyze code and return JSON report
/// 
/// # Arguments
/// * `code` - Rust source code as string
/// 
/// # Returns
/// JSON string containing the full analysis report
#[wasm_bindgen]
pub fn analyze_code_to_json(code: &str) -> Result<String, JsValue> {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let analyzer = CodeAnalyzer::new();
    let report = analyzer.analyze_code(code)
        .map_err(|e| JsValue::from_str(&format!("Analysis error: {}", e)))?;

    serde_json::to_string_pretty(&report)
        .map_err(|e| JsValue::from_str(&format!("JSON serialization error: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_analysis() {
        let code = r#"
fn main() {
    let result = some_operation();
    match result {
        Ok(value) => println!("{}", value),
        Err(e) => eprintln!("{}", e),
    }
}
"#;
        let analyzer = CodeAnalyzer::new();
        let report = analyzer.analyze_code(code).expect("Analysis should succeed");
        
        assert!(!report.patterns.is_empty(), "Should detect patterns");
    }

    #[test]
    fn test_svg_rendering() {
        let code = r#"
fn test() {
    if let Some(x) = optional_value {
        process(x);
    }
}
"#;
        let svg = analyze_code_to_svg(code, true).expect("Should render SVG");
        
        assert!(svg.contains("<svg"), "Should contain SVG tag");
        assert!(svg.contains("if let"), "Should mention the pattern");
    }
}
