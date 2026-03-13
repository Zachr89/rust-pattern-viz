use crate::models::*;
use anyhow::Result;
use askama::Template;

pub struct Visualizer;

impl Visualizer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, report: &AnalysisReport, format: VisualizationFormat) -> Result<String> {
        match format {
            VisualizationFormat::Json => self.render_json(report),
            VisualizationFormat::Html => self.render_html(report),
            VisualizationFormat::Svg => self.render_svg(report),
        }
    }

    fn render_json(&self, report: &AnalysisReport) -> Result<String> {
        Ok(serde_json::to_string_pretty(report)?)
    }

    fn render_html(&self, report: &AnalysisReport) -> Result<String> {
        let template = HtmlTemplate { report };
        Ok(template.render()?)
    }

    fn render_svg(&self, report: &AnalysisReport) -> Result<String> {
        let template = SvgTemplate { report };
        Ok(template.render()?)
    }
}

impl Default for Visualizer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Template)]
#[template(path = "report.html")]
struct HtmlTemplate<'a> {
    report: &'a AnalysisReport,
}

#[derive(Template)]
#[template(path = "graph.svg")]
struct SvgTemplate<'a> {
    report: &'a AnalysisReport,
}
