import * as vscode from 'vscode';
import { ResultsGridRenderRequest } from '../tableResultsPanel/resultsGridRenderRequest';
import { QueryResultsMapping } from './queryResultsMapping';

export class QueryResultsMappingService {

    public static getQueryResultsMappingUuid(globalState: vscode.Memento, textEditor: vscode.TextEditor): string | undefined {

        let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
        if (queryResultsMapping) {
            //possible corrections
            QueryResultsMappingService.correctQueryResultsMapping(globalState, queryResultsMapping);

            //
            const item = queryResultsMapping.find(c => c.textEditorUriString === textEditor.document.uri.toString());
            if (item) {
                return item.uuid;
            }
        }

        return undefined;
    };

    public static getQueryResultsMappingItem(globalState: vscode.Memento, uuid: string): QueryResultsMapping | undefined {

        let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
        if (queryResultsMapping) {
            //possible corrections
            QueryResultsMappingService.correctQueryResultsMapping(globalState, queryResultsMapping);

            //
            const item = queryResultsMapping.find(c => c.uuid === uuid);
            if (item) {
                return item;
            }
        }

        return undefined;
    };

    public static async upsertQueryResultsMapping(globalState: vscode.Memento, uuid: string, textEditor: vscode.TextEditor) {

        let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');

        const textEditorUriString = textEditor.document.uri.toString();

        if (queryResultsMapping) {
            const item = queryResultsMapping.find(c => c.uuid === uuid);
            if (item) {
                item.textEditorUriString = textEditorUriString;
            } else {
                queryResultsMapping.push({ uuid: uuid, textEditorUriString: textEditorUriString } as QueryResultsMapping);
            }
        } else {
            queryResultsMapping = [{ uuid: uuid, textEditorUriString: textEditorUriString } as QueryResultsMapping];
        }

        globalState.update('queryResultsMapping', queryResultsMapping);

    };

    public static async udpateQueryResultsMapping(globalState: vscode.Memento, uuid: string, request: ResultsGridRenderRequest) {

        let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
        if (queryResultsMapping) {

            const item = queryResultsMapping.find(c => c.uuid === uuid);
            if (item) {
                const jobs = await request.jobsPromise;
                const jobReferences = jobs.map(c => c.metadata.jobReference);

                item.jobReferences = jobReferences;
                item.jobIndex = request.jobIndex;
                globalState.update('queryResultsMapping', queryResultsMapping);
            }
        }

    };

    public static deleteQueryResultsMapping(globalState: vscode.Memento, uuid: string) {

        let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
        if (queryResultsMapping) {
            queryResultsMapping = queryResultsMapping.filter(c => c.uuid !== uuid);
            globalState.update('queryResultsMapping', queryResultsMapping);
            //possible corrections
            QueryResultsMappingService.correctQueryResultsMapping(globalState, queryResultsMapping);
        }

    };

    static getQueryResultsMappingWebviewPanel(queryResultsWebviewMapping: Map<string, vscode.WebviewPanel>, uuid: string): vscode.WebviewPanel | undefined {
        if (queryResultsWebviewMapping) {
            return queryResultsWebviewMapping.get(uuid);
        }
        return undefined;
    }

    static updateQueryResultsMappingWebviewPanel(queryResultsWebviewMapping: Map<string, vscode.WebviewPanel>, uuid: string, webviewPanel: vscode.WebviewPanel) {
        if (queryResultsWebviewMapping) {
            queryResultsWebviewMapping.set(uuid, webviewPanel);
        }
    }

    static async correctQueryResultsMapping(globalState: vscode.Memento, queryResultsMapping: QueryResultsMapping[]) {

        let uuids: string[] = [];

        for (let i = 0; i < vscode.window.tabGroups.all.length; i++) {
            const tabGroup = vscode.window.tabGroups.all[i];
            for (let t = 0; t < tabGroup.tabs.length; t++) {
                const element = tabGroup.tabs[t];
                if (element.input && (element.input as any).viewType === 'mainThreadWebview-bigquery-query-results') {
                    const uuid = element.label.substring(element.label.length - 8);
                    uuids.push(uuid);
                }
            }
        }

        queryResultsMapping = queryResultsMapping.filter(c => uuids.indexOf(c.uuid) >= 0);
        globalState.update('queryResultsMapping', queryResultsMapping);

    };

}