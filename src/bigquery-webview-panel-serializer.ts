import bigquery from '@google-cloud/bigquery/build/src/types';
import * as vscode from 'vscode';

export class BigQueryWebviewPanelSerializer implements vscode.WebviewPanelSerializer {

    async deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: unknown) {

        console.log(`Got state: ${state}`);

        // const extensionUri = vscode.extensions.getExtension('google.vscode-bigquery')?.extensionUri;
        // if (extensionUri === undefined) { return; }

        // webviewPanel.webview.options = {
        //     // Allow scripts in the webview
        //     enableScripts: true,

        //     localResourceRoots: [
        //         extensionUri
        //     ]
        // };

        webviewPanel.webview.html = `<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta http-equiv="Content-Security-Policy" content="default-src 'none';">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <title>test</title>
                </head>
                <body>
                    <h1>xxxx</h1>
                    <vscode-badge id="badge">2</vscode-badge>
                    
                </body>
            </html>`;


    }

}