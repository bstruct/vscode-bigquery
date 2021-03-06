import * as vscode from 'vscode';
import { BigQueryClient } from './services/bigquery-client';
import { authenticationWebviewProvider, bigQueryTreeDataProvider, bigqueryWebviewViewProvider, reporter } from './extension';
import { ResultsGridRender } from './table_results_panel/results_grid_render';
import { ResultsGridRenderRequest } from './table_results_panel/results_grid_render_request';
import { Authentication } from './services/authentication';
import { BigqueryTreeItem } from './activitybar/tree-item';
import { TableGridRenderRequest } from './table_results_panel/table_grid_render_request';
import { SchemaRender } from './table_results_panel/schema_render';

let resultsGridRender: ResultsGridRender | null = null;

export const COMMAND_RUN_QUERY = "vscode-bigquery.run-query";
export const COMMAND_RUN_SELECTED_QUERY = "vscode-bigquery.run-selected-query";
export const COMMAND_USER_LOGIN = "vscode-bigquery.user-login";
export const COMMAND_SERVICE_ACCOUNT_LOGIN = "vscode-bigquery.service-account-login";
export const COMMAND_AUTHENTICATION_REFRESH = "vscode-bigquery.authentication-refresh";
export const COMMAND_EXPLORER_REFRESH = "vscode-bigquery.explorer-refresh";
export const COMMAND_VIEW_TABLE = "vscode-bigquery.view-table";
export const COMMAND_VIEW_TABLE_SCHEMA = "vscode-bigquery.view-table-schema";

export const commandRunQuery = async function (...args: any[]) {

	const t1 = Date.now();

	if (vscode.window.activeTextEditor === undefined) {
		return;
	}

	const textEditor = vscode.window.activeTextEditor;

	const queryText: string = textEditor.document.getText() ?? '';

	const numberOfJobs = await runQuery(queryText);

	reporter?.sendTelemetryEvent('commandRunQuery', {}, { numberOfJobs: numberOfJobs, elapsedMs: Date.now() - t1 });

};

export const commandRunSelectedQuery = async function (...args: any[]) {

	const t1 = Date.now();

	if (vscode.window.activeTextEditor === undefined) {
		return;
	}

	const textEditor = vscode.window.activeTextEditor;

	const queryText: string = textEditor.document.getText(textEditor.selection) ?? '';

	if (queryText.length === 0) {
		vscode.window.showErrorMessage('No text selected');
		return;
	}

	const numberOfJobs = await runQuery(queryText);

	reporter?.sendTelemetryEvent('commandRunSelectedQuery', {}, { numberOfJobs: numberOfJobs, elapsedMs: Date.now() - t1 });

};

const runQuery = async function (queryText: string): Promise<number> {

	const queryResponse = getBigQueryClient().runQuery(queryText);

	let panel = bigqueryWebviewViewProvider.webviewView;

	if (resultsGridRender === null) {

		if (panel === null) {
			//https://www.eliostruyf.com/devhack-open-custom-vscode-webview-panel-focus-input/
			await vscode.commands.executeCommand('workbench.view.extension.vscode-bigquery-query-results');
			panel = bigqueryWebviewViewProvider.webviewView;
		}
		if (panel === null) { return 0; }

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

	return (await queryResponse).length;

};

export const commandUserLogin = function (...args: any[]) {

	resetBigQueryClient();

	Authentication.userLogin()
		.then(result => {
			if (result.valid) {
				vscode.window.showInformationMessage('Bigquery: User login - successful');
				vscode.commands.executeCommand(COMMAND_AUTHENTICATION_REFRESH);
			} else {
				vscode.window.showErrorMessage('Bigquery: User login - had invalid response');
				reporter?.sendTelemetryErrorEvent('commandUserLogin', { error: 'Bigquery: User login - had invalid response' });
			}
		});

	reporter?.sendTelemetryEvent('commandUserLogin', {});
};

export const commandServiceAccountLogin = function (...args: any[]) {

	resetBigQueryClient();

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
							reporter?.sendTelemetryErrorEvent('commandUserLogin', { error: 'Bigquery: Service account login - had invalid response' });
						}
					});
			}

		});

	reporter?.sendTelemetryEvent('commandServiceAccountLogin', {});

};

export const commandAuthenticationRefresh = function (...args: any[]) {

	const t1 = Date.now();

	resetBigQueryClient();

	authenticationWebviewProvider.refresh();

	reporter?.sendTelemetryEvent('commandAuthenticationRefresh', {}, { elapsedMs: Date.now() - t1 });
};

export const commandExplorerRefresh = function (...args: any[]) {

	const t1 = Date.now();

	bigQueryTreeDataProvider.refresh();

	reporter?.sendTelemetryEvent('commandExplorerRefresh', {}, { elapsedMs: Date.now() - t1 });
};

export const commandViewTable = function (...args: any[]) {

	const t1 = Date.now();

	const item = args[0] as BigqueryTreeItem;

	const title = `${item.projectId}.${item.datasetId}.${item.tableId}`;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	const table = getBigQueryClient().getTable(item.projectId, item.datasetId, item.tableId);

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

	reporter?.sendTelemetryEvent('commandViewTable', {}, { elapsedMs: Date.now() - t1 });

};

export const commandViewTableSchema = function (...args: any[]) {

	const t1 = Date.now();

	const item = args[0] as BigqueryTreeItem;

	const title = `Schema: ${item.projectId}.${item.datasetId}.${item.tableId}`;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	const metadataPromise = getBigQueryClient().getMetadata(item.projectId, item.datasetId, item.tableId);
	const panel = vscode.window.createWebviewPanel("vscode-bigquery-table-schema", title, { viewColumn: vscode.ViewColumn.Active }, { enableFindWidget: true, enableScripts: true });
	const schemaRender = new SchemaRender(panel.webview);

	schemaRender.render(metadataPromise);

	reporter?.sendTelemetryEvent('commandViewTableSchema', {}, { elapsedMs: Date.now() - t1 });

};

let bigQueryClient: BigQueryClient | null;

const getBigQueryClient = function (): BigQueryClient {
	if (!bigQueryClient) {
		bigQueryClient = new BigQueryClient();
	}

	return bigQueryClient;
};

const resetBigQueryClient = function () {
	bigQueryClient = null;
};