import * as vscode from 'vscode';
import { QueryResultsMappingService } from '../services/queryResultsMappingService';
import { ResultsRender } from '../services/resultsRender';
import { ResultsChartRender } from './resultsChartRender';
import { ResultsChartRenderRequest } from './ResultsChartRenderRequest';

export class ChartResultsSerializer implements vscode.WebviewPanelSerializer {

    private globalState: vscode.Memento;
    private queryResultsWebviewMapping: Map<string, ResultsRender>;

    constructor(globalState: vscode.Memento, queryResultsWebviewMapping: Map<string, ResultsRender>) {
        this.globalState = globalState;
        this.queryResultsWebviewMapping = queryResultsWebviewMapping;
    }

    deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: any): Thenable<void> {

        const uuid = webviewPanel.title.substring(webviewPanel.title.length - 8);

		const resultsChartRender = new ResultsChartRender(webviewPanel);

        QueryResultsMappingService.updateQueryResultsChartMappingWebviewPanel(this.queryResultsWebviewMapping, uuid, resultsChartRender);

        //action when panel is closed
        webviewPanel.onDidDispose(e => {
            QueryResultsMappingService.deleteQueryResultsMapping(this.globalState, uuid);
        });

        // const maxResults: number | undefined = state.maxResults;
        // const openInTabVisible: boolean | undefined = state.openInTabVisible;
        // const startIndex: number | undefined = state.startIndex;
        // const jobIndex: number | undefined = state.jobIndex;

        const queryResultsMappingItem = QueryResultsMappingService.getQueryResultsMappingItem(this.globalState, uuid);

        if (queryResultsMappingItem !== undefined
            && queryResultsMappingItem.jobReferences
            && queryResultsMappingItem.jobReferences.length > 0
            // && maxResults !== undefined
            // && openInTabVisible !== undefined
            // && startIndex !== undefined
            // && jobIndex !== undefined
            ) {

            const request = {
                jobReferences: queryResultsMappingItem.jobReferences,
                jobIndex:0,
            } as ResultsChartRenderRequest;

            return resultsChartRender.render(request);

        }

        // throw new Error('Method not implemented.');
        return new Promise((resolve, reject) => { resolve(undefined); });
    }
}