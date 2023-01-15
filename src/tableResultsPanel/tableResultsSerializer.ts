import * as vscode from 'vscode';
import { COMMAND_VIEW_TABLE } from '../extensionCommands';
import { TableReference } from '../services/tableMetadata';

export class TableResultsSerializer implements vscode.WebviewPanelSerializer {

    deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: any): Thenable<void> {

        const titleSplit = webviewPanel.title.split('.');
        if (titleSplit.length === 3) {

            const tableReference = { projectId: titleSplit[0], datasetId: titleSplit[1], tableId: titleSplit[2], } as TableReference;

            vscode.commands.executeCommand(COMMAND_VIEW_TABLE, tableReference, webviewPanel);
        }

        // throw new Error('Method not implemented.');
        return new Promise((resolve, reject) => { resolve(undefined); });
    }
}