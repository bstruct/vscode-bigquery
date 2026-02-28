import * as vscode from 'vscode';
// import { BigqueryIcons } from '../bigquery-icons';
// import { bigqueryIcons } from '../extension';
import * as commands from '../extensionCommands';
import { BigqueryIcons } from '../bigqueryIcons';

export enum BigqueryTreeItemType {
    none,
    project,
    dataset,
    table,
    partitionedTable,
    tableView,
    routine,
    datasetLink,
    model,
    tableShardGroup,
    loadMore
}

export class BigqueryTreeItem extends vscode.TreeItem {

    constructor(
        public readonly treeItemType: BigqueryTreeItemType,

        public readonly projectId: string | null,
        public readonly datasetId: string | null,
        public readonly tableId: string | null,

        public readonly label: string,
        private readonly version: string,
        private readonly pinned: boolean,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly command?: vscode.Command,
        public readonly pageOffset?: number
    ) {
        super(label, collapsibleState);

        const bigqueryIcons = new BigqueryIcons();

        switch (treeItemType) {
            case BigqueryTreeItemType.project:
                if (pinned) {
                    this.iconPath = bigqueryIcons.pinned;
                }
                this.contextValue = 'bq-gcp-project';
                break;
            case BigqueryTreeItemType.table:
                this.iconPath = bigqueryIcons.table;
                this.contextValue = 'bq-table';
                this.command = { command: commands.COMMAND_VIEW_TABLE, arguments: [this] } as vscode.Command;
                break;
            case BigqueryTreeItemType.partitionedTable:
                this.iconPath = bigqueryIcons.tablePartitioned;
                this.contextValue = 'bq-table';
                this.command = { command: commands.COMMAND_VIEW_TABLE, arguments: [this] } as vscode.Command;
                break;
            case BigqueryTreeItemType.tableView:
                this.iconPath = bigqueryIcons.tableView;
                this.contextValue = 'bq-table';
                this.command = { command: commands.COMMAND_VIEW_TABLE, arguments: [this] } as vscode.Command;
                break;
            case BigqueryTreeItemType.dataset:
                this.iconPath = bigqueryIcons.dataset;
                break;
            case BigqueryTreeItemType.datasetLink:
                this.iconPath = bigqueryIcons.datasetLink;
                break;
            case BigqueryTreeItemType.routine:
                this.iconPath = bigqueryIcons.routine;
                this.contextValue = 'bq-routine';
                break;
            case BigqueryTreeItemType.model:
                this.iconPath = bigqueryIcons.model;
                if (tableId !== null) {
                    this.contextValue = 'bq-model';
                }
                break;
            case BigqueryTreeItemType.tableShardGroup:
                this.iconPath = bigqueryIcons.tablePartitioned;
                this.contextValue = 'bq-table-shard-group';
                break;
            case BigqueryTreeItemType.loadMore:
                this.contextValue = 'bq-load-more';
                break;
        }

        this.description = this.version;
    }

}
