import bigquery from '@google-cloud/bigquery/build/src/types';
import * as vscode from 'vscode';
import { writeFile } from 'fs/promises';
// import {  } from 'fs';

export class BigQueryWebviewViewProvider implements vscode.WebviewViewProvider {

    private webviewView: vscode.WebviewView | null = null;

    private textEditorResults: [key: string, value: bigquery.IGetQueryResultsResponse | null][] = [];

    constructor() {
        this.setListeners();
    }

    setListeners() {
        vscode.window.onDidChangeVisibleTextEditors((textEditors: readonly vscode.TextEditor[]) => {
            if (textEditors.length) {

                const textEditor = textEditors[0];

                const textEditorUri = textEditors[0].document.uri.toString();

                const qExistingItem = this.textEditorResults.filter(c => c[0] == textEditorUri);

                if (qExistingItem.length > 0) {
                    this.resolveHtml(textEditor, qExistingItem[0][1]);
                } else {
                    this.resolveHtml(textEditor, null);
                }

            }
        });

    }

    resolveWebviewView(webviewView: vscode.WebviewView, context: vscode.WebviewViewResolveContext<unknown>, token: vscode.CancellationToken): void | Thenable<void> {


        webviewView.webview.html = `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            </head>
            <body>
                <h1>xxxx</h1>
                <vscode-badge id="badge">1</vscode-badge>
            </body>
        </html>`;


        // if (this.webviewView === null) 
        {
            this.webviewView = webviewView;

            const extensionUri = vscode.extensions.getExtension('google.vscode-bigquery')?.extensionUri;
            if (extensionUri === undefined) { return; }

            webviewView.webview.options = {
                // Allow scripts in the webview
                enableScripts: true,

                localResourceRoots: [
                    extensionUri
                ]
            };

        }

    }

    //https://microsoft.github.io/vscode-webview-ui-toolkit/?path=/docs/library-data-grid--default

    private async resolveHtml(textEditor: vscode.TextEditor, queryResultsResponse: bigquery.IGetQueryResultsResponse | null): Promise<void> {

        if (this.webviewView === null) {
            return;
        }


        this.webviewView.webview.html = `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            </head>
            <body>
                <h1>xxxx</h1>
                <vscode-badge id="badge">1</vscode-badge>
            </body>
        </html>`;

        const textEditorUri = textEditor.document.uri.toString();

        const qExistingItem = this.textEditorResults.filter(c => c[0] == textEditorUri);

        if (qExistingItem.length > 0) {
            qExistingItem[0][1] = queryResultsResponse;
        } else {
            this.textEditorResults.push([textEditorUri, queryResultsResponse]);
        }

        if (queryResultsResponse === null) {


        } else {

            // // Local path to main script run in the webview
            const extensionUri = vscode.extensions.getExtension('google.vscode-bigquery')?.extensionUri;
            if (extensionUri === undefined) { return; }

            // const scriptPathOnDisk = vscode.Uri.joinPath(extensionUri, 'bq-query-results.js');
            // await writeFile(scriptPathOnDisk.fsPath, `
            //     document.getElementById('basic-grid').rowsData = [
            //         {Header1: 'Cell Data', Header2: 'Cell Data', Header3: 'Cell Data', Header4: 'Cell Data'},
            //         {Header1: 'Cell Data', Header2: 'Cell Data', Header3: 'Cell Data', Header4: 'Cell Data'},
            //         {Header1: 'Cell Data', Header2: 'Cell Data', Header3: 'Cell Data', Header4: 'Cell Data'},
            //     ];
            // `);

            // // And the uri we use to load this script in the webview
            // const scriptUri = this.webviewView.webview.asWebviewUri(scriptPathOnDisk);

            // // Use a nonce to only allow specific scripts to be run
            // const nonce = this.getNonce();

            const toolkitUri = this.getUri(this.webviewView.webview, extensionUri, [
                'dist',
                'toolkit.js',
            ]);

            this.webviewView.webview.html = `<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <script type="module" src="${toolkitUri}"></script>
                </head>
                <body>
                <main>
                    <vscode-badge id="badge">1</vscode-badge>

                    <vscode-data-grid aria-label="Basic">
                        <vscode-data-grid-row row-type="header">
                            <vscode-data-grid-cell cell-type="columnheader" grid-column="1">Header 1</vscode-data-grid-cell>
                            <vscode-data-grid-cell cell-type="columnheader" grid-column="2">Header 2</vscode-data-grid-cell>
                            <vscode-data-grid-cell cell-type="columnheader" grid-column="3">Header 3</vscode-data-grid-cell>
                            <vscode-data-grid-cell cell-type="columnheader" grid-column="3">Header 4</vscode-data-grid-cell>
                        </vscode-data-grid-row>
                        <vscode-data-grid-row>
                            <vscode-data-grid-cell grid-column="1">Cell Data</vscode-data-grid-cell>
                            <vscode-data-grid-cell grid-column="2">Cell Data</vscode-data-grid-cell>
                            <vscode-data-grid-cell grid-column="3">Cell Data</vscode-data-grid-cell>
                            <vscode-data-grid-cell grid-column="4">Cell Data</vscode-data-grid-cell>
                        </vscode-data-grid-row>
                        <vscode-data-grid-row>
                            <vscode-data-grid-cell grid-column="1">Cell Data</vscode-data-grid-cell>
                            <vscode-data-grid-cell grid-column="2">Cell Data</vscode-data-grid-cell>
                            <vscode-data-grid-cell grid-column="3">Cell Data</vscode-data-grid-cell>
                            <vscode-data-grid-cell grid-column="4">Cell Data</vscode-data-grid-cell>
                        </vscode-data-grid-row>
                        <vscode-data-grid-row>
                            <vscode-data-grid-cell grid-column="1">Cell Data</vscode-data-grid-cell>
                            <vscode-data-grid-cell grid-column="2">Cell Data</vscode-data-grid-cell>
                            <vscode-data-grid-cell grid-column="3">Cell Data</vscode-data-grid-cell>
                            <vscode-data-grid-cell grid-column="4">Cell Data</vscode-data-grid-cell>
                        </vscode-data-grid-row>
                    </vscode-data-grid>

                    <vscode-data-grid id="basic-grid" aria-label="Default"></vscode-data-grid>
                </main>

                    <script>
                         document.getElementById('basic-grid').rowsData = [
                                 {Header1: 'Cell Data', Header2: 'Cell Data', Header3: 'Cell Data', Header4: 'Cell Data'},
                                 {Header1: 'Cell Data', Header2: 'Cell Data', Header3: 'Cell Data', Header4: 'Cell Data'},
                                 {Header1: 'Cell Data', Header2: 'Cell Data', Header3: 'Cell Data', Header4: 'Cell Data'},
                             ];
                       
                    </script>
                </body>
            </html>`;

        }

    }

    public setNewJob(textEditor: vscode.TextEditor, promisedJob: Promise<bigquery.IGetQueryResultsResponse>) {

        if (this.webviewView === null) {
            return;
        }

        if (!this.webviewView.visible) {
            this.webviewView.show();
        }

        promisedJob.then((job) => {

            this.resolveHtml(textEditor, job);

        });

    }

    private getUri = (
        webview: vscode.Webview,
        extensionUri: vscode.Uri,
        pathList: string[],
    ): vscode.Uri => {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    };

    private getNonce() {
        let text = '';
        const possible = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
        for (let i = 0; i < 32; i++) {
            text += possible.charAt(Math.floor(Math.random() * possible.length));
        }
        return text;
    }

}