import * as vscode from 'vscode';

export enum JobTreeItemType {
    job,
    loadMore
}

export class JobTreeItem extends vscode.TreeItem {
    constructor(
        public readonly treeItemType: JobTreeItemType,
        public readonly jobId: string | null,
        public readonly label: string,
        public readonly descriptionText: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly tooltipText?: string,
        public readonly pageToken?: string,
        public readonly queryText?: string
    ) {
        super(label, collapsibleState);

        switch (treeItemType) {
            case JobTreeItemType.job:
                this.contextValue = 'bq-job';
                this.iconPath = new vscode.ThemeIcon('pulse');
                this.command = { command: 'vscode-bigquery.job-open-query', title: 'Open Query', arguments: [this] };
                break;
            case JobTreeItemType.loadMore:
                this.contextValue = 'bq-job-load-more';
                this.iconPath = new vscode.ThemeIcon('add');
                this.command = { command: 'vscode-bigquery.job-load-more', title: 'Load more jobs', arguments: [this] };
                break;
        }

        this.description = descriptionText;
        if (tooltipText) {
            this.tooltip = tooltipText;
        }
    }
}
