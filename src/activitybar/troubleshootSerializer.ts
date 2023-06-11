import * as vscode from 'vscode';

export class TroubleshootSerializer implements vscode.WebviewPanelSerializer {

    deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: any): Thenable<void> {

        webviewPanel.webview.html = TroubleshootSerializer.getTroubleshootHtml();

        return new Promise((resolve, reject) => { resolve(undefined); });
    }

    static getTroubleshootHtml(): string {

        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Troubleshoot Authentication</title>
</head>
<body>
    <h1>Troubleshoot Authentication</h1>

</body>
</html>`;

    }

}