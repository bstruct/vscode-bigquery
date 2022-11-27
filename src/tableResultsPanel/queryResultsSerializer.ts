import * as vscode from 'vscode';

export class QueryResultsSerializer implements vscode.WebviewPanelSerializer {
    deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: unknown): Thenable<void> {

        


        // throw new Error('Method not implemented.');
        return new Promise((resolve, reject) => { resolve(undefined); });
    }
}