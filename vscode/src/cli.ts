import { execFile } from 'child_process';
import { promisify } from 'util';
import * as vscode from 'vscode';

const execFileAsync = promisify(execFile);

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

export class HyphaCli {
    private sortOrder: 'alpha' | 'modified' | 'created' = 'alpha';

    private getBinaryPath(): string {
        const config = vscode.workspace.getConfiguration('hypha');
        return config.get<string>('binaryPath') || 'hypha';
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
            const { stdout } = await execFileAsync(this.getBinaryPath(), fullArgs);
            return stdout;
        } catch (err: unknown) {
            const error = err as { stderr?: string; message?: string };
            throw new Error(error.stderr || error.message || 'Command failed');
        }
    }

    private async runAllowNonZero(args: string[]): Promise<string> {
        const root = this.getRootDir();
        const fullArgs = root ? ['--root', root, ...args] : args;

        try {
            const { stdout } = await execFileAsync(this.getBinaryPath(), fullArgs);
            return stdout;
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

    async search(query: string): Promise<Topic[]> {
        const sort = this.getSortArg();
        const output = await this.run(['search', query, '--json', '--sort', sort]);
        return this.parseTopics(output);
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
