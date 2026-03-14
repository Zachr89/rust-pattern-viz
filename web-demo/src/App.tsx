import React, { useState, useEffect } from 'react';
import Editor from '@monaco-editor/react';
import { WasmAnalyzer } from './wasm-loader';
import { AnalysisReport } from './types';
import { EXAMPLE_CODE } from './examples';
import './App.css';

function App() {
  const [analyzer, setAnalyzer] = useState<WasmAnalyzer | null>(null);
  const [code, setCode] = useState(EXAMPLE_CODE.errorHandling);
  const [report, setReport] = useState<AnalysisReport | null>(null);
  const [loading, setLoading] = useState(true);
  const [analyzing, setAnalyzing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    initWasm();
  }, []);

  const initWasm = async () => {
    try {
      setLoading(true);
      const wasmModule = await import('/wasm/rust_pattern_viz.js');
      await wasmModule.default();
      const wasmAnalyzer = new wasmModule.WasmAnalyzer();
      setAnalyzer(wasmAnalyzer);
      setError(null);
    } catch (err) {
      setError(`Failed to load WASM module: ${err}`);
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  const analyzeCode = async () => {
    if (!analyzer) return;
    
    setAnalyzing(true);
    setError(null);
    
    try {
      const result = analyzer.analyze(code, 'demo.rs');
      const parsedReport: AnalysisReport = JSON.parse(result);
      setReport(parsedReport);
    } catch (err: any) {
      setError(`Analysis failed: ${err.message || err}`);
      setReport(null);
    } finally {
      setAnalyzing(false);
    }
  };

  const loadExample = (exampleCode: string) => {
    setCode(exampleCode);
    setReport(null);
  };

  return (
    <div className="app">
      <header className="header">
        <div className="header-content">
          <h1>🦀 Rust Pattern Viz</h1>
          <p>Interactive Rust code pattern analyzer powered by WebAssembly</p>
        </div>
      </header>

      <main className="main">
        {loading && (
          <div className="loading-overlay">
            <div className="loading-spinner"></div>
            <p>Loading WebAssembly analyzer...</p>
          </div>
        )}

        {error && (
          <div className="error-banner">
            <strong>Error:</strong> {error}
          </div>
        )}

        <div className="layout">
          <div className="editor-section">
            <div className="toolbar">
              <div className="example-buttons">
                <button onClick={() => loadExample(EXAMPLE_CODE.errorHandling)}>
                  Error Handling
                </button>
                <button onClick={() => loadExample(EXAMPLE_CODE.iterators)}>
                  Iterators
                </button>
                <button onClick={() => loadExample(EXAMPLE_CODE.lifetimes)}>
                  Lifetimes
                </button>
                <button onClick={() => loadExample(EXAMPLE_CODE.traits)}>
                  Traits
                </button>
              </div>
              <button 
                className="analyze-button"
                onClick={analyzeCode}
                disabled={!analyzer || analyzing}
              >
                {analyzing ? 'Analyzing...' : '🔍 Analyze'}
              </button>
            </div>

            <Editor
              height="500px"
              defaultLanguage="rust"
              theme="vs-dark"
              value={code}
              onChange={(value) => setCode(value || '')}
              options={{
                minimap: { enabled: false },
                fontSize: 14,
                lineNumbers: 'on',
                scrollBeyondLastLine: false,
                automaticLayout: true,
              }}
            />
          </div>

          <div className="results-section">
            <h2>Analysis Results</h2>
            
            {!report && !analyzing && (
              <div className="placeholder">
                <p>👈 Enter Rust code and click "Analyze" to see pattern detection results</p>
              </div>
            )}

            {analyzing && (
              <div className="analyzing">
                <div className="loading-spinner small"></div>
                <p>Analyzing code patterns...</p>
              </div>
            )}

            {report && (
              <div className="report">
                <div className="report-header">
                  <div className="metric">
                    <span className="label">Overall Confidence:</span>
                    <span className="value">{(report.overall_confidence * 100).toFixed(0)}%</span>
                  </div>
                  <div className="metric">
                    <span className="label">Analysis Time:</span>
                    <span className="value">{report.metadata.analysis_duration_ms}ms</span>
                  </div>
                  <div className="metric">
                    <span className="label">Patterns Found:</span>
                    <span className="value">{report.patterns.length}</span>
                  </div>
                </div>

                <section className="patterns-section">
                  <h3>🎯 Detected Patterns</h3>
                  {report.patterns.length === 0 ? (
                    <p className="empty">No patterns detected</p>
                  ) : (
                    <div className="patterns-list">
                      {report.patterns.map((pattern, idx) => (
                        <div key={idx} className="pattern-card">
                          <div className="pattern-header">
                            <span className="pattern-type">{pattern.pattern_type}</span>
                            <span className="confidence">{(pattern.confidence * 100).toFixed(0)}%</span>
                          </div>
                          <div className="pattern-location">
                            Lines {pattern.start_line}-{pattern.end_line}
                          </div>
                          {pattern.reasoning && (
                            <div className="pattern-reasoning">{pattern.reasoning}</div>
                          )}
                          <code className="pattern-snippet">{pattern.code_snippet}</code>
                        </div>
                      ))}
                    </div>
                  )}
                </section>

                {report.decision_nodes.length > 0 && (
                  <section className="decisions-section">
                    <h3>🌳 Decision Tree</h3>
                    <div className="decisions-list">
                      {report.decision_nodes.map((node, idx) => (
                        <div key={idx} className="decision-card">
                          <div className="decision-header">
                            <span className="decision-type">{node.decision_type}</span>
                            <span className="confidence">{(node.confidence * 100).toFixed(0)}%</span>
                          </div>
                          <div className="decision-description">{node.description}</div>
                          <div className="decision-chosen">
                            <strong>Chosen:</strong> {node.chosen}
                          </div>
                          {node.alternatives.length > 0 && (
                            <details className="alternatives">
                              <summary>View alternatives ({node.alternatives.length})</summary>
                              <ul>
                                {node.alternatives.map((alt, altIdx) => (
                                  <li key={altIdx}>
                                    <strong>{alt.name}</strong> (score: {(alt.score * 100).toFixed(0)}%)
                                    <br />
                                    <span className="alt-description">{alt.description}</span>
                                  </li>
                                ))}
                              </ul>
                            </details>
                          )}
                        </div>
                      ))}
                    </div>
                  </section>
                )}

                {report.import_suggestions.length > 0 && (
                  <section className="imports-section">
                    <h3>📦 Import Analysis</h3>
                    <div className="imports-list">
                      {report.import_suggestions.map((imp, idx) => (
                        <div key={idx} className="import-card">
                          <code>{imp.module}</code>
                          <div className="import-reasoning">{imp.reasoning}</div>
                        </div>
                      ))}
                    </div>
                  </section>
                )}
              </div>
            )}
          </div>
        </div>
      </main>

      <footer className="footer">
        <p>
          Built with Rust + WebAssembly |{' '}
          <a href="https://github.com/yourusername/rust-pattern-viz" target="_blank" rel="noopener noreferrer">
            GitHub
          </a>
          {analyzer && <span> | v{analyzer.version()}</span>}
        </p>
      </footer>
    </div>
  );
}

export default App;
