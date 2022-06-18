import path = require('path');
import * as vscode from 'vscode';
import { ThemeIcons } from 'vscode-ext-codicons';

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

        this.tooltip = `${this.label}-${this.version}`;

        // vscode.workspace.

        this.iconPath = new vscode.ThemeIcon('pencil');

        // this.iconPath = {
        //     light: path.join(__filename, '..', '..', 'resources', 'light', 'dependency.svg'),
        //     dark: path.join(__filename, '..', '..', 'resources', 'dark', 'dependency.svg')
        // };

        // this.iconPath = {
        //     light: path.join('resources', 'light', 'dependency.svg'),
        //     dark: path.join('resources', 'dark', 'dependency.svg')
        // };

        this.description = this.version;
    }


    contextValue = 'dependency';
}
