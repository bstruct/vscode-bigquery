import * as vscode from 'vscode';
import { getExtensionUri } from '../extension';

export class TroubleshootSerializer implements vscode.WebviewPanelSerializer {

    deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: any): Thenable<void> {

        // let extensionUri = vscode.extensions.getExtension('bstruct.vscode-bigquery')?.extensionUri;

        webviewPanel.webview.html = TroubleshootSerializer.getTroubleshootHtml(webviewPanel);

        return new Promise((resolve, reject) => { resolve(undefined); });
    }

    static getTroubleshootHtml(webviewPanel: vscode.WebviewPanel): string {

        // let extensionUri = vscode.extensions.getExtension('bstruct.vscode-bigquery')?.extensionUri;
        let extensionUri = getExtensionUri();

        if (!extensionUri) {
            extensionUri = vscode.Uri.parse('../');
        }

        // Get path to resource on disk
        const onDiskPath = webviewPanel.webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, 'media', 'gcloud-authentication.png'));

        return `<!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Troubleshoot Authentication</title>
        </head>
        <body>
            <h1>Troubleshoot Google Cloud Authentication</h1>
        
            <p>The authentication in this visual studio extension is nothing but a reflection on the <a href="https://cloud.google.com/sdk/docs/install">Google Cloud CLI</a> setup in the computer.</p>
            <p><img src="${onDiskPath}" /></p>
            <p>Please make sure that you have a valid user or service account as form of authentication. Make sure that is Active.</p>
            <p>If the setup of the authentication was done outside of the command line of visual studio code, please restart visual studio code.</p>
            <p>Refresh the authentication and explorer using the refresh button on the top right of that section.</p>
            <br />
            <h2>I was granted permissions to Bigquery, but the explorer doesn't list anything.</h2>
            <p>Bigquery permissions can be given at project, dataset and table level. If the GCP project(s) that you should have access are not listed in the "explorer" tab, there are a few steps that can be taken to understand the cause.</p>
            <p>To run queries, there must be one GCP project that will be billed with that query. View tables is not charged by GCP.</p>
            <ul>
               <li>
                <p>To directly add a project id that the user has access to, please configure this in the settings.</p>
                <p>This step is needed when the permissions were given at dataset level, not project. Therefore is not possible to list the datasets in the project.</p>
                <p>If problems persist, this step will at least provide a much more detail error message.</p>
               </li>
               <li>
                <p>Is possible to add a table directly in the settings. Configure this when you were given table access, not dataset nor project.</p>
                <p></p>
               </li>
            </ul>
        
            <br />
            <h2>Problems when changing users (windows)</h2>
            <p>When changing adding or removing users in the \`Authentication\` tab, sometimes, in Windows, the new configuration might not be reflected.</p>
            <p>This happens because the credentials are cached and the <a href="https://cloud.google.com/sdk/docs/install">Google Cloud CLI</a> fails to update them.</p>
            <p>The suggested work around is to delete the cached credentials and re-authenticate</p>
            <p>To do this, navigate to the folder <i>C:\\Users\\%username%\\AppData\\Roaming\\gcloud</i> and delete the file <i>application_default_credentials.json</i>.</p>
        
            <br />
            <h2>My problem is not listed here</h2>
            <p>Please submit a new <a href="https://github.com/bstruct/vscode-bigquery/issues" target="new">issue</a> in the github repository. Please be as detailed as possible.</p>
        
        </body>
        </html>`;

    }

}