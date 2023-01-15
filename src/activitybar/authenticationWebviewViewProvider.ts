import * as vscode from 'vscode';
import { extensionUri } from '../extension';
import * as commands from '../extensionCommands';
import { Authentication } from '../services/authentication';
import { AuthenticationGrid } from './authenticationGrid';

export class BigqueryAuthenticationWebviewViewProvider implements vscode.WebviewViewProvider {

    private disposableEvent: vscode.Disposable | null = null;
    public webviewView: vscode.WebviewView | null = null;
    private context: vscode.WebviewViewResolveContext<unknown> | null = null;
    private token: vscode.CancellationToken | null = null;

    resolveWebviewView(webviewView: vscode.WebviewView, context: vscode.WebviewViewResolveContext<unknown>, token: vscode.CancellationToken, forceShowConsole: boolean = false): Thenable<void> | void {

        this.webviewView = webviewView;
        this.context = context;
        this.token = token;

        webviewView.webview.options = { enableScripts: true };

        //dispose event regardless of successful query or not
        if (this.disposableEvent) { this.disposableEvent.dispose(); }

        const toolkitUri = this.getUri(webviewView.webview, extensionUri, [
            'resources',
            'toolkit.min.js',
        ]);

        //in case that the search result needs pagination, this event is enabled
        this.disposableEvent = webviewView.webview.onDidReceiveMessage(this.listenerOnDidReceiveMessage);

        Authentication
            .list(forceShowConsole)
            .then(result => {

                webviewView.webview.html = `<!DOCTYPE html>
                <html lang="en">
                    <head>
                        <meta charset="UTF-8">
                        <meta name="viewport" content="width=device-width, initial-scale=1.0">
                        <script type="module" src="${toolkitUri}"></script>
                    </head>
                    <body>
                        ${(new AuthenticationGrid(result))}
                        <div>&nbsp;</div>
                        <div>
                            <div>New authentication via:</div>
                            <vscode-button appearance="secondary" onclick="vscode.postMessage('user_login')">User login</vscode-button>
                            <vscode-button appearance="secondary" onclick="vscode.postMessage('user_login_drive')">User login + GDrive</vscode-button>
                            <vscode-button appearance="secondary" onclick="vscode.postMessage('service_account_login')">Service account</vscode-button>
                        </div>
                        <div>&nbsp;</div>
                        <div>Authentication is based on the <a href="https://cloud.google.com/sdk/docs/install">gcloud CLI</a>.</div>
                        <div>&nbsp;</div>
                        <div>
                            <div>Still running into authentication issues? Please run: </div>
                            <vscode-button appearance="secondary" onclick="vscode.postMessage('gcloud_init')">gcloud init</vscode-button>
                        </div>
                        <script>
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
                        <div>Authentication is based on the <a href="https://cloud.google.com/sdk/docs/install">gcloud CLI</a>, therefore, it must be installed in this computer.</div>
                        <div>Error:</div>
                        <div>${error.stderr}</div>
                    </body>
                </html>`;

            });

    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

    /* This function will run as an event triggered when the JS on the webview triggers
     * the `postMessage` method
    */
    listenerOnDidReceiveMessage(message: any): void {

        switch (message.command || message) {
            case 'user_login':
                vscode.commands.executeCommand(commands.COMMAND_USER_LOGIN);
                break;
            case 'user_login_drive':
                vscode.commands.executeCommand(commands.COMMAND_USER_LOGIN_WITH_DRIVE);
                break;
            case 'gcloud_init':
                vscode.commands.executeCommand(commands.COMMAND_GCLOUD_INIT);
                break;
            case 'service_account_login':
                vscode.commands.executeCommand(commands.COMMAND_SERVICE_ACCOUNT_LOGIN);
                break;
            case 'activate':
                Authentication.activate(message.value)
                    .then(result => {
                        vscode.commands.executeCommand(commands.COMMAND_AUTHENTICATION_REFRESH);
                    });
                break;
            case 'revoke':
                Authentication.revoke(message.value)
                    .then(result => {
                        vscode.commands.executeCommand(commands.COMMAND_AUTHENTICATION_REFRESH);
                    });
                break;
            default:
                console.error(`Unexpected message "${message}"`);
        }

    }

    refresh() {
        if (this.webviewView !== null && this.context !== null && this.token !== null) {
            this.resolveWebviewView(this.webviewView, this.context, this.token, true);
        }
    }

}