import * as vscode from 'vscode';
import { JobTreeItem, JobTreeItemType } from './jobTreeItem';
import { getBigQueryClient } from '../extensionCommands';

const PAGE_SIZE = 50;

export class JobTreeDataProvider implements vscode.TreeDataProvider<JobTreeItem> {
    private _onDidChangeTreeData: vscode.EventEmitter<JobTreeItem | undefined | null | void> = new vscode.EventEmitter<JobTreeItem | undefined | null | void>();
    readonly onDidChangeTreeData: vscode.Event<JobTreeItem | undefined | null | void> = this._onDidChangeTreeData.event;

    private cachedJobs: JobTreeItem[] = [];
    private nextPageToken: string | undefined = undefined;

    constructor() {
    }

    refresh(): void {
        this.cachedJobs = [];
        this.nextPageToken = undefined;
        this._onDidChangeTreeData.fire();
    }

    getTreeItem(element: JobTreeItem): vscode.TreeItem {
        return element;
    }

    async getChildren(element?: JobTreeItem): Promise<JobTreeItem[]> {
        if (element) {
            return [];
        }

        if (this.cachedJobs.length === 0) {
            return this.fetchJobs();
        }

        return this.cachedJobs;
    }

    async loadMore(pageToken: string | undefined): Promise<void> {
        if (!pageToken) return;
        await this.fetchJobs(pageToken);
        this._onDidChangeTreeData.fire();
    }

    private async fetchJobs(pageToken?: string): Promise<JobTreeItem[]> {
        try {
            const bqClient = await getBigQueryClient();

            // Get setting
            const myJobsOnly = vscode.workspace.getConfiguration().get<boolean>('vscode-bigquery.my-jobs-only') ?? true;

            const [jobs, nextQuery] = await bqClient.bigQuery.getJobs({
                maxResults: PAGE_SIZE,
                allUsers: !myJobsOnly,
                pageToken: pageToken
            });

            this.nextPageToken = nextQuery?.pageToken;

            // Remove the previous loadMore item if it exists
            if (this.cachedJobs.length > 0 && this.cachedJobs[this.cachedJobs.length - 1].treeItemType === JobTreeItemType.loadMore) {
                this.cachedJobs.pop();
            }

            for (const job of jobs) {
                const state = job.metadata?.status?.state || 'UNKNOWN';
                let icon = 'pulse';
                if (state === 'DONE') {
                    if (job.metadata?.status?.errorResult) {
                        icon = 'error';
                    } else {
                        icon = 'check';
                    }
                }

                const creationTime = job.metadata?.statistics?.creationTime
                    ? new Date(Number(job.metadata.statistics.creationTime)).toLocaleString()
                    : '';

                const email = job.metadata?.user_email || '';
                const query = job.metadata?.configuration?.query?.query || '';
                const shortQuery = query ? query.substring(0, 50) + (query.length > 50 ? '...' : '') : 'No query';

                const item = new JobTreeItem(
                    JobTreeItemType.job,
                    job.id || null,
                    job.id || 'Unknown Job',
                    `${state} ${creationTime ? '- ' + creationTime : ''}`,
                    vscode.TreeItemCollapsibleState.None,
                    `User: ${email}\nState: ${state}\nQuery: ${shortQuery}`
                );
                item.iconPath = new vscode.ThemeIcon(icon);
                this.cachedJobs.push(item);
            }

            if (this.nextPageToken) {
                this.cachedJobs.push(new JobTreeItem(
                    JobTreeItemType.loadMore,
                    null,
                    'Load more...',
                    '',
                    vscode.TreeItemCollapsibleState.None,
                    undefined,
                    this.nextPageToken
                ));
            }

            return this.cachedJobs;

        } catch (error) {
            vscode.window.showErrorMessage(`Failed to fetch BigQuery jobs: ${error}`);
            return [];
        }
    }
}
