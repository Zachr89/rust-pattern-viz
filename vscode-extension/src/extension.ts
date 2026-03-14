import * as path from 'path';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
} from 'vscode-languageclient/node';
import * as fs from 'fs';
import * as child_process from 'child_process';

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext) {
    console.log('Rust Pattern Viz extension activating...');

    const serverPath = await findServerBinary();
    if (!serverPath) {
        vscode.window.showErrorMessage(
            'Could not find rpv-lsp binary. Please build it with `cargo build --bin rpv-lsp` or set rustPatternViz.serverPath.'
        );
        return;
    }

    console.log('Found rpv-lsp at:', serverPath);

    const serverOptions: ServerOptions = {
        command: serverPath,
        args: [],
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'rust' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.rs'),
        },
    };

    client = new LanguageClient(
        'rustPatternViz',
        'Rust Pattern Visualizer',
        serverOptions,
        clientOptions
    );

    try {
        await client.start();
        console.log('Rust Pattern Viz LSP server started successfully');
    } catch (error) {
        console.error('Failed to start LSP server:', error);
        vscode.window.showErrorMessage(
            `Failed to start Rust Pattern Viz LSP server: ${error}`
        );
        return;
    }

    // Register restart command
    context.subscriptions.push(
        vscode.commands.registerCommand('rustPatternViz.restartServer', async () => {
            if (client) {
                await client.stop();
                await client.start();
                vscode.window.showInformationMessage('Rust Pattern Viz server restarted');
            }
        })
    );

    // Register share analysis command
    context.subscriptions.push(
        vscode.commands.registerCommand('rustPatternViz.shareAnalysis', async () => {
            await shareCurrentAnalysis();
        })
    );

    console.log('Rust Pattern Viz extension activated');
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

async function findServerBinary(): Promise<string | undefined> {
    // 1. Check user configuration
    const config = vscode.workspace.getConfiguration('rustPatternViz');
    const configPath = config.get<string>('serverPath');
    if (configPath && fs.existsSync(configPath)) {
        return configPath;
    }

    // 2. Check workspace target directories
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders) {
        for (const folder of workspaceFolders) {
            const debugPath = path.join(folder.uri.fsPath, 'target', 'debug', 'rpv-lsp');
            const releasePath = path.join(folder.uri.fsPath, 'target', 'release', 'rpv-lsp');

            if (fs.existsSync(releasePath)) {
                return releasePath;
            }
            if (fs.existsSync(debugPath)) {
                return debugPath;
            }
        }
    }

    // 3. Check system PATH
    try {
        const result = child_process.execSync('which rpv-lsp', { encoding: 'utf8' });
        const systemPath = result.trim();
        if (systemPath && fs.existsSync(systemPath)) {
            return systemPath;
        }
    } catch {
        // which command failed, rpv-lsp not in PATH
    }

    return undefined;
}

async function shareCurrentAnalysis() {
    const editor = vscode.window.activeTextEditor;
    if (!editor || editor.document.languageId !== 'rust') {
        vscode.window.showWarningMessage('Please open a Rust file to share analysis');
        return;
    }

    const document = editor.document;
    const position = editor.selection.active;

    try {
        // Request hover at current position to get analysis
        const hover = await vscode.commands.executeCommand<vscode.Hover[]>(
            'vscode.executeHoverProvider',
            document.uri,
            position
        );

        if (!hover || hover.length === 0) {
            vscode.window.showInformationMessage('No analysis available at current position');
            return;
        }

        // Parse the analysis from hover markdown
        // In a real implementation, we'd have a custom LSP command to get the raw AnalysisReport
        // For now, we'll make a direct API call with the document content

        const config = vscode.workspace.getConfiguration('rustPatternViz');
        const shareServerUrl = config.get<string>('shareServerUrl') || 'http://localhost:3030';

        // Get analysis by analyzing current document
        const sourceCode = document.getText();
        const analysis = await analyzeCode(sourceCode, document.fileName);

        // Create share via API
        const response = await fetch(`${shareServerUrl}/api/share`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({ report: analysis }),
        });

        if (!response.ok) {
            throw new Error(`Share API returned ${response.status}`);
        }

        const result = await response.json();
        const shareUrl = result.share_url;

        // Copy to clipboard
        await vscode.env.clipboard.writeText(shareUrl);

        // Show notification with link
        const action = await vscode.window.showInformationMessage(
            `Analysis shared! Link copied to clipboard.`,
            'Open in Browser'
        );

        if (action === 'Open in Browser') {
            vscode.env.openExternal(vscode.Uri.parse(shareUrl));
        }
    } catch (error) {
        console.error('Failed to share analysis:', error);
        vscode.window.showErrorMessage(`Failed to share analysis: ${error}`);
    }
}

async function analyzeCode(sourceCode: string, filePath: string): Promise<any> {
    // This is a simplified version - in production, we'd use the actual analyzer
    // or communicate with the LSP server for the full analysis
    
    // For now, create a minimal analysis structure
    return {
        file_path: filePath,
        timestamp: new Date().toISOString(),
        patterns: [],
        import_suggestions: [],
        decision_nodes: [],
        overall_confidence: 0.0,
        metadata: {
            analyzer_version: '0.1.0',
            total_lines: sourceCode.split('\n').length,
            analyzed_constructs: 0,
        },
    };
}
