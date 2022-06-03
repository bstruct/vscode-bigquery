import * as vscode from 'vscode';
import { extensionUri } from '../extension';
import { Job, JobResponse, QueryResultsOptions } from '@google-cloud/bigquery';
import { SimpleQueryRowsResponseError } from '../bigquery/simple_query_rows_response_error';
import { ResultsGrid } from './results_grid';

//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class ResultsGridRender {

    private webView: vscode.Webview;

    private disposableEvent: vscode.Disposable | null = null;

    constructor(webView: vscode.Webview) {
        this.webView = webView;
    }

    public render(
        jobsPromise: Promise<Job[]>,
        startIndex: number = 0,
        maxResults: number = 50
    ) {

        const toolkitUri = this.getUri(this.webView, extensionUri, [
            "node_modules",
            "@vscode",
            "webview-ui-toolkit",
            "dist",
            "toolkit.min.js",
        ]);

        //dispose event regardless of successful query or not
        if (this.disposableEvent) { this.disposableEvent.dispose(); }

        //set waiting gif
        this.webView.html = this.getWaitingHtml(toolkitUri);

        jobsPromise
            .then(async (jobs) => {

                const codiconsUri = this.getUri(this.webView, extensionUri, [
                    'node_modules',
                    '@vscode/codicons',
                    'dist',
                    'codicon.css']
                );

                const [html, totalRows] = await this.getResultsHtml(toolkitUri, codiconsUri, jobs, startIndex, maxResults);
                this.webView.html = html;

                //in case that the search result needs pagination, this event is enabled
                this.disposableEvent = this.webView.onDidReceiveMessage(this.listenerOnDidReceiveMessage, [this, jobsPromise, startIndex, maxResults, totalRows]);

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

    /* 
    * weird response because the total rows are only known in the `getQueryResults` response
    */
    private async getResultsHtml(
        toolkitUri: vscode.Uri,
        codiconsUri: vscode.Uri,
        jobs : Job[],
        startIndex: number,
        maxResults: number,
    ): Promise<[string, number]> {

        const job = jobs[0];

        const queryResultOptions: QueryResultsOptions = { startIndex: startIndex.toString(), maxResults: maxResults };
        const queryRowsResponse = await job.getQueryResults(queryResultOptions);

        const totalRows: number = Number(queryRowsResponse[2]?.totalRows || 0);

        return [`<!DOCTYPE html>
        <html lang="en">
        	<head>
        		<meta charset="UTF-8">
        		<meta name="viewport" content="width=device-width, initial-scale=1.0">
        		<script type="module" src="${toolkitUri}"></script>
                <link href="${codiconsUri}" rel="stylesheet" />
        	</head>
        	<body>
                ${(new ResultsGrid(queryRowsResponse, startIndex, maxResults))}
                <script>
                    const vscode = acquireVsCodeApi();
                </script>
        	</body>
        </html>`,
            totalRows
        ];

    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

    /* This function will run as an event triggered when the JS on the webview triggers
     * the `postMessage` method
    */
    listenerOnDidReceiveMessage(message: any): void {

        const resultsGridRender: ResultsGridRender = (this as any)[0];
        const jobResponsePromise: Promise<Job[]> = (this as any)[1];

        const startIndex: number = (this as any)[2];
        const maxResults: number = (this as any)[3];
        const totalRows: number = (this as any)[4];

        switch (message) {
            case 'first_page':
                resultsGridRender.render(jobResponsePromise, 0, maxResults);
                break;
            case 'previous_page':
                resultsGridRender.render(jobResponsePromise, startIndex - maxResults, maxResults);
                break;
            case 'next_page':
                resultsGridRender.render(jobResponsePromise, startIndex + maxResults, maxResults);
                break;
            case 'last_page':
                const lastPageStartIndex = (Math.ceil(totalRows / maxResults) - 1) * maxResults;
                resultsGridRender.render(jobResponsePromise, lastPageStartIndex, maxResults);
                break;
        }

    }

}
