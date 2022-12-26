import * as vscode from 'vscode';
import { getBigQueryClient } from '../extensionCommands';
import { QueryResultsMappingService } from '../services/queryResultsMappingService';
import { ResultsGridRender } from './resultsGridRender';
import { ResultsGridRenderRequest } from './resultsGridRenderRequest';

export class QueryResultsSerializer implements vscode.WebviewPanelSerializer {

    private globalState: vscode.Memento;

    constructor(globalState: vscode.Memento) {
        this.globalState = globalState;
    }

    deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: any): Thenable<void> {

        const uuid = webviewPanel.title.substring(webviewPanel.title.length - 8);

		QueryResultsMappingService.updateQueryResultsMappingWebviewPanel(uuid, webviewPanel);

        //action when panel is closed
        webviewPanel.onDidDispose(e => {
            QueryResultsMappingService.deleteQueryResultsMapping(this.globalState, uuid);
        });

        const maxResults: number | undefined = state.maxResults;
        const openInTabVisible: boolean | undefined = state.openInTabVisible;
        const startIndex: number | undefined = state.startIndex;
        const jobIndex: number | undefined = state.jobIndex;

        const queryResultsMappingItem = QueryResultsMappingService.getQueryResultsMappingItem(this.globalState, uuid);

        if (queryResultsMappingItem !== undefined
            && queryResultsMappingItem.jobReferences
            && queryResultsMappingItem.jobReferences.length > 0
            && maxResults !== undefined
            && openInTabVisible !== undefined
            && startIndex !== undefined
            && jobIndex !== undefined) {

            const bqClient = getBigQueryClient();
            const jobReferences = queryResultsMappingItem.jobReferences;

            const jobsPromise = new Promise((resolve, reject) => {
                resolve(jobReferences.map(c => bqClient.getJob(c)));
            });

            const resultsGridRender = new ResultsGridRender(webviewPanel.webview);

            const request = {
                jobsPromise: jobsPromise,
                startIndex: 0,
                maxResults: 50,
                jobIndex: 0,
                openInTabVisible: true
            } as ResultsGridRenderRequest;

            resultsGridRender.render(request);

        }

        // throw new Error('Method not implemented.');
        return new Promise((resolve, reject) => { resolve(undefined); });
    }
}