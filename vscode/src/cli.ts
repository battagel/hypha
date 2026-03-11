import { execFile, ExecFileOptions } from 'child_process';
import { promisify } from 'util';
import * as vscode from 'vscode';
import * as path from 'path';
import * as fs from 'fs';

const execFileAsync = promisify(execFile);

// 10MB buffer to handle large search results
const EXEC_OPTIONS: ExecFileOptions = {
    maxBuffer: 10 * 1024 * 1024,
    timeout: 30000,
    encoding: 'utf8',
};

export interface Topic {
    title: string;
    description: string | null;
    path: string;
    frontmatter: Record<string, unknown>;
}

export interface LintWarning {
    message: string;
    line?: number;
    column?: number;
}

export interface LintResult {
    title: string;
    path: string;
    warnings: LintWarning[];
}

export interface SearchMatch {
    line: number;
    column: number;
    content: string;
}

export interface SearchResult {
    path: string;
    title: string;
    matches: SearchMatch[];
}

export class HyphaCli {
    private sortOrder: 'alpha' | 'modified' | 'created' = 'alpha';

    private getBundledBinaryPath(): string | null {
        const ext = process.platform === 'win32' ? '.exe' : '';
        
        // Map process.platform and process.arch to our binary directory structure
        const platformMap: Record<string, string> = {
            'darwin-x64': 'darwin-x64',
            'darwin-arm64': 'darwin-arm64',
            'linux-x64': 'linux-x64',
            'win32-x64': 'win32-x64'
        };
        
        const platformKey = `${process.platform}-${process.arch}`;
        const platformDir = platformMap[platformKey];
        
        if (!platformDir) {
            return null;
        }
        
        // Path to bundled binary in the extension
        const bundledPath = path.join(__dirname, '..', 'bin', platformDir, `hypha${ext}`);
        
        if (fs.existsSync(bundledPath)) {
            return bundledPath;
        }
        
        return null;
    }

    private getBinaryPath(): string {
        const config = vscode.workspace.getConfiguration('hypha');
        const configPath = config.get<string>('binaryPath');
        
        // Priority: 1. User config, 2. Bundled binary, 3. PATH
        if (configPath) {
            return configPath;
        }
        
        const bundled = this.getBundledBinaryPath();
        if (bundled) {
            return bundled;
        }
        
        return 'hypha';
    }

    getRootDir(): string | undefined {
        const config = vscode.workspace.getConfiguration('hypha');
        return config.get<string>('rootDir') || undefined;
    }

    isConfigured(): boolean {
        return !!this.getRootDir();
    }

    getSortOrder(): 'alpha' | 'modified' | 'created' {
        return this.sortOrder;
    }

    setSortOrder(order: 'alpha' | 'modified' | 'created'): void {
        this.sortOrder = order;
    }

    cycleSortOrder(): 'alpha' | 'modified' | 'created' {
        const orders: Array<'alpha' | 'modified' | 'created'> = ['alpha', 'modified', 'created'];
        const currentIndex = orders.indexOf(this.sortOrder);
        this.sortOrder = orders[(currentIndex + 1) % orders.length];
        return this.sortOrder;
    }

    async isAvailable(): Promise<boolean> {
        try {
            await execFileAsync(this.getBinaryPath(), ['--version']);
            return true;
        } catch {
            return false;
        }
    }

    private async run(args: string[]): Promise<string> {
        const root = this.getRootDir();
        const fullArgs = root ? ['--root', root, ...args] : args;

        try {
            const { stdout } = await execFileAsync(this.getBinaryPath(), fullArgs, EXEC_OPTIONS);
            return stdout as string;
        } catch (err: unknown) {
            const error = err as { stderr?: string; message?: string };
            throw new Error(error.stderr || error.message || 'Command failed');
        }
    }

    private async runAllowNonZero(args: string[]): Promise<string> {
        const root = this.getRootDir();
        const fullArgs = root ? ['--root', root, ...args] : args;

        try {
            const { stdout } = await execFileAsync(this.getBinaryPath(), fullArgs, EXEC_OPTIONS);
            return stdout as string;
        } catch (err: unknown) {
            const error = err as { stdout?: string; stderr?: string };
            if (error.stdout) {
                return error.stdout;
            }
            throw new Error(error.stderr || String(err));
        }
    }

    private getSortArg(): string {
        return this.sortOrder;
    }

    async list(): Promise<Topic[]> {
        const sort = this.getSortArg();
        const output = await this.run(['list', '--json', '--sort', sort]);
        return this.parseTopics(output);
    }

    async find(query: string): Promise<Topic[]> {
        const sort = this.getSortArg();
        const output = await this.run(['find', query, '--json', '--sort', sort]);
        return this.parseTopics(output);
    }

    async search(pattern: string): Promise<SearchResult[]> {
        try {
            const output = await this.run(['search', pattern, '--json', '-i']);
            return JSON.parse(output) as SearchResult[];
        } catch (err) {
            console.error('Search failed:', err);
            return [];
        }
    }

    async newTopic(title: string): Promise<string> {
        const output = await this.run(['new', title, '--no-edit']);
        const match = output.match(/Created: (.+)/);
        return match?.[1]?.trim() || '';
    }

    async deleteTopic(topic: string): Promise<void> {
        await this.run(['delete', topic]);
    }

    async renameTopic(oldName: string, newName: string): Promise<void> {
        await this.run(['rename', oldName, newName]);
    }

    async backlinks(topic: string): Promise<Topic[]> {
        const output = await this.run(['backlinks', topic, '--json']);
        return this.parseTopics(output);
    }

    async info(verbose: boolean = true): Promise<string> {
        const args = verbose ? ['info', '--verbose'] : ['info'];
        return await this.run(args);
    }

    async lint(): Promise<string> {
        return await this.runAllowNonZero(['lint']);
    }

    async lintJson(): Promise<LintResult[]> {
        try {
            const output = await this.runAllowNonZero(['lint', '--json']);
            return JSON.parse(output) as LintResult[];
        } catch {
            return [];
        }
    }

    private parseTopics(output: string): Topic[] {
        try {
            const topics = JSON.parse(output) as Topic[];
            return topics.map(t => ({
                title: t.title,
                description: t.description || null,
                path: t.path,
                frontmatter: t.frontmatter || {},
            }));
        } catch {
            return [];
        }
    }
}
