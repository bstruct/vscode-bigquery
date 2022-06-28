import * as vscode from 'vscode';
// import { BigqueryIcons } from '../bigquery-icons';
import { bigqueryIcons } from '../extension';
import * as commands from '../extension-commands';

export enum TreeItemType {
    None,
    Project,
    Dataset,
    Table,
    PartitionedTable,
    TableView,
    Routine,
    DatasetLink,
    Model
}

export class BigqueryTreeItem extends vscode.TreeItem {

    constructor(
        public readonly treeItemType: TreeItemType,

        public readonly projectId: string | null,
        public readonly datasetId: string | null,
        public readonly tableId: string | null,

        public readonly label: string,
        private readonly version: string,
        public readonly collapsibleState: vscode.TreeItemCollapsibleState,
        public readonly command?: vscode.Command
    ) {
        super(label, collapsibleState);

        switch (treeItemType) {
            case TreeItemType.Table:
                this.iconPath = bigqueryIcons.table;
                this.contextValue = 'bq-table';
                this.command = { command: commands.COMMAND_VIEW_TABLE, arguments: [this] } as vscode.Command;
                break;
            case TreeItemType.PartitionedTable:
                this.iconPath = bigqueryIcons.tablePartitioned;
                this.contextValue = 'bq-table';
                this.command = { command: commands.COMMAND_VIEW_TABLE, arguments: [this] } as vscode.Command;
                break;
            case TreeItemType.TableView:
                this.iconPath = bigqueryIcons.tableView;
                break;
            case TreeItemType.Dataset:
                this.iconPath = bigqueryIcons.dataset;
                break;
            case TreeItemType.DatasetLink:
                this.iconPath = bigqueryIcons.datasetLink;
                break;
            case TreeItemType.Routine:
                this.iconPath = bigqueryIcons.routine;
                break;
            case TreeItemType.Model:
                this.iconPath = bigqueryIcons.model;
                break;
        }

        this.description = this.version;
    }

}
