import * as vscode from 'vscode';
import { HyphaCli } from './cli';
import { TopicTreeProvider } from './topicTree';
import { registerCommands } from './commands';
import { PROJECT_REPO_URL } from './constants';

export async function activate(context: vscode.ExtensionContext): Promise<void> {
    console.log('Hypha extension activating...');

    const cli = new HyphaCli();

    // Set initial configuration context
    await vscode.commands.executeCommand('setContext', 'hypha.isConfigured', cli.isConfigured());
    await vscode.commands.executeCommand('setContext', 'hypha.sortOrder', cli.getSortOrder());

    // Check if hypha CLI is available
    const available = await cli.isAvailable();
    if (!available) {
        vscode.window.showWarningMessage(
            'Hypha CLI not found. Please install Hypha and ensure it is in your PATH, or configure the binary path in settings.',
            'Open Documentation'
        ).then(selection => {
            if (selection === 'Open Documentation') {
                vscode.env.openExternal(vscode.Uri.parse(PROJECT_REPO_URL));
            }
        });
    }

    // Create diagnostics collection for lint warnings
    const diagnostics = vscode.languages.createDiagnosticCollection('hypha');
    context.subscriptions.push(diagnostics);

    // Run lint asynchronously and update diagnostics + tree icons
    const runLintAsync = async () => {
        try {
            const results = await cli.lintJson();
            diagnostics.clear();
            
            const warningPaths: string[] = [];
            for (const result of results) {
                warningPaths.push(result.path);
                const uri = vscode.Uri.file(result.path);
                const fileDiagnostics = result.warnings.map(warning => {
                    // Use 0-based positions (VS Code is 0-indexed, CLI is 1-indexed)
                    const line = warning.line ? warning.line - 1 : 0;
                    const column = warning.column ? warning.column - 1 : 0;
                    const diagnostic = new vscode.Diagnostic(
                        new vscode.Range(line, column, line, column),
                        warning.message,
                        vscode.DiagnosticSeverity.Warning
                    );
                    diagnostic.source = 'hypha';
                    return diagnostic;
                });
                diagnostics.set(uri, fileDiagnostics);
            }
            
            treeProvider.setWarnings(warningPaths);
        } catch {
            // Silently ignore lint errors
        }
    };

    const treeProvider = new TopicTreeProvider(cli);
    
    const treeView = vscode.window.createTreeView('hypha.topicsView', {
        treeDataProvider: treeProvider,
        showCollapseAll: false,
    });
    context.subscriptions.push(treeView);

    // Reveal topic in tree when active editor changes (only if tree is visible)
    const revealActiveEditor = (editor: vscode.TextEditor | undefined) => {
        if (!treeView.visible) {
            return; // Don't reveal if panel isn't already visible
        }
        if (!editor || editor.document.languageId !== 'markdown') {
            return;
        }
        const filePath = editor.document.uri.fsPath;
        const topicItem = treeProvider.findTopicByPath(filePath);
        if (topicItem) {
            treeView.reveal(topicItem, { select: true, focus: false });
        }
    };

    // Update title and sync selection when topics are loaded
    treeProvider.onTopicsLoaded(() => {
        const count = treeProvider.getTopicCount();
        const search = treeProvider.getSearchQuery();
        const sortOrder = cli.getSortOrder();
        const sortLabel = sortOrder === 'alpha' ? '' : ` • ${sortOrder}`;
        if (search) {
            treeView.title = `Topics (${count} results${sortLabel})`;
        } else if (count > 0) {
            treeView.title = `Topics (${count}${sortLabel})`;
        } else {
            treeView.title = 'Topics';
        }
        revealActiveEditor(vscode.window.activeTextEditor);
    });

    registerCommands(context, cli, treeProvider);

    // Watch for markdown file changes
    const watcher = vscode.workspace.createFileSystemWatcher('**/*.md');
    watcher.onDidChange(() => {
        treeProvider.refresh();
        runLintAsync();
    });
    watcher.onDidCreate(() => {
        treeProvider.refresh();
        runLintAsync();
    });
    watcher.onDidDelete(() => {
        treeProvider.refresh();
        runLintAsync();
    });
    context.subscriptions.push(watcher);

    // Also listen to document saves (more reliable for editor changes)
    context.subscriptions.push(
        vscode.workspace.onDidSaveTextDocument(doc => {
            if (doc.languageId === 'markdown') {
                treeProvider.refresh();
                runLintAsync();
            }
        })
    );

    // Watch for config changes
    context.subscriptions.push(
        vscode.workspace.onDidChangeConfiguration(e => {
            if (e.affectsConfiguration('hypha')) {
                treeProvider.refresh();
            }
        })
    );

    context.subscriptions.push(
        vscode.window.onDidChangeActiveTextEditor(revealActiveEditor)
    );

    // When tree view becomes visible, sync selection with active editor
    context.subscriptions.push(
        treeView.onDidChangeVisibility(e => {
            if (e.visible) {
                revealActiveEditor(vscode.window.activeTextEditor);
            }
        })
    );

    treeProvider.refresh();
    runLintAsync();

    console.log('Hypha extension activated');
}

export function deactivate(): void {
    console.log('Hypha extension deactivated');
}