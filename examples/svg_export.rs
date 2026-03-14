use rust_pattern_viz::{CodeAnalyzer, SvgRenderer};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Sample Rust code with various patterns
    let sample_code = r#"
use std::fs::File;
use std::io::{self, Read};

fn read_config(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn process_items<T>(items: Vec<T>) -> Vec<T> 
where
    T: Clone + PartialOrd,
{
    items.into_iter()
        .filter(|x| x > &x.clone())
        .collect()
}
"#;

    // Analyze the code
    let analyzer = CodeAnalyzer::new();
    let report = analyzer.analyze(sample_code, "example.rs")?;

    // Render to SVG
    let renderer = SvgRenderer::new();
    let svg = renderer.render(&report);

    // Write to file
    fs::write("output.svg", svg)?;
    
    println!("✓ SVG diagram generated: output.svg");
    println!("✓ Detected {} patterns", report.patterns.len());
    println!("✓ Found {} decision points", report.decision_nodes.len());
    println!("✓ Analyzed {} imports", report.import_suggestions.len());

    Ok(())
}
