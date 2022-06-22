import * as vscode from 'vscode';
import { BigQueryQueryRunner } from './services/bigquery-query-runner';
import { authenticationWebviewProvider, bigqueryWebviewViewProvider } from './extension';
import { ResultsGridRender } from './table_results_panel/results_grid_render';
import { ResultsGridRenderRequest } from './table_results_panel/results_grid_render_request';
import { Authentication } from './services/authentication';

let resultsGridRender: ResultsGridRender | null = null;

export const COMMAND_RUN_QUERY = "vscode-bigquery.run-query";
export const COMMAND_USER_LOGIN = "vscode-bigquery.user-login";
export const COMMAND_SERVICE_ACCOUNT_LOGIN = "vscode-bigquery.service-account-login";
export const COMMAND_AUTHENTICATION_REFRESH = "vscode-bigquery.authentication-refresh";
export const COMMAND_EXPLORER_REFRESH = "vscode-bigquery.explorer-refresh";
export const COMMAND_PREVIEW_TABLE = "vscode-bigquery.preview-table";

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

	const request = {
		jobsPromise: queryResponse,
		startIndex: 0,
		maxResults: 50,
		jobIndex: 0,
		openInTabVisible: true
	} as ResultsGridRenderRequest;

	resultsGridRender.render(request);

}

export const command_userLogin = function (...args: any[]) {
	Authentication.userLogin()
		.then(result => {
			if (result.valid) {
				vscode.window.showInformationMessage('Bigquery: User login - successful');
				vscode.commands.executeCommand(COMMAND_AUTHENTICATION_REFRESH);
			} else {
				vscode.window.showErrorMessage('Bigquery: User login - had invalid response');
			}
		});
}

export const command_serviceAccountLogin = function (...args: any[]) {

	vscode.window.showOpenDialog({ canSelectFiles: true, canSelectMany: false, canSelectFolders: false })
		.then(file => {

			if (file) {
				Authentication.serviceAccountLogin(file[0].path)
					.then(result => {
						if (result.valid) {
							vscode.window.showInformationMessage('Bigquery: Service account login - successful');
							vscode.commands.executeCommand(COMMAND_AUTHENTICATION_REFRESH);
						} else {
							vscode.window.showErrorMessage('Bigquery: Service account login - had invalid response');
						}
					});
			}

		});

}

export const command_authenticationRefresh = function (...args: any[]) {
	authenticationWebviewProvider.refresh();
}

export const command_explorerRefresh = function (...args: any[]) {
	vscode.commands.executeCommand('workbench.actions.treeView.bigquery-tree-data-provider.refresh');
}

export const command_previewTable = function (...args: any[]) {
	debugger;
}