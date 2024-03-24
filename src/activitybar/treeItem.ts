import * as vscode from 'vscode';
// import { BigqueryIcons } from '../bigquery-icons';
// import { bigqueryIcons } from '../extension';
import * as commands from '../extensionCommands';
import { BigqueryIcons } from '../bigqueryIcons';

export enum TreeItemType {
    none,
    project,
    dataset,
    table,
    partitionedTable,
    tableView,
    routine,
    datasetLink,
    model
}

export class BigqueryTreeItem extends vscode.TreeItem {

    constructor(
        public readonly treeItemType: TreeItemType,

        public readonly projectId: string | null,
        public readonly datasetId: string | null,
        public readonly tableId: string | null,

        public readonly label: string,
        private readonly version: string,
        private readonly pinned: boolean,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly command?: vscode.Command
    ) {
        super(label, collapsibleState);

        const bigqueryIcons = new BigqueryIcons();

        switch (treeItemType) {
            case TreeItemType.project:
                if (pinned) {
                    this.iconPath = bigqueryIcons.pinned;
                }
                this.contextValue = 'bq-gcp-project';
                break;
            case TreeItemType.table:
                this.iconPath = bigqueryIcons.table;
                this.contextValue = 'bq-table';
                this.command = { command: commands.COMMAND_VIEW_TABLE, arguments: [this] } as vscode.Command;
                break;
            case TreeItemType.partitionedTable:
                this.iconPath = bigqueryIcons.tablePartitioned;
                this.contextValue = 'bq-table';
                this.command = { command: commands.COMMAND_VIEW_TABLE, arguments: [this] } as vscode.Command;
                break;
            case TreeItemType.tableView:
                this.iconPath = bigqueryIcons.tableView;
                this.contextValue = 'bq-table';
                this.command = { command: commands.COMMAND_VIEW_TABLE, arguments: [this] } as vscode.Command;
                break;
            case TreeItemType.dataset:
                this.iconPath = bigqueryIcons.dataset;
                break;
            case TreeItemType.datasetLink:
                this.iconPath = bigqueryIcons.datasetLink;
                break;
            case TreeItemType.routine:
                this.iconPath = bigqueryIcons.routine;
                this.contextValue = 'bq-routine';
                break;
            case TreeItemType.model:
                this.iconPath = bigqueryIcons.model;
                break;
        }

        this.description = this.version;
    }

}
