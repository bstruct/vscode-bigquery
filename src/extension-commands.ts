import * as vscode from 'vscode';
import { BigQueryClient } from './services/bigquery-client';
import { authenticationWebviewProvider, bigQueryTreeDataProvider, bigqueryWebviewViewProvider } from './extension';
import { ResultsGridRender } from './table_results_panel/results_grid_render';
import { ResultsGridRenderRequest } from './table_results_panel/results_grid_render_request';
import { Authentication } from './services/authentication';
import { BigqueryTreeItem } from './activitybar/tree-item';
import { TableGridRenderRequest } from './table_results_panel/table_grid_render_request';
import { SchemaRender } from './table_results_panel/schema_render';

let resultsGridRender: ResultsGridRender | null = null;

export const COMMAND_RUN_QUERY = "vscode-bigquery.run-query";
export const COMMAND_USER_LOGIN = "vscode-bigquery.user-login";
export const COMMAND_SERVICE_ACCOUNT_LOGIN = "vscode-bigquery.service-account-login";
export const COMMAND_AUTHENTICATION_REFRESH = "vscode-bigquery.authentication-refresh";
export const COMMAND_EXPLORER_REFRESH = "vscode-bigquery.explorer-refresh";
export const COMMAND_VIEW_TABLE = "vscode-bigquery.view-table";
export const COMMAND_VIEW_TABLE_SCHEMA = "vscode-bigquery.view-table-schema";

export const commandRunQuery = async function (...args: any[]) {

	if (vscode.window.activeTextEditor === undefined) {
		return;
	}

	const textEditor = vscode.window.activeTextEditor;

	const queryText: string = textEditor.document.getText() ?? '';

	const queryResponse = BigQueryClient.runQuery(queryText);

	let panel = bigqueryWebviewViewProvider.webviewView;

	if (resultsGridRender === null) {

		if (panel === null) {
			//https://www.eliostruyf.com/devhack-open-custom-vscode-webview-panel-focus-input/
			await vscode.commands.executeCommand('workbench.view.extension.vscode-bigquery-query-results');
			panel = bigqueryWebviewViewProvider.webviewView;
		}
		if (panel === null) { return; }

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

};

export const commandUserLogin = function (...args: any[]) {
	Authentication.userLogin()
		.then(result => {
			if (result.valid) {
				vscode.window.showInformationMessage('Bigquery: User login - successful');
				vscode.commands.executeCommand(COMMAND_AUTHENTICATION_REFRESH);
			} else {
				vscode.window.showErrorMessage('Bigquery: User login - had invalid response');
			}
		});
};

export const commandServiceAccountLogin = function (...args: any[]) {

	vscode.window.showOpenDialog({ canSelectFiles: true, canSelectMany: false, canSelectFolders: false })
		.then(file => {

			if (file) {

				let path = file[0].path;
				if (process.platform === 'win32') {
					path = path.substring(1);
				}

				Authentication.serviceAccountLogin(path)
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

};

export const commandAuthenticationRefresh = function (...args: any[]) {
	authenticationWebviewProvider.refresh();
};

export const commandExplorerRefresh = function (...args: any[]) {
	bigQueryTreeDataProvider.refresh();
};

export const commandViewTable = function (...args: any[]) {

	const item = args[0] as BigqueryTreeItem;

	const title = `${item.projectId}.${item.datasetId}.${item.tableId}`;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	const table = BigQueryClient.getTable(item.projectId, item.datasetId, item.tableId);

	const request = {
		table: table,
		startIndex: 0,
		maxResults: 50,
		jobIndex: 0,
		openInTabVisible: false
	} as TableGridRenderRequest;

	const panel = vscode.window.createWebviewPanel("vscode-bigquery-query-results", title, { viewColumn: vscode.ViewColumn.Active }, { enableFindWidget: true, enableScripts: true });
	const newresultsGridRender = new ResultsGridRender(panel.webview);

	request.openInTabVisible = false;
	newresultsGridRender.renderTable(request);

};


export const commandViewTableSchema = function (...args: any[]) {

	const item = args[0] as BigqueryTreeItem;

	const title = `Schema: ${item.projectId}.${item.datasetId}.${item.tableId}`;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	const metadataPromise = BigQueryClient.getMetadata(item.projectId, item.datasetId, item.tableId);
	const panel = vscode.window.createWebviewPanel("vscode-bigquery-table-schema", title, { viewColumn: vscode.ViewColumn.Active }, { enableFindWidget: true, enableScripts: true });
	const schemaRender = new SchemaRender(panel.webview);

	schemaRender.render(metadataPromise);

};