import * as vscode from 'vscode';
import { HyphaCli } from './cli';
import { TopicTreeProvider, TopicItem } from './topicTree';
import { DEBOUNCE_MS, PROJECT_REPO_URL, MAX_RECENT_TOPICS, MAX_DESCRIPTION_LENGTH, SEPARATOR_WIDTH } from './constants';

export function registerCommands(
    context: vscode.ExtensionContext,
    cli: HyphaCli,
    treeProvider: TopicTreeProvider
): void {
    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.new', async () => {
            const title = await vscode.window.showInputBox({
                prompt: 'Enter topic title',
                placeHolder: 'My New Topic',
            });
            if (!title) return;

            try {
                const filePath = await cli.newTopic(title);
                treeProvider.refresh();

                if (filePath) {
                    const doc = await vscode.workspace.openTextDocument(filePath);
                    await vscode.window.showTextDocument(doc);
                }
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
                const doc = await vscode.workspace.openTextDocument(filePath);
                await vscode.window.showTextDocument(doc);
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
            inputBox.placeholder = 'Search topics (e.g., "status:active" or "meeting")';
            inputBox.value = treeProvider.getSearchQuery();
            inputBox.title = 'Search Topics';

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
                quickPick.placeholder = 'Search topics... (e.g. subject:AI status:active)';
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
                            const results = value ? await cli.search(value) : await cli.list();
                            quickPick.items = makeItems(results);
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

                    const newRecent = [selected.path, ...recentPaths.filter(p => p !== selected.path)].slice(0, MAX_RECENT_TOPICS);
                    await context.workspaceState.update('hypha.recentTopics', newRecent);

                    const doc = await vscode.workspace.openTextDocument(selected.path);
                    await vscode.window.showTextDocument(doc);
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
            vscode.commands.executeCommand('workbench.action.openSettings', '@ext:hypha');
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

            const query = await vscode.window.showInputBox({
                prompt: 'Search in note content',
                placeHolder: 'Enter search term...',
            });
            if (!query) return;

            await vscode.commands.executeCommand('workbench.action.findInFiles', {
                query: query,
                filesToInclude: rootDir,
                triggerSearch: true,
                isRegex: false,
                isCaseSensitive: false,
            });
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.cycleSort', async () => {
            const newOrder = cli.cycleSortOrder();
            await vscode.commands.executeCommand('setContext', 'hypha.sortOrder', newOrder);
            treeProvider.refresh();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.sortAlpha', async () => {
            cli.setSortOrder('alpha');
            await vscode.commands.executeCommand('setContext', 'hypha.sortOrder', 'alpha');
            treeProvider.refresh();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.sortModified', async () => {
            cli.setSortOrder('modified');
            await vscode.commands.executeCommand('setContext', 'hypha.sortOrder', 'modified');
            treeProvider.refresh();
        })
    );

    context.subscriptions.push(
        vscode.commands.registerCommand('hypha.sortCreated', async () => {
            cli.setSortOrder('created');
            await vscode.commands.executeCommand('setContext', 'hypha.sortOrder', 'created');
            treeProvider.refresh();
        })
    );

}

function showOutput(title: string, content: string): void {
    const channel = vscode.window.createOutputChannel('Hypha');
    channel.clear();
    channel.appendLine(title);
    channel.appendLine('='.repeat(SEPARATOR_WIDTH));
    channel.appendLine(content);
    channel.show();
}
