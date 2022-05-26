import * as vscode from 'vscode';

export class WebviewViewProvider implements vscode.WebviewViewProvider {

    public webviewView: vscode.WebviewView | null = null;

    resolveWebviewView(webviewView: vscode.WebviewView, context: vscode.WebviewViewResolveContext<unknown>, token: vscode.CancellationToken): void | Thenable<void> {

        console.info(context);

        webviewView.webview.options = { enableScripts: true };

        this.webviewView = webviewView;

    }

}