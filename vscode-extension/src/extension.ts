import * as path from 'path';
import * as vscode from 'vscode';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    TransportKind
} from 'vscode-languageclient/node';

let client: LanguageClient | undefined;

export async function activate(context: vscode.ExtensionContext) {
    console.log('Rust Pattern Viz extension activating...');

    const config = vscode.workspace.getConfiguration('rustPatternViz');
    
    if (!config.get('enable')) {
        console.log('Rust Pattern Viz is disabled');
        return;
    }

    // Find the LSP server binary
    const serverPath = await findServerPath(config.get('serverPath') as string);
    
    if (!serverPath) {
        vscode.window.showErrorMessage(
            'Rust Pattern Viz: Could not find rpv-lsp binary. Please build the project with: cargo build --bin rpv-lsp'
        );
        return;
    }

    // Define server options
    const serverOptions: ServerOptions = {
        command: serverPath,
        args: [],
        transport: TransportKind.stdio
    };

    // Define client options
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'rust' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/*.rs')
        }
    };

    // Create and start the language client
    client = new LanguageClient(
        'rustPatternViz',
        'Rust Pattern Visualizer',
        serverOptions,
        clientOptions
    );

    // Start the client (this will also start the server)
    await client.start();

    console.log('Rust Pattern Viz extension activated');

    // Register restart command
    const restartCommand = vscode.commands.registerCommand(
        'rustPatternViz.restartServer',
        async () => {
            if (client) {
                await client.stop();
                await client.start();
                vscode.window.showInformationMessage('Rust Pattern Viz server restarted');
            }
        }
    );

    context.subscriptions.push(restartCommand);

    // Show activation message
    vscode.window.showInformationMessage(
        'Rust Pattern Viz is now active! Hover over Rust functions to see pattern analysis.'
    );
}

export async function deactivate(): Promise<void> {
    if (client) {
        await client.stop();
    }
}

async function findServerPath(configPath: string): Promise<string | undefined> {
    // If user specified a path, use that
    if (configPath && configPath.trim() !== '') {
        return configPath;
    }

    // Try to find in workspace
    const workspaceFolders = vscode.workspace.workspaceFolders;
    if (workspaceFolders) {
        for (const folder of workspaceFolders) {
            // Check debug build
            let serverPath = path.join(folder.uri.fsPath, 'target', 'debug', 'rpv-lsp');
            if (process.platform === 'win32') {
                serverPath += '.exe';
            }
            
            try {
                await vscode.workspace.fs.stat(vscode.Uri.file(serverPath));
                return serverPath;
            } catch {
                // Try release build
                serverPath = path.join(folder.uri.fsPath, 'target', 'release', 'rpv-lsp');
                if (process.platform === 'win32') {
                    serverPath += '.exe';
                }
                
                try {
                    await vscode.workspace.fs.stat(vscode.Uri.file(serverPath));
                    return serverPath;
                } catch {
                    // Continue to next folder
                }
            }
        }
    }

    // Try to find in PATH
    const pathEnv = process.env.PATH || '';
    const pathDirs = pathEnv.split(process.platform === 'win32' ? ';' : ':');
    
    for (const dir of pathDirs) {
        let serverPath = path.join(dir, 'rpv-lsp');
        if (process.platform === 'win32') {
            serverPath += '.exe';
        }
        
        try {
            await vscode.workspace.fs.stat(vscode.Uri.file(serverPath));
            return serverPath;
        } catch {
            // Continue to next directory
        }
    }

    return undefined;
}
