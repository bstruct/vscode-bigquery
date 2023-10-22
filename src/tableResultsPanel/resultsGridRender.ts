import * as vscode from 'vscode';
// import bigquery from '@google-cloud/bigquery/build/src/types';
import { getExtensionUri, QUERY_RESULTS_VIEW_TYPE, getTelemetryReporter } from '../extension';
// import { QueryResultsOptions, Table } from '@google-cloud/bigquery';
import { SimpleQueryRowsResponseError } from '../services/simpleQueryRowsResponseError';
// import { ResultsGrid } from './resultsGrid';
import { ResultsGridRenderRequest } from './resultsGridRenderRequest';
import { COMMAND_DOWNLOAD_CSV, getBigQueryClient } from '../extensionCommands';
import { JobReference } from '../services/queryResultsMapping';
import { TableReference } from '../services/tableMetadata';
import { ResultsGridRenderRequestV2 } from './resultsGridRenderRequestV2';

// import { get_web_components_list } from "grid_render/grid_render";


//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class ResultsGridRender {

    private webViewPanel: vscode.WebviewPanel;

    constructor(webViewPanel: vscode.WebviewPanel) {
        this.webViewPanel = webViewPanel;
        const listener = this.webViewPanel.webview.onDidReceiveMessage(this.listenerResultsOnDidReceiveMessage, this);
        webViewPanel.onDidDispose(c => { listener.dispose(); });
    }

    // public renderLoadingIcon() {
    //     this.webViewPanel.webview.html = this.getWaitingHtml(50, false, 0, 0);
    // }

    public render1() {

        const extensionUri = getExtensionUri();

        const gridJs = this.getUri(this.webViewPanel.webview, extensionUri, [
            'resources',
            'grid.js']
        );

        this.webViewPanel.webview.html = `<!DOCTYPE html>
        <html lang="en">
        	<head>
        		<meta charset="UTF-8">
                <script>
                    const vscode = acquireVsCodeApi();
                </script>
        	</head>
        	<body>
                <bq id="q1"></bq>
                <script type="module" src="${gridJs}"></script>
        	</body>
        </html>`;
    }

    public postMessage(message: ResultsGridRenderRequestV2): Thenable<boolean> {
        return this.webViewPanel.webview.postMessage(message);
    }

    public async render(request: ResultsGridRenderRequest) {

        try {

            // // possible solution to prevent the memory leak
            // const viewColumn = this.webViewPanel.viewColumn || vscode.ViewColumn.Two;
            // const newPanel = vscode.window.createWebviewPanel(this.webViewPanel.viewType, this.webViewPanel.title, { viewColumn: viewColumn, preserveFocus: true }, { enableFindWidget: true, enableScripts: true });
            // this.webViewPanel.dispose();
            // this.webViewPanel = newPanel;
            // const listener = newPanel.webview.onDidReceiveMessage(this.listenerResultsOnDidReceiveMessage, this);
            // newPanel.onDidDispose(c => { listener.dispose(); });

            //set waiting gif
            // const x = vscode.dis;
            // this.webViewPanel.webview.html = this.getWaitingHtml(request.maxResults, request.openInTabVisible, request.startIndex, request.jobIndex);

            if (this.webViewPanel.webview.html.length === 0) {
                const [html, totalRows] = await this.getResultsHtml(request);
                this.webViewPanel.webview.html = html;
            }

            this.webViewPanel.webview.postMessage(request);

        } catch (error: any) {
            // vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
            this.webViewPanel.webview.html = this.getExceptionHtml(error);
        }
    }

    public renderException(error: any) {
        this.webViewPanel.webview.html = this.getExceptionHtml(error);
    }

    // private getWaitingHtml(maxResults: number, openInTabVisible: boolean, startIndex: number, jobIndex: number | undefined): string {

    //     const toolkitUri = this.getUri(this.webViewPanel.webview, getExtensionUri(), [
    //         "resources",
    //         "toolkit.min.js",
    //     ]);

    //     return `<!DOCTYPE html>
    // 	<html lang="en">
    // 		<head>
    // 			<meta charset="UTF-8">
    // 			<meta name="viewport" content="width=device-width, initial-scale=1.0">
    // 			<script type="module" src="${toolkitUri}"></script>
    //             <script>
    //                 const qElement = document.querySelectorAll('div.editor-actions ul.actions-container > li.action-item a[aria-label="\${x1}"]');
    //                 if(qElement.length >0){
    //                     const element = qElement[0];
    //                     element.innerText = 'trying';
    //                 }

    //                 const vscode = acquireVsCodeApi();
    //                 vscode.setState({ maxResults: ${maxResults}, openInTabVisible: ${openInTabVisible}, startIndex: ${startIndex}, jobIndex: ${jobIndex} });
    //             </script>
    // 		</head>
    // 		<body>
    //             <vscode-progress-ring></vscode-progress-ring>
    // 		</body>
    // 	</html>`;

    // }

    private getExceptionHtml(exception: any): string {

        const toolkitUri = this.getUri(this.webViewPanel.webview, getExtensionUri(), [
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
    private async getResultsHtml(request: ResultsGridRenderRequest): Promise<[string, number]> {

        let totalRows: number = 0;
        // let rows: any[] = [];
        // let schema: bigquery.ITableSchema = {};

        // if (request.jobReferences && request.jobReferences.length > 0) {
        //     const jobsReference = request.jobReferences[request.jobIndex];
        //     const job = getBigQueryClient().getJob(jobsReference);

        //     const metadata = await job.getMetadata();
        //     const statementType = metadata[0].statistics.query.statementType;
        //     if (statementType === 'INSERT' || statementType === 'UPDATE' || statementType === 'DELETE' || statementType === 'MERGE') {
        //         const dmlStats = metadata[0].statistics.query.dmlStats;
        //         rows = [
        //             {
        //                 insertedRowCount: dmlStats.insertedRowCount ?? null,
        //                 updatedRowCount: dmlStats.updatedRowCount ?? null,
        //                 deletedRowCount: dmlStats.deletedRowCount ?? null,
        //             }
        //         ];
        //         schema = {
        //             fields: [
        //                 {
        //                     name: 'insertedRowCount',
        //                     type: 'STRING'
        //                 } as bigquery.ITableFieldSchema,
        //                 {
        //                     name: 'updatedRowCount',
        //                     type: 'STRING'
        //                 } as bigquery.ITableFieldSchema,
        //                 {
        //                     name: 'deletedRowCount',
        //                     type: 'STRING'
        //                 } as bigquery.ITableFieldSchema
        //             ]
        //         } as bigquery.ITableSchema;
        //         totalRows = 1;

        //     } else {
        //         const queryResultOptions: QueryResultsOptions = { startIndex: request.startIndex.toString(), maxResults: request.maxResults };
        //         const queryRowsResponse = (await job.getQueryResults(queryResultOptions));
        //         rows = queryRowsResponse[0];
        //         schema = queryRowsResponse[2]?.schema || {};
        //         totalRows = Number(queryRowsResponse[2]?.totalRows || 0);
        //     }

        // } else {
        //     if (request.tableReference) {

        //         const tableReference = request.tableReference;
        //         const table = getBigQueryClient().getTable(tableReference.projectId, tableReference.datasetId, tableReference.tableId);
        //         const metadata = await table.getMetadata();
        //         schema = metadata[0].schema;
        //         totalRows = Number(metadata[0].numRows || 0);
        //         rows = (await table.getRows({ startIndex: request.startIndex.toString(), maxResults: request.maxResults }))[0];

        //     } else {
        //         throw new Error('Unexpected error: "No job results nor table was found"');
        //     }
        // }

        const extensionUri = getExtensionUri();

        const toolkitUri = this.getUri(this.webViewPanel.webview, extensionUri, [
            "resources",
            "toolkit.min.js",
        ]);

        const codiconsUri = this.getUri(this.webViewPanel.webview, extensionUri, [
            'resources',
            'codicon.css']
        );

        const gridJs = this.getUri(this.webViewPanel.webview, extensionUri, [
            'resources',
            'grid.js']
        );

        return [`<!DOCTYPE html>
        <html lang="en" style="display:flex;">
        	<head>
        		<meta charset="UTF-8">
        		<script type="module" src="${toolkitUri}"></script>
                <link href="${codiconsUri}" rel="stylesheet" />
                <script>
                    const vscode = acquireVsCodeApi();
                    vscode.setState({ maxResults: ${request.maxResults}, openInTabVisible: ${request.openInTabVisible}, startIndex: ${request.startIndex}, jobIndex: ${request.jobIndex} });
                </script>
        	</head>
        	<body>
                <table-with-controls id="q1"></table-with-controls>
                <script type="module" src="${gridJs}"></script>
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
    async listenerResultsOnDidReceiveMessage(message: any): Promise<void> {

        const reporter = getTelemetryReporter();

        const resultsGridRender: ResultsGridRender = this as ResultsGridRender;

        if (message.parameters && message.parameters.length === 7) {
            const jobReferences: JobReference[] = message.parameters[0];
            const tableReference: TableReference = message.parameters[1];

            const startIndex: number = message.parameters[2];
            const maxResults: number = message.parameters[3];
            const totalRows: number = message.parameters[4];
            const jobIndex: number = message.parameters[5];
            const openInTabVisible: boolean = message.parameters[6];

            const request = {
                jobReferences: jobReferences,
                tableReference: tableReference,
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
                    // const newIndex = Number(message.value || 0);
                    request.startIndex = 0;
                    request.jobIndex = Number.parseInt(request.jobIndex as any);
                    resultsGridRender.render(request);


                    reporter?.sendTelemetryEvent('listenerResultsOnDidReceiveMessage_query_index_change', {});

                    break;
                case 'open_in_tab':

                    if (request.jobReferences && request.jobReferences.length > 0) {

                        let jobName = 'Query_1';
                        if (request.jobReferences.length === 1) {
                            jobName = request.jobReferences[0].jobId || '';
                        } else {
                            jobName = request.jobReferences[0].jobId?.replace(RegExp('_\\d+$'), '') || '';
                        }

                        const panel = vscode.window.createWebviewPanel(QUERY_RESULTS_VIEW_TYPE, jobName, { viewColumn: vscode.ViewColumn.Beside, preserveFocus: false }, { enableFindWidget: true, enableScripts: true });
                        const newresultsGridRender = new ResultsGridRender(panel);

                        request.openInTabVisible = false;
                        newresultsGridRender.render(request);

                        reporter?.sendTelemetryEvent('listenerResultsOnDidReceiveMessage_open_in_tab', {});
                    }

                    break;

                case 'download_csv':

                    await vscode.commands.executeCommand(COMMAND_DOWNLOAD_CSV, "resultsTable");

                    break;
                default:
                    console.error(`Unexpected message "${message}"`);
            }
        }
    }

    reveal(viewColumn?: vscode.ViewColumn, preserveFocus?: boolean): void {
        this.webViewPanel.reveal(viewColumn, preserveFocus);
    }

}

