import * as vscode from 'vscode';

export class PatternVizViewProvider implements vscode.WebviewViewProvider {
    private _view?: vscode.WebviewView;
    private _currentSvg?: string;

    constructor(private readonly _extensionUri: vscode.Uri) {}

    public resolveWebviewView(
        webviewView: vscode.WebviewView,
        context: vscode.WebviewViewResolveContext,
        _token: vscode.CancellationToken
    ) {
        this._view = webviewView;

        webviewView.webview.options = {
            enableScripts: true,
            localResourceRoots: [
                vscode.Uri.joinPath(this._extensionUri, 'wasm')
            ]
        };

        webviewView.webview.html = this._getInitialHtml(webviewView.webview);

        // Handle messages from the webview
        webviewView.webview.onDidReceiveMessage(message => {
            switch (message.command) {
                case 'ready':
                    // Webview is ready, trigger initial analysis
                    const editor = vscode.window.activeTextEditor;
                    if (editor && editor.document.languageId === 'rust') {
                        this.updateContent(editor.document);
                    }
                    break;
                case 'error':
                    vscode.window.showErrorMessage(`Pattern Viz: ${message.error}`);
                    break;
            }
        });
    }

    public refresh() {
        const editor = vscode.window.activeTextEditor;
        if (editor && editor.document.languageId === 'rust') {
            this.updateContent(editor.document);
        } else {
            vscode.window.showWarningMessage('No active Rust file to analyze');
        }
    }

    public async updateContent(document: vscode.TextDocument) {
        if (!this._view) {
            return;
        }

        const code = document.getText();
        
        // Send code to webview for WASM analysis
        this._view.webview.postMessage({
            command: 'analyze',
            code: code,
            fileName: document.fileName
        });
    }

    public async getSvg(): Promise<string | undefined> {
        return this._currentSvg;
    }

    private _getInitialHtml(webview: vscode.Webview): string {
        const wasmUri = webview.asWebviewUri(
            vscode.Uri.joinPath(this._extensionUri, 'wasm', 'rust_pattern_viz_bg.wasm')
        );
        const jsUri = webview.asWebviewUri(
            vscode.Uri.joinPath(this._extensionUri, 'wasm', 'rust_pattern_viz.js')
        );

        const config = vscode.workspace.getConfiguration('rustPatternViz');
        const showConfidence = config.get('showConfidenceScores', true);

        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src ${webview.cspSource} 'unsafe-inline'; script-src ${webview.cspSource} 'unsafe-inline'; img-src ${webview.cspSource} data:;">
    <title>Rust Pattern Visualization</title>
    <style>
        body {
            padding: 0;
            margin: 0;
            overflow: hidden;
            background: var(--vscode-editor-background);
            color: var(--vscode-editor-foreground);
            font-family: var(--vscode-font-family);
        }
        #container {
            width: 100%;
            height: 100vh;
            overflow: auto;
            padding: 10px;
            box-sizing: border-box;
        }
        #loading {
            text-align: center;
            padding: 20px;
            color: var(--vscode-descriptionForeground);
        }
        #error {
            color: var(--vscode-errorForeground);
            padding: 10px;
            border: 1px solid var(--vscode-errorBorder);
            border-radius: 4px;
            margin: 10px;
            display: none;
        }
        #visualization {
            width: 100%;
        }
        #visualization svg {
            width: 100%;
            height: auto;
        }
        .status {
            padding: 5px 10px;
            font-size: 12px;
            color: var(--vscode-descriptionForeground);
            border-bottom: 1px solid var(--vscode-panel-border);
        }
        .empty-state {
            text-align: center;
            padding: 40px 20px;
            color: var(--vscode-descriptionForeground);
        }
        .empty-state h3 {
            margin: 0 0 10px 0;
        }
        .empty-state p {
            margin: 5px 0;
            font-size: 13px;
        }
    </style>
</head>
<body>
    <div id="container">
        <div class="status" id="status">Ready to analyze Rust code...</div>
        <div id="loading" style="display: none;">Analyzing patterns...</div>
        <div id="error"></div>
        <div id="visualization">
            <div class="empty-state">
                <h3>🦀 Rust Pattern Visualizer</h3>
                <p>Open a Rust file to see pattern matching and control flow visualizations.</p>
                <p>Supported patterns: match expressions, if let, while let, error handling, and more.</p>
            </div>
        </div>
    </div>

    <script type="module">
        const vscode = acquireVsCodeApi();
        let wasmModule;

        // Load WASM module
        async function initWasm() {
            try {
                const { default: init, analyze_code_to_svg } = await import('${jsUri}');
                await init('${wasmUri}');
                wasmModule = { analyze_code_to_svg };
                vscode.postMessage({ command: 'ready' });
            } catch (error) {
                showError('Failed to initialize WASM module: ' + error.message);
                vscode.postMessage({ command: 'error', error: error.message });
            }
        }

        // Handle messages from extension
        window.addEventListener('message', event => {
            const message = event.data;
            
            switch (message.command) {
                case 'analyze':
                    analyzeCode(message.code, message.fileName);
                    break;
            }
        });

        async function analyzeCode(code, fileName) {
            if (!wasmModule) {
                showError('WASM module not initialized');
                return;
            }

            showLoading(true);
            hideError();

            try {
                // Call WASM function to analyze and generate SVG
                const svg = wasmModule.analyze_code_to_svg(code, ${showConfidence});
                
                const vizDiv = document.getElementById('visualization');
                vizDiv.innerHTML = svg;
                
                updateStatus(\`Analyzed \${fileName.split('/').pop() || fileName}\`);
                
                // Store SVG for export
                window.currentSvg = svg;
            } catch (error) {
                showError('Analysis failed: ' + error.message);
                vscode.postMessage({ command: 'error', error: error.message });
            } finally {
                showLoading(false);
            }
        }

        function showLoading(show) {
            document.getElementById('loading').style.display = show ? 'block' : 'none';
        }

        function showError(message) {
            const errorDiv = document.getElementById('error');
            errorDiv.textContent = message;
            errorDiv.style.display = 'block';
        }

        function hideError() {
            document.getElementById('error').style.display = 'none';
        }

        function updateStatus(message) {
            document.getElementById('status').textContent = message;
        }

        // Initialize on load
        initWasm();
    </script>
</body>
</html>`;
    }
}
