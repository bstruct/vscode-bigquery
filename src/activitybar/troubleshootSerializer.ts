import * as vscode from 'vscode';

export class TroubleshootSerializer implements vscode.WebviewPanelSerializer {

    deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: any): Thenable<void> {

        webviewPanel.webview.html = TroubleshootSerializer.getTroubleshootHtml();

        return new Promise((resolve, reject) => { resolve(undefined); });
    }

    static getTroubleshootHtml(): string {

        let extensionUri = vscode.extensions.getExtension('vscode-bigquery')?.extensionUri;

        if (!extensionUri) {
            extensionUri = vscode.Uri.parse('../');
        }

        // Get path to resource on disk
        const onDiskPath = vscode.Uri.joinPath(extensionUri, 'troubleshoot', 'gcloud-authentication.png');


        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Troubleshoot Authentication</title>
</head>
<body>
    <h1>Troubleshoot Authentication</h1>

    <br />
    <h2>I was granted permissions to Bigquery, but the explorer is not working</h2>
    <p>Bigquery permissions can be given at project, dataset and table level. If the GCP project(s) that you should have access are not listed in the "explorer" tab, there are a few steps that can be taken to understand the cause.</p>
    <ul>
       <li><p>Confirm that there is at least one login specified</p></li>
       <li>
        <p>Run the command <a href="https://cloud.google.com/sdk/gcloud/reference/auth/list">"gcloud auth list"</a> to confirm what is reflected on the "Authentication" tab.</p>
        <p>The information in the "Authentication" tab just reflects the gcloud commands that run in the background and logged into the "gcloud authentication" terminal tab.</p>
        <p><img src="${onDiskPath}" /></p>
       </li>
       <li>
        <p></p>
       </li>
    </ul>

    <br />
    <h2>Problems when changing users (windows)</h2>
    <p></p>

    <br />
    <h2>My problem is not listed here</h2>
    <p>Please submit a new <a href="https://github.com/bstruct/vscode-bigquery/issues" target="new">issue</a> in the github repository. Please be as detailed as possible.</p>

</body>
</html>`;

    }

}