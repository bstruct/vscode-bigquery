import * as vscode from 'vscode';
import { BigQueryQueryRunner } from './bigquery/bigquery-query-runner';
import { ResultsGridRender } from './table_results_panel/results_grid_render';

export const command_runQuery = async function (...args: any[]) {

	if (vscode.window.activeTextEditor === undefined) {
		return;
	}

	const textEditor = vscode.window.activeTextEditor;

	const bqRunner = new BigQueryQueryRunner();

	const queryText: string = textEditor.document.getText() ?? '';

	const queryResponse = bqRunner.runQuery(queryText);

	//display result in panel to show the query result rows
	const panel = vscode.window.createWebviewPanel("xxx", "xxx", vscode.ViewColumn.Beside,
		{
			enableScripts: true,
			enableFindWidget: true,
		});

	const _ = new ResultsGridRender().render(panel.webview, queryResponse);

}
