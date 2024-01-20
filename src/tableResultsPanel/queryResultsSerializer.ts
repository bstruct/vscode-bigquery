import * as vscode from 'vscode';
import { QueryResultsMappingService } from '../services/queryResultsMappingService';
import { ResultsRender } from '../services/resultsRender';
import { ResultsGridRender } from './resultsGridRender';
import { getBigQueryClient } from '../extensionCommands';
import { ResultsGridRenderRequestV2, ResultsGridRenderRequestV2Type } from './resultsGridRenderRequestV2';

export class QueryResultsSerializer implements vscode.WebviewPanelSerializer {

    private globalState: vscode.Memento;
    private queryResultsWebviewMapping: Map<string, ResultsRender>;

    constructor(globalState: vscode.Memento, queryResultsWebviewMapping: Map<string, ResultsRender>) {
        this.globalState = globalState;
        this.queryResultsWebviewMapping = queryResultsWebviewMapping;
    }

    async deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: any): Promise<void> {

        const uuid = webviewPanel.title.substring(webviewPanel.title.length - 8);

        const resultsGridRender = new ResultsGridRender(webviewPanel);

        webviewPanel.webview.onDidReceiveMessage(async c => {
            if ((c as any).command === 'load_complete') {
                await loadComplete(resultsGridRender, state);
            }
        });

        await resultsGridRender.render2();

        QueryResultsMappingService.updateQueryResultsMappingWebviewPanel(this.queryResultsWebviewMapping, uuid, resultsGridRender);

        //action when panel is closed
        webviewPanel.onDidDispose(e => {
            QueryResultsMappingService.deleteQueryResultsMapping(this.globalState, uuid);
        });
    }
}

let loadComplete = async function (resultsGridRender: ResultsGridRender, state: any): Promise<void> {

    let _postMessageResult1 = await resultsGridRender.postMessage({
        requestType: ResultsGridRenderRequestV2Type.clear.toString(),
        projectId: null,
        token: null,
        job: null,
        error: null
    } as ResultsGridRenderRequestV2);

    const jobId: string | undefined = state.jobId;
    const projectId: string | undefined = state.projectId;
    const location: string | undefined = state.location;
    // const jobIndex: number | undefined = state.jobIndex;

    // const queryResultsMappingItem = QueryResultsMappingService.getQueryResultsMappingItem(this.globalState, uuid);

    if (
        // queryResultsMappingItem !== undefined
        jobId !== undefined
        && projectId !== undefined
        && location !== undefined
        //     && queryResultsMappingItem.jobReferences
        //     && queryResultsMappingItem.jobReferences.length > 0
        //     && maxResults !== undefined
        //     && openInTabVisible !== undefined
        //     && startIndex !== undefined
        //     && jobIndex !== undefined
    ) {

        const bqClient = await getBigQueryClient();
        console.log('QueryResultsSerializer bqClient');

        const token = await bqClient.getToken();
        console.log('QueryResultsSerializer token');

        const b = bqClient.getJob({
            jobId: jobId,
            location: location,
            projectId: projectId
        });
        const job = await b.get();
        const metadata = job[0].metadata;
        console.log('QueryResultsSerializer job');

        try {

            let _postMessageResult2 = await resultsGridRender.postMessage({
                requestType: ResultsGridRenderRequestV2Type.executeQuery.toString(),
                projectId: projectId,
                token: token,
                job: metadata,
                error: null
            } as ResultsGridRenderRequestV2);

            console.log('resultsGridRender.postMessage ', _postMessageResult2);

        } catch (errorx) {
            // resultsGridRender.renderException(error);
            const error =
            {
                message: (errorx as any).message || 'undefined message',
                reason: ''
            };

            let _postMessageResult3 = await resultsGridRender.postMessage({
                requestType: ResultsGridRenderRequestV2Type.error.toString(),
                projectId: null,
                token: null,
                job: null,
                error: error
            } as ResultsGridRenderRequestV2);

        }
    }
};

