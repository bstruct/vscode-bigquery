import * as vscode from 'vscode';
import { extensionUri } from '../extension';
import { Authentication } from '../services/authentication';

export class BigqueryAuthenticationWebviewViewProvider implements vscode.WebviewViewProvider {

    resolveWebviewView(webviewView: vscode.WebviewView, context: vscode.WebviewViewResolveContext<unknown>, token: vscode.CancellationToken): Thenable<void> | void {

        webviewView.webview.options = { enableScripts: true };

        const toolkitUri = this.getUri(webviewView.webview, extensionUri, [
            'node_modules',
            '@vscode',
            'webview-ui-toolkit',
            'dist',
            'toolkit.min.js',
        ]);

        Authentication
            .list()
            .then(result => { 

                webviewView.webview.html = `<!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="UTF-8">
                        <meta name="viewport" content="width=device-width, initial-scale=1.0">
                        <script type="module" src="${toolkitUri}"></script>
                    </head>
                    <body>
                        <vscode-data-grid id="basic-grid" generate-header="sticky" aria-label="Default"></vscode-data-grid>
                        <div>&nbsp;</div>
                        <div>
                            <div>New authentication via:</div>
                            <vscode-button appearance="secondary" onclick="vscode.postMessage("user_login")">User login</vscode-button>
                            <vscode-button appearance="secondary">Service account</vscode-button>
                        </div>
                        <div>&nbsp;</div>
                        <div>Authentication is based on the <a href="https://cloud.google.com/sdk/docs/install">gcloud CLI</a>.</div>
            
                        <script>
                            document.getElementById('basic-grid').rowsData = ${JSON.stringify(result)};
                            const vscode = acquireVsCodeApi();
                        </script>
                    </body>
                </html>`;        

            })
            .catch(error => { 

                webviewView.webview.html = `<!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="UTF-8">
                        <meta name="viewport" content="width=device-width, initial-scale=1.0">
                        <script type="module" src="${toolkitUri}"></script>
                    </head>
                    <body>
                        <div>Authentication is based on the <a href="https://cloud.google.com/sdk/docs/install">gcloud CLI</a>b, therefore, it must be installed in this computer.</div>
                        <div>Error:</div>
                        <div>${error.stderr}</div>
                    </body>
                </html>`;        

            });

    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

}