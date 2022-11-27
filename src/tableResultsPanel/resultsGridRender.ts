import * as vscode from 'vscode';
import bigquery from '@google-cloud/bigquery/build/src/types';
import { extensionUri, reporter } from '../extension';
import { Job, QueryResultsOptions, Table } from '@google-cloud/bigquery';
import { SimpleQueryRowsResponseError } from '../services/simpleQueryRowsResponseError';
import { ResultsGrid } from './resultsGrid';
import { ResultsGridRenderRequest } from './resultsGridRenderRequest';
import { TableGridRenderRequest } from './tableGridRenderRequest';
import { DownloadCsvRequest } from './downloadCsvRequest';
import { request } from 'http';
import { DownloadCsv } from './downloadCsv';

//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class ResultsGridRender {

    private webView: vscode.Webview;

    private disposableEvent: vscode.Disposable | null = null;

    constructor(webView: vscode.Webview) {
        this.webView = webView;
    }

    public render(request: ResultsGridRenderRequest) {

        try {
            //dispose event regardless of successful query or not
            if (this.disposableEvent) { this.disposableEvent.dispose(); }

            //set waiting gif
            this.webView.html = this.getWaitingHtml();

            request.jobsPromise
                .then(async (jobs) => {

                    const [html, totalRows] = await this.getResultsHtml(jobs, request.startIndex, request.maxResults, request.jobIndex, request.openInTabVisible);
                    this.webView.html = html;

                    //in case that the search result needs pagination, this event is enabled
                    this.disposableEvent = this.webView.onDidReceiveMessage(this.listenerResultsOnDidReceiveMessage, [this, request.jobsPromise, request.startIndex, request.maxResults, totalRows, request.jobIndex, request.openInTabVisible]);

                })
                .catch(exception => {
                    this.webView.html = this.getExceptionHtml(exception);
                });

        } catch (error: any) {
            vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
        }
    }

    public async renderTable(request: TableGridRenderRequest) {

        try {

            //dispose event regardless of successful query or not
            if (this.disposableEvent) { this.disposableEvent.dispose(); }

            //set waiting gif
            this.webView.html = this.getWaitingHtml();

            const [html, totalRows] = await this.getTableHtml(request.table, request.startIndex, request.maxResults, request.openInTabVisible);
            this.webView.html = html;

            //in case that the search result needs pagination, this event is enabled
            this.disposableEvent = this.webView.onDidReceiveMessage(this.listenerTableOnDidReceiveMessage, [this, request.table, request.startIndex, request.maxResults, totalRows, request.jobIndex, request.openInTabVisible]);

        } catch (error: any) {
            vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
        }
    }

    private getWaitingHtml(): string {

        const toolkitUri = this.getUri(this.webView, extensionUri, [
            "resources",
            "toolkit.min.js",
        ]);

        return `<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="UTF-8">
				<meta name="viewport" content="width=device-width, initial-scale=1.0">
				<script type="module" src="${toolkitUri}"></script>
                <script>
                    const vscode = acquireVsCodeApi();
                    vscode.setState({ value: 112222 });
                </script>
			</head>
			<body>
                <vscode-progress-ring></vscode-progress-ring>
			</body>
		</html>`;

    }

    private getExceptionHtml(exception: any): string {

        const toolkitUri = this.getUri(this.webView, extensionUri, [
            "resources",
            "toolkit.min.js",
        ]);

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
        jobs: Job[],
        startIndex: number,
        maxResults: number,
        jobIndex: number,
        openInTabVisible: boolean
    ): Promise<[string, number]> {

        const job = jobs[jobIndex];
        const jobCount = jobs.length;

        const queryResultOptions: QueryResultsOptions = { startIndex: startIndex.toString(), maxResults: maxResults };
        const queryRowsResponse = await job.getQueryResults(queryResultOptions);
        const schema: bigquery.ITableSchema = queryRowsResponse[2]?.schema || {};

        const totalRows: number = Number(queryRowsResponse[2]?.totalRows || 0);

        const toolkitUri = this.getUri(this.webView, extensionUri, [
            "resources",
            "toolkit.min.js",
        ]);

        const codiconsUri = this.getUri(this.webView, extensionUri, [
            'resources',
            'codicon.css']
        );

        const gridCss = this.getUri(this.webView, extensionUri, [
            'resources',
            'grid.css']
        );

        return [`<!DOCTYPE html>
        <html lang="en" style="display:flex;">
        	<head>
        		<meta charset="UTF-8">
        		<script type="module" src="${toolkitUri}"></script>
                <link href="${codiconsUri}" rel="stylesheet" />
                <link href="${gridCss}" rel="stylesheet" />
        	</head>
        	<body>
                ${(new ResultsGrid(schema, queryRowsResponse[0], totalRows, startIndex, maxResults, jobCount, jobIndex, openInTabVisible))}
                <script>
                    const vscode = acquireVsCodeApi();
                </script>
        	</body>
        </html>`,
            totalRows
        ];

    }

    private async getTableHtml(
        table: Table,
        startIndex: number,
        maxResults: number,
        openInTabVisible: boolean
    ): Promise<[string, number]> {

        const metadata = await table.getMetadata();
        const schema: bigquery.ITableSchema = metadata[0].schema;

        const tableStream = await table.getRows({ startIndex: startIndex.toString(), maxResults: maxResults });
        const totalRows: number = Number(metadata[0].numRows || 0);

        const toolkitUri = this.getUri(this.webView, extensionUri, [
            "resources",
            "toolkit.min.js",
        ]);

        const codiconsUri = this.getUri(this.webView, extensionUri, [
            'resources',
            'codicon.css']
        );

        const gridCss = this.getUri(this.webView, extensionUri, [
            'resources',
            'grid.css']
        );

        return [`<!DOCTYPE html>
        <html lang="en" style="display:flex;">
        	<head>
        		<meta charset="UTF-8">
        		<script type="module" src="${toolkitUri}"></script>
                <link href="${codiconsUri}" rel="stylesheet" />
                <link href="${gridCss}" rel="stylesheet" />
        	</head>
        	<body>
                ${new ResultsGrid(schema, tableStream[0], totalRows, startIndex, maxResults, 1, 1, openInTabVisible)}
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
     * the `postMessage` method. For query results
    */
    listenerResultsOnDidReceiveMessage(message: any): void {

        const resultsGridRender: ResultsGridRender = (this as any)[0];
        const jobResponsePromise: Promise<Job[]> = (this as any)[1];

        const startIndex: number = (this as any)[2];
        const maxResults: number = (this as any)[3];
        const totalRows: number = (this as any)[4];
        const jobIndex: number = (this as any)[5];
        const openInTabVisible: boolean = (this as any)[6];

        const request = {
            jobsPromise: jobResponsePromise,
            startIndex: startIndex,
            maxResults: maxResults,
            jobIndex: jobIndex,
            openInTabVisible: openInTabVisible
        } as ResultsGridRenderRequest;

        switch (message.command || message) {
            case 'first_page':
                request.startIndex = 0;
                resultsGridRender.render(request);

                reporter?.sendTelemetryEvent('listenerResultsOnDidReceiveMessage_first_page', {});

                break;
            case 'previous_page':
                request.startIndex = startIndex - maxResults;
                resultsGridRender.render(request);

                reporter?.sendTelemetryEvent('listenerResultsOnDidReceiveMessage_previous_page', {});

                break;
            case 'next_page':
                request.startIndex = startIndex + maxResults;
                resultsGridRender.render(request);

                reporter?.sendTelemetryEvent('listenerResultsOnDidReceiveMessage_next_page', {});

                break;
            case 'last_page':
                const lastPageStartIndex = (Math.ceil(totalRows / maxResults) - 1) * maxResults;
                request.startIndex = lastPageStartIndex;
                resultsGridRender.render(request);

                reporter?.sendTelemetryEvent('listenerResultsOnDidReceiveMessage_last_page', {});

                break;
            case 'query_index_change':
                const newIndex = Number(message.value || 0);
                request.startIndex = 0;
                request.jobIndex = newIndex;
                resultsGridRender.render(request);


                reporter?.sendTelemetryEvent('listenerResultsOnDidReceiveMessage_query_index_change', {});

                break;
            case 'open_in_tab':

                //evaluate jobResponsePromise to get the ID of the job 
                jobResponsePromise
                    .then(jobs => {

                        let jobName = 'Query_1';
                        if (jobs.length === 1) {
                            jobName = jobs[0].id || '';
                        } else {
                            jobName = jobs[0].id?.replace(RegExp('_\\d+$'), '') || '';
                        }

                        const panel = vscode.window.createWebviewPanel("vscode-bigquery-query-results", jobName, { viewColumn: vscode.ViewColumn.Beside }, { enableFindWidget: true, enableScripts: true });
                        const newresultsGridRender = new ResultsGridRender(panel.webview);

                        request.openInTabVisible = false;
                        newresultsGridRender.render(request);

                        //close the panel, to give more space for the new tab just opened
                        vscode.commands.executeCommand('workbench.action.closePanel');

                    });

                reporter?.sendTelemetryEvent('listenerResultsOnDidReceiveMessage_open_in_tab', {});

                break;

            case 'download_csv':

                const downloadCsvRequest = {
                    jobsPromise: request.jobsPromise,
                    jobIndex:request.jobIndex
                } as DownloadCsvRequest;

                DownloadCsv.download(downloadCsvRequest);

                break;
            default:
                console.error(`Unexpected message "${message}"`);
        }

    }

    /* This function will run as an event triggered when the JS on the webview triggers
     * the `postMessage` method. For tables
    */
    listenerTableOnDidReceiveMessage(message: any): void {

        const resultsGridRender: ResultsGridRender = (this as any)[0];
        const table: Table = (this as any)[1];

        const startIndex: number = (this as any)[2];
        const maxResults: number = (this as any)[3];
        const totalRows: number = (this as any)[4];
        const jobIndex: number = (this as any)[5];
        const openInTabVisible: boolean = (this as any)[6];

        const request = {
            table: table,
            startIndex: startIndex,
            maxResults: maxResults,
            jobIndex: jobIndex,
            openInTabVisible: openInTabVisible
        } as TableGridRenderRequest;

        switch (message.command || message) {
            case 'first_page':
                request.startIndex = 0;
                resultsGridRender.renderTable(request);

                reporter?.sendTelemetryEvent('listenerTableOnDidReceiveMessage_first_page', {});

                break;
            case 'previous_page':
                request.startIndex = startIndex - maxResults;
                resultsGridRender.renderTable(request);

                reporter?.sendTelemetryEvent('listenerTableOnDidReceiveMessage_previous_page', {});

                break;
            case 'next_page':
                request.startIndex = startIndex + maxResults;
                resultsGridRender.renderTable(request);

                reporter?.sendTelemetryEvent('listenerTableOnDidReceiveMessage_next_page', {});

                break;
            case 'last_page':
                const lastPageStartIndex = (Math.ceil(totalRows / maxResults) - 1) * maxResults;
                request.startIndex = lastPageStartIndex;
                resultsGridRender.renderTable(request);

                reporter?.sendTelemetryEvent('listenerTableOnDidReceiveMessage_last_page', {});

                break;
            case 'query_index_change':
                const newIndex = Number(message.value || 0);
                request.startIndex = 0;
                request.jobIndex = newIndex;
                resultsGridRender.renderTable(request);

                reporter?.sendTelemetryEvent('listenerTableOnDidReceiveMessage_query_index_change', {});

                break;
            case 'open_in_tab':
                break;
            default:
                console.error(`Unexpected message "${message}"`);
        }

    }

}