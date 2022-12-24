import * as vscode from 'vscode';
import { getBigQueryClient } from '../extensionCommands';
import { QueryResultsMapping } from '../queryResultsMapping';
import { ResultsGridRender } from './resultsGridRender';
import { ResultsGridRenderRequest } from './resultsGridRenderRequest';

export class QueryResultsSerializer implements vscode.WebviewPanelSerializer {

    private globalState: vscode.Memento;

    constructor(globalState: vscode.Memento) {
        this.globalState = globalState;
    }

    deserializeWebviewPanel(webviewPanel: vscode.WebviewPanel, state: any): Thenable<void> {

        const maxResults: number | undefined = state.maxResults;
        const openInTabVisible: boolean | undefined = state.openInTabVisible;
        const startIndex: number | undefined = state.startIndex;
        const jobIndex: number | undefined = state.jobIndex;

        const uuid = webviewPanel.title.substring(webviewPanel.title.length - 8);
        const queryResultsMapping: QueryResultsMapping[] | undefined = this.globalState.get('queryResultsMapping');
        if (queryResultsMapping
            && queryResultsMapping.length > 0
            && maxResults !== undefined
            && openInTabVisible !== undefined
            && startIndex !== undefined
            && jobIndex !== undefined) {

            const item = queryResultsMapping.find(c => c.uuid === uuid);
            if (item && item.jobReferences) {

                const bqClient = getBigQueryClient();
                const jobReferences = item.jobReferences;

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
        }

        // throw new Error('Method not implemented.');
        return new Promise((resolve, reject) => { resolve(undefined); });
    }
}