import * as vscode from 'vscode';
import { extensionUri } from '../extension';

export class BigqueryAuthenticationWebviewViewProvider implements vscode.WebviewViewProvider {

    resolveWebviewView(webviewView: vscode.WebviewView, context: vscode.WebviewViewResolveContext<unknown>, token: vscode.CancellationToken): Thenable<void> | void {

        webviewView.webview.options = { enableScripts: true };

        const toolkitUri = this.getUri(webviewView.webview, extensionUri, [
            "node_modules",
            "@vscode",
            "webview-ui-toolkit",
            "dist",
            "toolkit.min.js",
        ]);

        //https://cloud.google.com/sdk/docs/cheatsheet#credentials
        
        webviewView.webview.html = `<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="UTF-8">
				<meta name="viewport" content="width=device-width, initial-scale=1.0">
				<script type="module" src="${toolkitUri}"></script>
			</head>
			<body>
            <h1>test</h1>
            <vscode-button appearance="secondary">Button Text</vscode-button>
			</body>
		</html>`;

    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

}