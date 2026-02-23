import * as vscode from 'vscode';
import { getExtensionUri } from '../extension';
import { COMMAND_DOWNLOAD_CSV, COMMAND_DOWNLOAD_JSONL, COMMAND_SEND_PUBSUB } from '../extensionCommands';
import { ResultsGridRenderRequestV2 } from './resultsGridRenderRequestV2';

//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class ResultsGridRender {

    private webViewPanel: vscode.WebviewPanel;
    private diagnosticsAttached: boolean = false;

    constructor(webViewPanel: vscode.WebviewPanel) {
        this.webViewPanel = webViewPanel;
        this.attachDiagnostics();
    }

    private attachDiagnostics(): void {
        if (this.diagnosticsAttached) {
            return;
        }
        this.diagnosticsAttached = true;

        this.webViewPanel.onDidChangeViewState(e => {
            console.log(`[vscode-bigquery] results panel viewState title="${e.webviewPanel.title}" visible=${e.webviewPanel.visible} active=${e.webviewPanel.active}`);
        });

        this.webViewPanel.onDidDispose(() => {
            console.log(`[vscode-bigquery] results panel disposed title="${this.webViewPanel.title}"`);
        });
    }

    public static executeCommand(c: any) {
        if ((c as any).command) {
            const command = (c as any).command;
            const data = {
                tableReference: (c as any).table_reference,
                jobReference: (c as any).job_reference,
                command: command,
            };

            switch (command) {
                case "download_csv": { vscode.commands.executeCommand(COMMAND_DOWNLOAD_CSV, data); }
                case "download_jsonl": { vscode.commands.executeCommand(COMMAND_DOWNLOAD_JSONL, data); }
                case "send_pubsub": { vscode.commands.executeCommand(COMMAND_SEND_PUBSUB, data); }
            }
        }
    }

    public render1(): Promise<boolean> {

        const extensionUri = getExtensionUri();

        const gridJs = this.getUri(this.webViewPanel.webview, extensionUri, [
            'resources',
            'grid.js']
        );

        const gridCss = this.getUri(this.webViewPanel.webview, extensionUri, [
            'resources',
            'grid.css']
        );

        return new Promise((resolve, reject) => {

            const timer = setTimeout(() => {
                reject(null);
            }, 10 * 1000);

            this.webViewPanel.webview.onDidReceiveMessage(c => {
                if ((c as any).command === 'load_complete') {
                    console.log(`[vscode-bigquery] webview load_complete received title="${this.webViewPanel.title}"`);
                    clearTimeout(timer);
                    resolve(true);
                } else {
                    ResultsGridRender.executeCommand(c);
                }
            });

            this.webViewPanel.webview.html = `<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <link rel="stylesheet" href="${gridCss}">
                    <script>
                        const vscode = acquireVsCodeApi();    
                    </script>
                </head>
                <body style="padding:0;">
                    <div id="q1"></div>
                    <script type="module" src="${gridJs}"></script>
                    <script>
                        vscode.postMessage({command:'load_complete'});
                    </script>
                </body>
            </html>`;
        });
    }

    public render2() {

        const extensionUri = getExtensionUri();

        const gridJs = this.getUri(this.webViewPanel.webview, extensionUri, [
            'resources',
            'grid.js']
        );

        const gridCss = this.getUri(this.webViewPanel.webview, extensionUri, [
            'resources',
            'grid.css']
        );

        this.webViewPanel.webview.onDidReceiveMessage(c => {
            if ((c as any).command !== 'load_complete') {
                ResultsGridRender.executeCommand(c);
            } else {
                console.log(`[vscode-bigquery] webview load_complete received (render2) title="${this.webViewPanel.title}"`);
            }
        });

        this.webViewPanel.webview.html = `<!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="UTF-8">
                <link rel="stylesheet" href="${gridCss}">
                <script>
                    const vscode = acquireVsCodeApi();    
                </script>
            </head>
            <body style="padding:0;">
                <div id="q1"></div>
                <script type="module" src="${gridJs}"></script>
                <script>
                    vscode.postMessage({command:'load_complete'});                    
                </script>
            </body>
        </html>`;

    }

    public postMessage(message: ResultsGridRenderRequestV2): Thenable<boolean> {
        return this.webViewPanel.webview.postMessage(message);
    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

    reveal(viewColumn?: vscode.ViewColumn, preserveFocus?: boolean): void {
        this.webViewPanel.reveal(viewColumn, preserveFocus);
    }

}