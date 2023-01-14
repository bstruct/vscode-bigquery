import * as vscode from 'vscode';
import { QueryResultsMappingService } from '../services/queryResultsMappingService';
import { ResultsGridRender } from './resultsGridRender';
import { ResultsGridRenderRequest } from './resultsGridRenderRequest';

export class QueryResultsSerializer implements vscode.WebviewPanelSerializer {

    private globalState: vscode.Memento;
    private queryResultsWebviewMapping: Map<string, ResultsGridRender>;

    constructor(globalState: vscode.Memento, queryResultsWebviewMapping: Map<string, ResultsGridRender>) {
        this.globalState = globalState;
        this.queryResultsWebviewMapping = queryResultsWebviewMapping;
    }

    deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: any): Thenable<void> {

        const uuid = webviewPanel.title.substring(webviewPanel.title.length - 8);

		const resultsGridRender = new ResultsGridRender(webviewPanel);

        QueryResultsMappingService.updateQueryResultsMappingWebviewPanel(this.queryResultsWebviewMapping, uuid, resultsGridRender);

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

            const resultsGridRender = new ResultsGridRender(webviewPanel);

            const request = {
                jobReferences: queryResultsMappingItem.jobReferences,
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