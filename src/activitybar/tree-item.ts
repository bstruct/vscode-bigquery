import * as vscode from 'vscode';
import { BigqueryIcons } from '../bigquery-icons';

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

        const bigqueryIcons = new BigqueryIcons();

        switch (treeItemType) {
            case TreeItemType.Table:
                this.iconPath = bigqueryIcons.Table;
                break;
            case TreeItemType.PartitionedTable:
                this.iconPath = bigqueryIcons.TablePartitioned;
                break;
            case TreeItemType.TableView:
                this.iconPath = bigqueryIcons.TableView;
                break;
            case TreeItemType.Dataset:
                this.iconPath = bigqueryIcons.Dataset;
                break;
            case TreeItemType.DatasetLink:
                this.iconPath = bigqueryIcons.DatasetLink;
                break;
            case TreeItemType.Routine:
                this.iconPath = bigqueryIcons.Routine;
                break;
            case TreeItemType.Model:
                this.iconPath = bigqueryIcons.Model;
                break;
        }

        this.description = this.version;
    }


    contextValue = 'dependency';
}
