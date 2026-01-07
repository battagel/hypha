import * as vscode from 'vscode';
import { HyphaCli, Topic } from './cli';
import { MAX_BADGE_VALUE_LENGTH } from './constants';

/**
 * Format a frontmatter value for display as a badge.
 */
function formatValue(value: unknown): string {
    if (Array.isArray(value)) {
        return value.map(String).join(', ');
    }
    return String(value);
}

/**
 * Get field display settings from configuration.
 */
function getFieldSettings(): { displayFields: string[]; maxBadges: number } {
    const config = vscode.workspace.getConfiguration('hypha');
    return {
        displayFields: config.get<string[]>('displayFields') || [],
        maxBadges: config.get<number>('maxBadges') || 3,
    };
}

/**
 * Build badge string from frontmatter fields.
 * Only shows fields explicitly listed in displayFields (whitelist-only mode).
 */
function buildBadges(frontmatter: Record<string, unknown>): string {
    const { displayFields, maxBadges } = getFieldSettings();
    
    // Whitelist-only: if no fields specified, show nothing
    if (displayFields.length === 0) {
        return '';
    }

    let entries = Object.entries(frontmatter || {});
    if (entries.length === 0) {
        return '';
    }

    // Only show whitelisted fields
    entries = entries.filter(([key]) => displayFields.includes(key));

    if (entries.length === 0) {
        return '';
    }

    const badges: string[] = [];
    const shown = entries.slice(0, maxBadges);
    const remaining = entries.length - maxBadges;

    for (const [key, value] of shown) {
        const formatted = formatValue(value);
        // Keep badges concise
        if (formatted.length > MAX_BADGE_VALUE_LENGTH) {
            badges.push(`${key}:${formatted.slice(0, MAX_BADGE_VALUE_LENGTH - 3)}...`);
        } else {
            badges.push(`${key}:${formatted}`);
        }
    }

    if (remaining > 0) {
        badges.push(`+${remaining}`);
    }

    return badges.join(' │ ');
}

/**
 * Tree item representing a topic in the sidebar.
 */
export class TopicItem extends vscode.TreeItem {
    constructor(
        public readonly topic: Topic,
        public readonly rootDir: string,
        public readonly hasWarning: boolean = false
    ) {
        super(topic.title, vscode.TreeItemCollapsibleState.None);
        
        // Show frontmatter badges only (no description inline)
        const badges = buildBadges(topic.frontmatter);
        if (badges) {
            this.description = badges;
        }

        // Tooltip with full details (description shown here)
        this.tooltip = new vscode.MarkdownString();
        this.tooltip.appendMarkdown(`**${topic.title}**\n\n`);
        if (topic.description) {
            this.tooltip.appendMarkdown(`${topic.description}\n\n`);
        }
        
        if (Object.keys(topic.frontmatter || {}).length > 0) {
            this.tooltip.appendMarkdown(`---\n\n`);
            for (const [key, value] of Object.entries(topic.frontmatter || {})) {
                const formatted = formatValue(value);
                this.tooltip.appendMarkdown(`**${key}:** ${formatted}\n\n`);
            }
        }

        // Icon - show warning if topic has issues
        this.iconPath = hasWarning 
            ? new vscode.ThemeIcon('warning', new vscode.ThemeColor('list.warningForeground'))
            : new vscode.ThemeIcon('note');

        // Context value for menus
        this.contextValue = 'topic';

        // Command to open on click
        this.command = {
            command: 'hypha.openTopic',
            title: 'Open Topic',
            arguments: [this],
        };
    }
}

/**
 * Message item shown in tree (e.g., search results header).
 */
export class MessageItem extends vscode.TreeItem {
    constructor(message: string, icon?: string) {
        super(message, vscode.TreeItemCollapsibleState.None);
        this.iconPath = new vscode.ThemeIcon(icon || 'info');
        this.contextValue = 'message';
    }
}

/**
 * Tree data provider for the Hypha sidebar.
 */
export class TopicTreeProvider implements vscode.TreeDataProvider<vscode.TreeItem> {
    private _onDidChangeTreeData = new vscode.EventEmitter<void>();
    readonly onDidChangeTreeData = this._onDidChangeTreeData.event;

    private _onTopicsLoaded = new vscode.EventEmitter<void>();
    readonly onTopicsLoaded = this._onTopicsLoaded.event;

    private searchQuery: string = '';
    private backlinksTarget: string = ''; // Topic title to find backlinks for
    private topics: Topic[] = [];
    private filteredTopics: Topic[] | null = null; // External filter from filter view
    private rootDir: string = '';
    private warningPaths: Set<string> = new Set();

    constructor(private readonly cli: HyphaCli) {}

    /**
     * Set externally filtered topics from filter view.
     * Pass null to clear the filter.
     */
    setFilteredTopics(topics: Topic[] | null): void {
        this.filteredTopics = topics;
        this.refresh();
    }

    /**
     * Set search filter.
     */
    setSearch(query: string): void {
        this.searchQuery = query;
        this.backlinksTarget = ''; // Clear backlinks when searching
        vscode.commands.executeCommand('setContext', 'hypha.hasSearch', true);
        this.refresh();
    }

    /**
     * Clear search filter.
     */
    clearSearch(): void {
        this.searchQuery = '';
        this.backlinksTarget = '';
        vscode.commands.executeCommand('setContext', 'hypha.hasSearch', false);
        this.refresh();
    }

    /**
     * Set backlinks filter to show topics linking to a specific topic.
     */
    setBacklinksFilter(topicTitle: string): void {
        this.backlinksTarget = topicTitle;
        this.searchQuery = ''; // Clear search when showing backlinks
        vscode.commands.executeCommand('setContext', 'hypha.hasSearch', true);
        this.refresh();
    }

    /**
     * Get current search query.
     */
    getSearchQuery(): string {
        return this.searchQuery;
    }

    /**
     * Refresh the tree.
     */
    refresh(): void {
        this._onDidChangeTreeData.fire();
    }

    async getChildren(): Promise<vscode.TreeItem[]> {
        // Check if configured
        if (!this.cli.isConfigured()) {
            // Set context so welcome view shows
            await vscode.commands.executeCommand('setContext', 'hypha.isConfigured', false);
            return []; // Welcome view will show instead
        }

        await vscode.commands.executeCommand('setContext', 'hypha.isConfigured', true);

        try {
            // Get root dir for path resolution
            this.rootDir = this.cli.getRootDir() || '';

            // Fetch topics (backlinks, search, or list)
            if (this.backlinksTarget) {
                this.topics = await this.cli.backlinks(this.backlinksTarget);
            } else if (this.searchQuery) {
                this.topics = await this.cli.search(this.searchQuery);
            } else if (this.filteredTopics !== null) {
                // Use externally filtered topics from filter view
                this.topics = this.filteredTopics;
            } else {
                this.topics = await this.cli.list();
            }

            if (this.topics.length === 0) {
                this._onTopicsLoaded.fire();
                if (this.backlinksTarget) {
                    return [new MessageItem(`No backlinks to "${this.backlinksTarget}"`, 'search')];
                }
                if (this.searchQuery) {
                    return [new MessageItem(`No results for "${this.searchQuery}"`, 'search')];
                }
                return [];
            }

            // Build tree items
            const items: vscode.TreeItem[] = [];

            // Show header if filtering
            if (this.backlinksTarget) {
                items.push(new MessageItem(
                    `Backlinks to "${this.backlinksTarget}" (${this.topics.length})`,
                    'search'
                ));
            } else if (this.searchQuery) {
                items.push(new MessageItem(
                    `Search: "${this.searchQuery}" (${this.topics.length} results)`,
                    'search'
                ));
            }

            // Add topic items
            for (const topic of this.topics) {
                const hasWarning = this.warningPaths.has(topic.path);
                items.push(new TopicItem(topic, this.rootDir, hasWarning));
            }

            // Notify that topics are loaded (for title update)
            this._onTopicsLoaded.fire();

            return items;
        } catch (err) {
            console.error('Failed to load topics:', err);
            return [new MessageItem('Failed to load topics', 'error')];
        }
    }

    getTreeItem(element: vscode.TreeItem): vscode.TreeItem {
        return element;
    }

    /**
     * Get parent of an element (required for reveal).
     * Returns undefined since we have a flat tree structure.
     */
    getParent(): vscode.TreeItem | undefined {
        return undefined;
    }

    /**
     * Get all loaded topics.
     */
    getTopics(): Topic[] {
        return this.topics;
    }

    /**
     * Get topic count.
     */
    getTopicCount(): number {
        return this.topics.length;
    }

    /**
     * Get root directory.
     */
    getRootDir(): string {
        return this.rootDir;
    }

    /**
     * Set paths that have warnings (from lint results).
     * Only refreshes if warnings actually changed.
     */
    setWarnings(paths: string[]): void {
        const newSet = new Set(paths);
        
        // Check if warnings changed
        if (this.warningPaths.size === newSet.size && 
            [...this.warningPaths].every(p => newSet.has(p))) {
            return; // No change
        }
        
        this.warningPaths = newSet;
        this._onDidChangeTreeData.fire();
    }

    /**
     * Find a TopicItem by file path.
     * Returns undefined if no matching topic is found.
     */
    findTopicByPath(filePath: string): TopicItem | undefined {
        const topic = this.topics.find(t => t.path === filePath);
        if (topic) {
            const hasWarning = this.warningPaths.has(topic.path);
            return new TopicItem(topic, this.rootDir, hasWarning);
        }
        return undefined;
    }
}
