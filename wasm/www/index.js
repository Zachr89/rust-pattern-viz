import * as wasm from "rust-pattern-viz-wasm";

const editor = document.getElementById('editor');
const analyzeBtn = document.getElementById('analyze-btn');
const output = document.getElementById('output');
const examplesGrid = document.getElementById('examples-grid');

// Load hard pattern examples
function loadExamples() {
    try {
        const patterns = wasm.get_hard_patterns();
        
        patterns.forEach((pattern, index) => {
            const button = document.createElement('button');
            button.className = 'example-button';
            button.innerHTML = `
                <div class="example-name">${pattern.name}</div>
                <div class="example-desc">${pattern.description}</div>
            `;
            
            button.addEventListener('click', () => {
                editor.value = pattern.code;
                editor.focus();
                
                // Auto-analyze after loading example
                setTimeout(() => {
                    analyzeBtn.click();
                }, 100);
            });
            
            examplesGrid.appendChild(button);
        });
    } catch (error) {
        console.error('Failed to load examples:', error);
        examplesGrid.innerHTML = '<p style="color: #e53e3e;">Failed to load examples</p>';
    }
}

// Initialize examples on page load
loadExamples();

function showLoading() {
    output.innerHTML = `
        <div class="loading">
            <div class="spinner"></div>
            <p>Analyzing patterns...</p>
        </div>
    `;
}

function showError(message) {
    output.innerHTML = `
        <div class="error">
            <strong>❌ Analysis Error</strong><br>
            ${message}
        </div>
    `;
}

function renderAnalysis(report) {
    if (!report.patterns || report.patterns.length === 0) {
        output.innerHTML = `
            <p style="color: #718096; text-align: center; padding: 40px;">
                No patterns detected. Try adding some match expressions or pattern matching code.
            </p>
        `;
        return;
    }

    let html = '<div style="color: #2d3748;">';
    
    // Render patterns
    report.patterns.forEach(pattern => {
        const confidenceClass = 
            pattern.confidence >= 0.8 ? '' :
            pattern.confidence >= 0.5 ? 'medium' : 'low';
        
        html += `
            <div class="pattern-box ${confidenceClass}">
                <div class="pattern-title">${escapeHtml(pattern.pattern_type)}</div>
                <div class="pattern-confidence">
                    Confidence: ${(pattern.confidence * 100).toFixed(0)}%
                </div>
                <div class="pattern-location">
                    Lines ${pattern.start_line}-${pattern.end_line}
                </div>
                ${pattern.reasoning ? `
                    <div class="pattern-reasoning">
                        ${escapeHtml(pattern.reasoning)}
                    </div>
                ` : ''}
            </div>
        `;
    });

    // Render decision nodes if present
    if (report.decision_nodes && report.decision_nodes.length > 0) {
        html += '<h3 style="margin-top: 20px; margin-bottom: 10px; color: #2d3748;">Decision Nodes</h3>';
        report.decision_nodes.forEach(node => {
            html += `
                <div class="pattern-box" style="background: #fff3e0; border-left-color: #f57c00;">
                    <div class="pattern-title">${escapeHtml(node.node_type)}</div>
                    <div class="pattern-reasoning">
                        ${escapeHtml(node.description)}
                    </div>
                    ${node.chosen_option ? `
                        <div style="margin-top: 8px; color: #2e7d32;">
                            <strong>Chosen:</strong> ${escapeHtml(node.chosen_option)}
                        </div>
                    ` : ''}
                </div>
            `;
        });
    }

    // Render imports if present
    if (report.imports && report.imports.length > 0) {
        html += '<h3 style="margin-top: 20px; margin-bottom: 10px; color: #2d3748;">Imports</h3>';
        report.imports.forEach(imp => {
            html += `
                <div class="pattern-box" style="background: #e1bee7; border-left-color: #7b1fa2;">
                    <div class="pattern-title">${escapeHtml(imp.path)}</div>
                    <div class="pattern-location">Category: ${escapeHtml(imp.category)}</div>
                </div>
            `;
        });
    }

    html += '</div>';
    output.innerHTML = html;
}

function escapeHtml(text) {
    const div = document.createElement('div');
    div.textContent = text;
    return div.innerHTML;
}

analyzeBtn.addEventListener('click', async () => {
    const code = editor.value.trim();
    
    if (!code) {
        showError('Please enter some Rust code to analyze');
        return;
    }

    showLoading();
    analyzeBtn.disabled = true;

    try {
        // Small delay to show loading state
        await new Promise(resolve => setTimeout(resolve, 100));
        
        const resultJson = wasm.analyze_code(code);
        const report = JSON.parse(resultJson);
        
        renderAnalysis(report);
    } catch (error) {
        showError(error.toString());
    } finally {
        analyzeBtn.disabled = false;
    }
});

// Allow Ctrl/Cmd+Enter to analyze
editor.addEventListener('keydown', (e) => {
    if ((e.ctrlKey || e.metaKey) && e.key === 'Enter') {
        e.preventDefault();
        analyzeBtn.click();
    }
});
