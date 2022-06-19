import path = require('path');
import * as vscode from 'vscode';
import { BigqueryIcons } from '../bigquery-icons';
import { extensionUri } from '../extension';

export enum TreeItemType {
    None,
    Project,
    Dataset,
    Table,
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

        // this.tooltip = `${this.label}-${this.version}`;

        // vscode.workspace.

        const bigqueryIcons = new BigqueryIcons();

        //https://code.visualstudio.com/api/references/icons-in-labels#icon-listing
        switch (treeItemType) {
            case TreeItemType.Table:
                this.iconPath = bigqueryIcons.Table;
                break;
            case TreeItemType.Dataset:
                this.iconPath = bigqueryIcons.Dataset;
                break;
        }

        this.description = this.version;
    }


    contextValue = 'dependency';
}
