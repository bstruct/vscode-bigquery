import * as vscode from 'vscode';
import { extensionUri } from '../extension';
import { JobResponse, QueryResultsOptions } from '@google-cloud/bigquery';
import { SimpleQueryRowsResponseError } from '../bigquery/simple_query_rows_response_error';
import { ResultsGrid } from './results_grid';

//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class ResultsGridRender {

    private webView: vscode.Webview;

    constructor(webView: vscode.Webview) {
        this.webView = webView;
    }

    public render(
        jobResponse: Promise<JobResponse>,
        maxResults: number = 10,
        startIndex: number = 0,
        setEventListener: boolean = true
    ) {

        const toolkitUri = this.getUri(this.webView, extensionUri, [
            "node_modules",
            "@vscode",
            "webview-ui-toolkit",
            "dist",
            "toolkit.min.js",
        ]);

        // this.webView.onDidReceiveMessage.
        if(setEventListener){
            this.webView.onDidReceiveMessage(this.listenerOnDidReceiveMessage, [this, jobResponse]);
        }

        this.webView.html = this.getWaitingHtml(toolkitUri);

        jobResponse
            .then(async (result) => {

                const codiconsUri = this.getUri(this.webView, extensionUri, [
                    'node_modules',
                    '@vscode/codicons',
                    'dist',
                    'codicon.css']
                );

                this.webView.html = await this.getResultsHtml(toolkitUri, codiconsUri, result, startIndex, maxResults);

            })
            .catch(exception => {

                this.webView.html = this.getExceptionHtml(toolkitUri, exception);

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

    private async getResultsHtml(
        toolkitUri: vscode.Uri,
        codiconsUri: vscode.Uri,
        jobResponse: JobResponse,
        startIndex: number = 0,
        maxResults: number = 10,
    ) {

        const queryResultOptions: QueryResultsOptions = { startIndex: startIndex.toString(), maxResults: maxResults };
        const queryRowsResponse = await jobResponse[0].getQueryResults(queryResultOptions);

        return `<!DOCTYPE html>
        <html lang="en">
        	<head>
        		<meta charset="UTF-8">
        		<meta name="viewport" content="width=device-width, initial-scale=1.0">
        		<script type="module" src="${toolkitUri}"></script>
                <link href="${codiconsUri}" rel="stylesheet" />
        	</head>
        	<body>
                ${(new ResultsGrid(queryRowsResponse))}
                <script>
                    const vscode = acquireVsCodeApi();
                </script>
        	</body>
        </html>`;

    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

    /* This function will run as an event triggered when the JS on the webview triggers
     * the `postMessage` method
    */
    async listenerOnDidReceiveMessage(message: any): Promise<void> {

        const resultsGridRender: ResultsGridRender = (this as any)[0];
        const jobResponsePromise: Promise<JobResponse> = (this as any)[1];
        const jobResponse: JobResponse = await jobResponsePromise;
        
        // const job: bigquery.IJob = jobResponse[1]
        // const queryResults: bigquery.IGetQueryResultsResponse = jobResponse[1];

        // vscode.window.showInformationMessage(message);


        switch (message) {
            case 'first_page':

                resultsGridRender.render(jobResponsePromise, 10, 0, false);

                break;
            case 'previous_page':

                break;
            case 'next_page':

                resultsGridRender.render(jobResponsePromise, 10, 10, false);
        
                break;
            case 'last_page':

                break;
        }

    }

}