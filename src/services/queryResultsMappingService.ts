import * as vscode from 'vscode';
import { ResultsChartRender } from '../charts/resultsChartRender';
import { ResultsChartRenderRequest } from '../charts/ResultsChartRenderRequest';
import { ResultsGridRender } from '../tableResultsPanel/resultsGridRender';
import { ResultsGridRenderRequest } from '../tableResultsPanel/resultsGridRenderRequest';
import { QueryResultsMapping } from './queryResultsMapping';
import { QueryResultsVisualizationType } from './queryResultsVisualizationType';
import { ResultsRender } from './resultsRender';

export class QueryResultsMappingService {

    public static getQueryResultsMappingUuid(globalState: vscode.Memento, textEditor: vscode.TextEditor, visualizationType: QueryResultsVisualizationType): string | undefined {

        let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
        if (queryResultsMapping) {
            //possible corrections
            QueryResultsMappingService.correctQueryResultsMapping(globalState, queryResultsMapping);

            //
            const item = queryResultsMapping.find(c => c.textEditorUriString === textEditor.document.uri.toString() && c.visualizationType === visualizationType);
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

    public static getQueryResultsChartMappingItem(globalState: vscode.Memento, uuid: string): QueryResultsMapping | undefined {

        let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsChartMapping');
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

    public static async upsertQueryResultsMapping(globalState: vscode.Memento, uuid: string, textEditor: vscode.TextEditor, visualizationType: QueryResultsVisualizationType) {

        let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');

        const textEditorUriString = textEditor.document.uri.toString();

        if (queryResultsMapping) {
            const item = queryResultsMapping.find(c => c.uuid === uuid);
            if (item) {
                item.textEditorUriString = textEditorUriString;
            } else {
                queryResultsMapping.push({ uuid: uuid, textEditorUriString: textEditorUriString, visualizationType: visualizationType } as QueryResultsMapping);
            }
        } else {
            queryResultsMapping = [{ uuid: uuid, textEditorUriString: textEditorUriString, visualizationType: visualizationType } as QueryResultsMapping];
        }

        globalState.update('queryResultsMapping', queryResultsMapping);

    };

    public static async updateQueryResultsMapping(globalState: vscode.Memento, uuid: string, request: ResultsGridRenderRequest | ResultsChartRenderRequest) {

        let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
        if (queryResultsMapping) {

            const item = queryResultsMapping.find(c => c.uuid === uuid);
            if (item) {
                item.jobReferences = request.jobReferences;
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

    static getQueryResultsMappingResultsGridRender(queryResultsWebviewMapping: Map<string, ResultsRender>, uuid: string): ResultsGridRender | undefined {
        if (queryResultsWebviewMapping) {
            const mapping = queryResultsWebviewMapping.get(uuid);
            if (mapping && mapping.resultsGridRender) {
                return mapping.resultsGridRender;
            }
        }
        return undefined;
    }

    static getQueryResultsMappingResultsChartRender(queryResultsWebviewMapping: Map<string, ResultsRender>, uuid: string): ResultsChartRender | undefined {
        if (queryResultsWebviewMapping) {
            const mapping = queryResultsWebviewMapping.get(uuid);
            if (mapping && mapping.resultsChartRender) {
                return mapping.resultsChartRender;
            }
        }
        return undefined;
    }

    static updateQueryResultsMappingWebviewPanel(queryResultsWebviewMapping: Map<string, ResultsRender>, uuid: string, resultsGridRender: ResultsGridRender) {
        if (queryResultsWebviewMapping) {
            const mapping = queryResultsWebviewMapping.get(uuid);
            if (mapping) {
                queryResultsWebviewMapping.set(uuid, { resultsGridRender: resultsGridRender, resultsChartRender: mapping.resultsChartRender });
            } else {
                queryResultsWebviewMapping.set(uuid, { resultsGridRender: resultsGridRender, resultsChartRender: undefined });
            }
        }
    }

    static updateQueryResultsChartMappingWebviewPanel(queryResultsWebviewMapping: Map<string, ResultsRender>, uuid: string, resultsChartRender: ResultsChartRender) {
        if (queryResultsWebviewMapping) {
            const mapping = queryResultsWebviewMapping.get(uuid);
            if (mapping) {
                queryResultsWebviewMapping.set(uuid, { resultsGridRender: mapping.resultsGridRender, resultsChartRender: resultsChartRender });
            } else {
                queryResultsWebviewMapping.set(uuid, { resultsGridRender: undefined, resultsChartRender: resultsChartRender });
            }
        }
    }

    static async correctQueryResultsMapping(globalState: vscode.Memento, queryResultsMapping: QueryResultsMapping[]) {

        let uuids: string[] = [];

        for (let i = 0; i < vscode.window.tabGroups.all.length; i++) {
            const tabGroup = vscode.window.tabGroups.all[i];
            for (let t = 0; t < tabGroup.tabs.length; t++) {
                const element = tabGroup.tabs[t];
                if (element.input
                    && (
                        (element.input as any).viewType === 'mainThreadWebview-bigquery-query-results'
                        ||
                        (element.input as any).viewType === 'mainThreadWebview-bigquery-query-chart'
                    )
                ) {
                    const uuid = element.label.substring(element.label.length - 8);
                    uuids.push(uuid);
                }
            }
        }

        queryResultsMapping = queryResultsMapping.filter(c => uuids.indexOf(c.uuid) >= 0);
        globalState.update('queryResultsMapping', queryResultsMapping);

    };

}