import * as vscode from 'vscode';
import * as path from 'path';
import { HyphaCli, SearchResult } from './cli';
import { TopicTreeProvider, TopicItem } from './topicTree';
import { DEBOUNCE_MS, PROJECT_REPO_URL, MAX_RECENT_TOPICS, MAX_DESCRIPTION_LENGTH, SEPARATOR_WIDTH } from './constants';

/**
 * Helper function to open a document in the editor.
 */
async function openDocument(filePath: string): Promise<void> {
    const doc = await vscode.workspace.openTextDocument(filePath);
    await vscode.window.showTextDocument(doc);
}

/**
 * Helper function to create a new topic and open it.
 */
async function createAndOpenTopic(
    cli: HyphaCli,
    title: string,
    treeProvider: TopicTreeProvider
): Promise<void> {
    const filePath = await cli.newTopic(title);
    
    treeProvider.refresh();

    if (filePath) {
        await openDocument(filePath);
    }
}

export function registerCommands(
    context: vscode.ExtensionContext,
    cli: HyphaCli,
    treeProvider: TopicTreeProvider
): void {
    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.new', async (providedTitle?: string) => {
            const title = providedTitle || await vscode.window.showInputBox({
                prompt: 'Enter topic title',
                placeHolder: 'My New Topic',
            });
            if (!title) return;

            try {
                await createAndOpenTopic(cli, title, treeProvider);
            } catch (err) {
                vscode.window.showErrorMessage(`Failed to create topic: ${err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.openTopic', async (item?: TopicItem) => {
            if (!item) {
                const topics = treeProvider.getTopics();
                if (topics.length === 0) {
                    vscode.window.showInformationMessage('No topics to open');
                    return;
                }

                const selected = await vscode.window.showQuickPick(
                    topics.map(t => ({
                        label: t.title,
                        description: t.description || '',
                        topic: t,
                    })),
                    { placeHolder: 'Select topic to open' }
                );

                if (!selected) return;
                item = new TopicItem(selected.topic, treeProvider.getRootDir());
            }

            const filePath = item.topic.path;
            if (!filePath) {
                vscode.window.showErrorMessage('Topic path not available');
                return;
            }

            try {
                await openDocument(filePath);
            } catch (err) {
                vscode.window.showErrorMessage(`Failed to open topic: ${err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.openPreview', async (item?: TopicItem) => {
            if (!item) return;

            const filePath = item.topic.path;
            if (!filePath) {
                vscode.window.showErrorMessage('Topic path not available');
                return;
            }

            try {
                const uri = vscode.Uri.file(filePath);
                await vscode.commands.executeCommand('markdown.showPreview', uri);
            } catch (err) {
                vscode.window.showErrorMessage(`Failed to open preview: ${err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.deleteTopic', async (item?: TopicItem) => {
            if (!item) return;

            const confirm = await vscode.window.showWarningMessage(
                `Delete "${item.topic.title}"?`,
                { modal: true },
                'Delete'
            );
            if (confirm !== 'Delete') return;

            try {
                await cli.deleteTopic(item.topic.title);
                treeProvider.refresh();
            } catch (err) {
                vscode.window.showErrorMessage(`Failed to delete topic: ${err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.findBacklinks', async (item?: TopicItem) => {
            if (!item) return;

            try {
                // Focus the view and show backlinks
                await vscode.commands.executeCommand('hypha.topicsView.focus');
                treeProvider.setBacklinksFilter(item.topic.title);
            } catch (err) {
                vscode.window.showErrorMessage(`Failed to find backlinks: ${err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.renameTopic', async (item?: TopicItem) => {
            if (!item) return;

            const newName = await vscode.window.showInputBox({
                prompt: 'Enter new topic name',
                value: item.topic.title,
                placeHolder: 'New Topic Name',
            });
            if (!newName || newName === item.topic.title) return;

            try {
                await cli.renameTopic(item.topic.title, newName);
                treeProvider.refresh();
                vscode.window.showInformationMessage(`Renamed to "${newName}"`);
            } catch (err) {
                vscode.window.showErrorMessage(`Failed to rename topic: ${err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.search', async () => {
            // Focus the Hypha view first so user can see results
            await vscode.commands.executeCommand('hypha.topicsView.focus');
            
            const inputBox = vscode.window.createInputBox();
            inputBox.placeholder = 'Filter topics (e.g., "status:active" or "meeting")';
            inputBox.value = treeProvider.getSearchQuery();
            inputBox.title = 'Filter View';

            let debounceTimer: NodeJS.Timeout | undefined;

            inputBox.onDidChangeValue(value => {
                if (debounceTimer) clearTimeout(debounceTimer);
                debounceTimer = setTimeout(() => {
                    if (value === '') {
                        treeProvider.clearSearch();
                    } else {
                        treeProvider.setSearch(value);
                    }
                }, DEBOUNCE_MS);
            });

            inputBox.onDidAccept(() => {
                const value = inputBox.value;
                if (value === '') {
                    treeProvider.clearSearch();
                } else {
                    treeProvider.setSearch(value);
                }
                inputBox.hide();
            });

            inputBox.onDidHide(() => {
                if (debounceTimer) clearTimeout(debounceTimer);
                inputBox.dispose();
            });

            inputBox.show();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.clearSearch', () => {
            treeProvider.clearSearch();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.quickFind', async () => {
            try {
                const recentPaths = context.workspaceState.get<string[]>('hypha.recentTopics', []);

                type QuickPickItem = {
                    label: string;
                    description: string;
                    detail: string | undefined;
                    path: string;
                    isRecent: boolean;
                    alwaysShow: boolean;
                };

                const makeItems = (topicList: Awaited<ReturnType<typeof cli.list>>): QuickPickItem[] => topicList.map(t => ({
                    label: t.title,
                    description: t.description?.slice(0, MAX_DESCRIPTION_LENGTH) || '',
                    detail: recentPaths.includes(t.path) ? '$(history) Recently opened' : undefined,
                    path: t.path,
                    isRecent: recentPaths.includes(t.path),
                    alwaysShow: true,
                })).sort((a, b) => {
                    if (a.isRecent && !b.isRecent) return -1;
                    if (!a.isRecent && b.isRecent) return 1;
                    return a.label.localeCompare(b.label);
                });

                const quickPick = vscode.window.createQuickPick<QuickPickItem>();
                quickPick.placeholder = 'Find topic... (e.g. subject:AI status:active)';
                quickPick.matchOnDescription = false;
                quickPick.matchOnDetail = false;
                quickPick.busy = true;

                // Initial load
                const initialTopics = await cli.list();
                quickPick.items = makeItems(initialTopics);
                quickPick.busy = false;

                if (initialTopics.length === 0) {
                    vscode.window.showInformationMessage('No topics found');
                    return;
                }

                // Debounced search using CLI
                let debounceTimer: NodeJS.Timeout | undefined;
                quickPick.onDidChangeValue(value => {
                    if (debounceTimer) clearTimeout(debounceTimer);
                    debounceTimer = setTimeout(async () => {
                        quickPick.busy = true;
                        try {
                            const results = value ? await cli.find(value) : await cli.list();
                            
                            // If no results and user typed something, offer to create a new topic
                            if (value && results.length === 0) {
                                quickPick.items = [{
                                    label: `$(add) Create new topic: "${value}"`,
                                    description: 'No matches found',
                                    detail: 'Press Enter to create this topic',
                                    path: '', // Will be created
                                    isRecent: false,
                                    alwaysShow: true,
                                }];
                            } else {
                                quickPick.items = makeItems(results);
                            }
                        } catch {
                            // Keep current items on error
                        }
                        quickPick.busy = false;
                    }, DEBOUNCE_MS);
                });

                quickPick.onDidAccept(async () => {
                    const selected = quickPick.selectedItems[0];
                    if (!selected) return;

                    quickPick.hide();

                    // Check if this is the "create new topic" item
                    if (!selected.path && selected.label.startsWith('$(add)')) {
                        const titleMatch = selected.label.match(/Create new topic: "(.+)"/);
                        if (titleMatch) {
                            const title = titleMatch[1];
                            try {
                                await createAndOpenTopic(cli, title, treeProvider);
                            } catch (err) {
                                vscode.window.showErrorMessage(`Failed to create topic: ${err}`);
                            }
                        }
                        return;
                    }

                    const newRecent = [selected.path, ...recentPaths.filter(p => p !== selected.path)].slice(0, MAX_RECENT_TOPICS);
                    await context.workspaceState.update('hypha.recentTopics', newRecent);

                    await openDocument(selected.path);
                });

                quickPick.onDidHide(() => {
                    if (debounceTimer) clearTimeout(debounceTimer);
                    quickPick.dispose();
                });
                quickPick.show();
            } catch (err) {
                vscode.window.showErrorMessage(`Failed to open topic: ${err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.info', async () => {
            try {
                const output = await cli.info(true);
                showOutput('Hypha Info', output);
            } catch (err) {
                vscode.window.showErrorMessage(`Failed to get info: ${err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.lint', async () => {
            try {
                const output = await cli.lint();

                if (output.includes('No issues found')) {
                    vscode.window.showInformationMessage('✓ No issues found');
                } else {
                    showOutput('Lint Results', output);
                    vscode.window.showWarningMessage('Issues found. See Output panel.');
                }
            } catch (err) {
                vscode.window.showErrorMessage(`Lint failed: ${err}`);
            }
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.copyLink', async (item?: TopicItem) => {
            if (!item) return;

            const fileName = item.topic.path.split('/').pop() || '';
            const title = item.topic.title;
            const markdownLink = `[${title}](${fileName})`;

            await vscode.env.clipboard.writeText(markdownLink);
            vscode.window.showInformationMessage(`Copied: ${markdownLink}`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.copyPath', async (item?: TopicItem) => {
            if (!item) return;

            const rootDir = cli.getRootDir();
            let relativePath = item.topic.path;
            if (rootDir && relativePath.startsWith(rootDir)) {
                relativePath = relativePath.slice(rootDir.length + 1);
            }

            await vscode.env.clipboard.writeText(relativePath);
            vscode.window.showInformationMessage(`Copied: ${relativePath}`);
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.refresh', () => {
            treeProvider.refresh();
        })
    );
    
    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.openInFinder', () => {
            const rootDir = cli.getRootDir();
            if (!rootDir) {
                vscode.window.showWarningMessage('Hypha root directory not configured');
                return;
            }
            vscode.env.openExternal(vscode.Uri.file(rootDir));
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.settings', () => {
            vscode.commands.executeCommand('workbench.action.openSettings', '@ext:battagel.hypha-vscode');
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.documentation', () => {
            vscode.env.openExternal(vscode.Uri.parse(PROJECT_REPO_URL));
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.searchBody', async () => {
            const rootDir = cli.getRootDir();
            if (!rootDir) {
                vscode.window.showWarningMessage('Hypha root directory not configured');
                return;
            }

            type SearchQuickPickItem = vscode.QuickPickItem & {
                filePath: string;
                line: number;
            };

            const quickPick = vscode.window.createQuickPick<SearchQuickPickItem>();
            quickPick.placeholder = 'Search in topic content...';
            quickPick.matchOnDescription = true;
            quickPick.matchOnDetail = true;

            const makeItems = (results: SearchResult[]): SearchQuickPickItem[] => {
                const items: SearchQuickPickItem[] = [];
                const MAX_ITEMS = 100; // Limit to prevent UI overload
                for (const result of results) {
                    if (items.length >= MAX_ITEMS) break;
                    const filename = path.basename(result.path);
                    for (const match of result.matches) {
                        if (items.length >= MAX_ITEMS) break;
                        items.push({
                            label: `$(file) ${result.title}`,
                            description: `${filename}:${match.line}`,
                            detail: match.content.trim(),
                            filePath: result.path,
                            line: match.line,
                        });
                    }
                }
                return items;
            };

            let debounceTimer: NodeJS.Timeout | undefined;
            quickPick.onDidChangeValue(value => {
                if (debounceTimer) clearTimeout(debounceTimer);
                if (!value || value.length < 3) {
                    quickPick.items = [];
                    return;
                }
                debounceTimer = setTimeout(async () => {
                    quickPick.busy = true;
                    try {
                        const results = await cli.search(value);
                        quickPick.items = makeItems(results);
                    } catch (err) {
                        console.error('Search error:', err);
                        quickPick.items = [];
                    }
                    quickPick.busy = false;
                }, DEBOUNCE_MS);
            });

            quickPick.onDidAccept(async () => {
                const selected = quickPick.selectedItems[0];
                if (!selected) return;

                quickPick.hide();

                const doc = await vscode.workspace.openTextDocument(selected.filePath);
                const editor = await vscode.window.showTextDocument(doc);
                
                // Go to the matched line
                const position = new vscode.Position(selected.line - 1, 0);
                editor.selection = new vscode.Selection(position, position);
                editor.revealRange(new vscode.Range(position, position), vscode.TextEditorRevealType.InCenter);
            });

            quickPick.onDidHide(() => {
                if (debounceTimer) clearTimeout(debounceTimer);
                quickPick.dispose();
            });

            quickPick.show();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.cycleSort', async () => {
            const newOrder = cli.cycleSortOrder();
            await vscode.commands.executeCommand('setContext', 'hypha.sortOrder', newOrder);
            treeProvider.refresh();
        })
    );

    // Register sort commands using a helper to reduce duplication
    const registerSortCommand = (name: 'alpha' | 'modified' | 'created') => {
        context.subscriptions.push(
            vscode.commands.registerCommand(`hypha.sort${name.charAt(0).toUpperCase() + name.slice(1)}`, async () => {
                cli.setSortOrder(name);
                await vscode.commands.executeCommand('setContext', 'hypha.sortOrder', name);
                treeProvider.refresh();
            })
        );
    };

    registerSortCommand('alpha');
    registerSortCommand('modified');
    registerSortCommand('created');

}

function showOutput(title: string, content: string): void {
    const channel = vscode.window.createOutputChannel('Hypha');
    channel.clear();
    channel.appendLine(title);
    channel.appendLine('='.repeat(SEPARATOR_WIDTH));
    channel.appendLine(content);
    channel.show();
}
