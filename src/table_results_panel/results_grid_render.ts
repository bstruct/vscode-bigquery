import * as vscode from 'vscode';
import { extensionUri } from '../extension';
import { SimpleQueryRowsResponse } from '@google-cloud/bigquery';
import { SimpleQueryRowsResponseError } from '../bigquery/simple_query_rows_response_error';
import { ResultsGrid } from './results_grid';
import { BigQueryQueryRunner } from '../bigquery/bigquery-query-runner';

//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class ResultsGridRender {

    private _webView: vscode.Webview | null = null;
    private _queryResponse: SimpleQueryRowsResponse | null = null;

    public render(webView: vscode.Webview, queryResponse: Promise<SimpleQueryRowsResponse>) {

        this._webView = webView;

        const toolkitUri = this.getUri(webView, extensionUri, [
            "node_modules",
            "@vscode",
            "webview-ui-toolkit",
            "dist",
            "toolkit.min.js",
        ]);

        webView.onDidReceiveMessage(this.listenerOnDidReceiveMessage);

        webView.html = this.getWaitingHtml(toolkitUri);

        queryResponse
            .then(result => {

                this._queryResponse = result;

                const codiconsUri = this.getUri(webView, extensionUri, [
                    'node_modules',
                    '@vscode/codicons',
                    'dist',
                    'codicon.css']
                );

                webView.html = this.getResultsHtml(toolkitUri, codiconsUri, result);

            })
            .catch(exception => {

                webView.html = this.getExceptionHtml(toolkitUri, exception);

            });

    }

    private getWaitingHtml(toolkitUri: vscode.Uri): string {

        return `<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="UTF-8">
				<meta name="viewport" content="width=device-width, initial-scale=1.0">
				<script type="module" src="${toolkitUri}"></script>
			</head>
			<body>
                <vscode-progress-ring></vscode-progress-ring>
			</body>
		</html>`;

    }

    private getExceptionHtml(toolkitUri: vscode.Uri, exception: any): string {

        if (exception.errors) {

            const errors = (exception as SimpleQueryRowsResponseError).errors;

            const rows = JSON.stringify(errors.map(c => (
                {
                    "message": c.message,
                    "reason": c.reason,
                    "locationType": c.locationType
                }
            )));

            return `<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <script type="module" src="${toolkitUri}"></script>
                </head>
                <body>
                <vscode-data-grid id="basic-grid" generate-header="sticky" aria-label="Default"></vscode-data-grid>
    
                <script>
                    document.getElementById('basic-grid').rowsData = ${rows};
                </script>
                </body>
            </html>`;

        } else {

            const rows = JSON.stringify([{ message: exception.message, stack: exception.stack }]);

            return `<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <script type="module" src="${toolkitUri}"></script>
                </head>
                <body>
                <vscode-data-grid id="basic-grid" generate-header="sticky" aria-label="Default"></vscode-data-grid>
    
                <script>
                    document.getElementById('basic-grid').rowsData = ${rows};
                </script>
                </body>
            </html>`;

        }

    }

    private getResultsHtml(toolkitUri: vscode.Uri, codiconsUri: vscode.Uri, results: SimpleQueryRowsResponse) {

        return `<!DOCTYPE html>
        <html lang="en">
        	<head>
        		<meta charset="UTF-8">
        		<meta name="viewport" content="width=device-width, initial-scale=1.0">
        		<script type="module" src="${toolkitUri}"></script>
                <link href="${codiconsUri}" rel="stylesheet" />
        	</head>
        	<body>
                ${(new ResultsGrid(results))}
                <script>
                    const vscode = acquireVsCodeApi();
                </script>
        	</body>
        </html>`;

    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

    listenerOnDidReceiveMessage(message: any) {

        if (this._webView == null) { return; }

        debugger;

        vscode.window.showInformationMessage(message);

        const bqRunner = new BigQueryQueryRunner();

        switch (message) {
            case 'first_page':

                break;
            case 'previous_page':

                break;
            case 'next_page':

                // bqRunner.runQuery(, 10,)

                break;
            case 'last_page':

                break;
        }

    }

}