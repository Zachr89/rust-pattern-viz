import * as vscode from 'vscode';
import { PatternVizViewProvider } from './viewProvider';

let viewProvider: PatternVizViewProvider | undefined;

export function activate(context: vscode.ExtensionContext) {
    console.log('Rust Pattern Visualizer extension activated');

    // Create and register the webview provider
    viewProvider = new PatternVizViewProvider(context.extensionUri);
    
    context.subscriptions.push(
        vscode.window.registerWebviewViewProvider(
            'rustPatternViz.view',
            viewProvider,
            {
                webviewOptions: {
                    retainContextWhenHidden: true
                }
            }
        )
    );

    // Register refresh command
    context.subscriptions.push(
        vscode.commands.registerCommand('rustPatternViz.refresh', () => {
            viewProvider?.refresh();
        })
    );

    // Register export command
    context.subscriptions.push(
        vscode.commands.registerCommand('rustPatternViz.exportSvg', async () => {
            const svg = await viewProvider?.getSvg();
            if (svg) {
                const uri = await vscode.window.showSaveDialog({
                    filters: {
                        'SVG Images': ['svg']
                    },
                    defaultUri: vscode.Uri.file('pattern-viz.svg')
                });
                
                if (uri) {
                    await vscode.workspace.fs.writeFile(uri, Buffer.from(svg, 'utf8'));
                    vscode.window.showInformationMessage(`SVG exported to ${uri.fsPath}`);
                }
            }
        })
    );

    // Listen for active editor changes
    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(editor => {
            if (editor && editor.document.languageId === 'rust') {
                viewProvider?.updateContent(editor.document);
            }
        })
    );

    // Listen for document changes (with debouncing via configuration)
    let debounceTimer: NodeJS.Timeout | undefined;
    context.subscriptions.push(
        vscode.workspace.onDidChangeTextDocument(event => {
            const config = vscode.workspace.getConfiguration('rustPatternViz');
            if (config.get('autoRefresh') && event.document.languageId === 'rust') {
                if (debounceTimer) {
                    clearTimeout(debounceTimer);
                }
                const delay = config.get<number>('debounceDelay', 500);
                debounceTimer = setTimeout(() => {
                    viewProvider?.updateContent(event.document);
                }, delay);
            }
        })
    );

    // Initial analysis if a Rust file is already open
    const editor = vscode.window.activeTextEditor;
    if (editor && editor.document.languageId === 'rust') {
        viewProvider.updateContent(editor.document);
    }
}

export function deactivate() {
    console.log('Rust Pattern Visualizer extension deactivated');
}
