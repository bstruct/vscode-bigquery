import * as vscode from 'vscode';
import { BigQueryQueryRunner } from './bigquery/bigquery-query-runner';
import { bigqueryWebviewViewProvider } from './extension';
import { ResultsGridRender } from './table_results_panel/results_grid_render';

let resultsGridRender: ResultsGridRender | null = null;

export const command_runQuery = async function (...args: any[]) {

	if (vscode.window.activeTextEditor === undefined) {
		return;
	}

	const textEditor = vscode.window.activeTextEditor;

	const bqRunner = new BigQueryQueryRunner();

	const queryText: string = textEditor.document.getText() ?? '';

	const queryResponse = bqRunner.runQuery(queryText);

	let panel = bigqueryWebviewViewProvider.webviewView;

	if (resultsGridRender == null) {

		if (panel == null) {
			//https://www.eliostruyf.com/devhack-open-custom-vscode-webview-panel-focus-input/
			await vscode.commands.executeCommand('workbench.view.extension.vscode-bigquery-query-results');
			panel = bigqueryWebviewViewProvider.webviewView;
		}
		if (panel == null) { return; }

		resultsGridRender = new ResultsGridRender(panel.webview);
	}

	if (panel && !panel.visible) { panel.show(); }

	resultsGridRender.render(queryResponse);

}