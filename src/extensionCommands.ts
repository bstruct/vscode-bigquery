import * as vscode from 'vscode';
import { BigQueryClient } from './services/bigqueryClient';
import { authenticationWebviewProvider, bigQueryTreeDataProvider, reporter } from './extension';
import { ResultsGridRenderRequest } from './tableResultsPanel/resultsGridRenderRequest';
import { Authentication } from './services/authentication';
import { BigqueryTreeItem } from './activitybar/treeItem';
import { TableGridRenderRequest } from './tableResultsPanel/tableGridRenderRequest';
import { SchemaRender } from './tableResultsPanel/schemaRender';
import { QueryGeneratorService } from './services/queryGeneratorService';
import { ResultsGridRender } from './tableResultsPanel/resultsGridRender';
import { v4 as uuidv4 } from 'uuid';
import { DownloadCsv } from './tableResultsPanel/downloadCsv';
import { QueryResultsMappingService } from './services/queryResultsMappingService';
import { QueryResultsMapping } from './services/queryResultsMapping';

export const COMMAND_RUN_QUERY = "vscode-bigquery.run-query";
export const COMMAND_RUN_SELECTED_QUERY = "vscode-bigquery.run-selected-query";
export const COMMAND_USER_LOGIN = "vscode-bigquery.user-login";
export const COMMAND_USER_LOGIN_WITH_DRIVE = "vscode-bigquery.user-login-drive";
export const COMMAND_GCLOUD_INIT = "vscode-bigquery.gcloud-init";
export const COMMAND_SERVICE_ACCOUNT_LOGIN = "vscode-bigquery.service-account-login";
export const COMMAND_AUTHENTICATION_REFRESH = "vscode-bigquery.authentication-refresh";
export const COMMAND_EXPLORER_REFRESH = "vscode-bigquery.explorer-refresh";
export const COMMAND_VIEW_TABLE = "vscode-bigquery.view-table";
export const COMMAND_VIEW_TABLE_SCHEMA = "vscode-bigquery.view-table-schema";
export const COMMAND_CREATE_TABLE_DEFAULT_QUERY = "vscode-bigquery.create-table-default-query";
export const COMMAND_SET_DEFAULT_PROJECT = "vscode-bigquery.set-default-project";
export const COMMAND_PROJECT_PIN = "vscode-bigquery.project-pin";
export const COMMAND_DOWNLOAD_CSV = "vscode-bigquery.download-csv";
export const SETTING_PINNED_PROJECTS = "vscode-bigquery.pinned-projects";

export const commandRunQuery = async function (this: any, ...args: any[]) {

	const t1 = Date.now();

	// const commands = await vscode.commands.getCommands();
	// const q = commands.filter(c => c.toLowerCase().indexOf('set') >= 0);

	// await vscode.commands.executeCommand('setContext', 'x', 0);
	// await vscode.commands.executeCommand('setContext', 'x1', 'my text');

	const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

	if (activeTab === undefined) {
		return;
	}

	const textEditor = vscode.window.activeTextEditor;

	if (textEditor === undefined) {
		return;
	}

	const queryText: string = textEditor.document.getText() ?? '';

	const globalState: vscode.Memento = this.globalState;
	const queryResultsWebviewMapping: Map<string, vscode.WebviewPanel> = this.queryResultsWebviewMapping;

	let uuid = QueryResultsMappingService.getQueryResultsMappingUuid(globalState, textEditor);
	if (!uuid) {
		uuid = uuidv4().substring(0, 8);
	}

	QueryResultsMappingService.upsertQueryResultsMapping(globalState, uuid, textEditor);

	const numberOfJobs = await runQuery(globalState, queryResultsWebviewMapping, uuid, activeTab.label, queryText);

	reporter?.sendTelemetryEvent('commandRunQuery', {}, { numberOfJobs: numberOfJobs, elapsedMs: Date.now() - t1 });

};

export const commandRunSelectedQuery = async function (this: any, ...args: any[]) {

	const t1 = Date.now();

	const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

	if (activeTab === undefined) {
		return;
	}

	const textEditor = vscode.window.activeTextEditor;

	if (textEditor === undefined) {
		return;
	}

	const queryText: string = textEditor.document.getText(textEditor.selection) ?? '';

	if (queryText.length === 0) {
		vscode.window.showErrorMessage('No text selected');
		return;
	}

	const globalState: vscode.Memento = this.globalState;
	const queryResultsWebviewMapping: Map<string, vscode.WebviewPanel> = this.queryResultsWebviewMapping;

	const uuid = uuidv4().substring(0, 8);

	const numberOfJobs = await runQuery(globalState, queryResultsWebviewMapping, uuid, activeTab.label, queryText);

	reporter?.sendTelemetryEvent('commandRunSelectedQuery', {}, { numberOfJobs: numberOfJobs, elapsedMs: Date.now() - t1 });

};

const runQuery = async function (globalState: vscode.Memento, queryResultsWebviewMapping: Map<string, vscode.WebviewPanel>, uuid: string, mainLabel: string, queryText: string): Promise<number> {

	const queryResponse = getBigQueryClient().runQuery(queryText);

	if (vscode.window.tabGroups.all.filter(c => c.viewColumn === vscode.ViewColumn.Two).length === 0) {
		await vscode.commands.executeCommand('workbench.action.editorLayoutTwoRows');
	}

	const label = `Result: ${mainLabel} | ${uuid}`;

	let panel = QueryResultsMappingService.getQueryResultsMappingWebviewPanel(queryResultsWebviewMapping, uuid);

	if (panel) {

		panel.reveal(undefined, true);

	} else {

		panel = vscode.window.createWebviewPanel("bigquery-query-results", label, { viewColumn: vscode.ViewColumn.Two, preserveFocus: true }, { enableFindWidget: true, enableScripts: true });

		QueryResultsMappingService.updateQueryResultsMappingWebviewPanel(queryResultsWebviewMapping, uuid, panel);

		//action when panel is closed
		panel.onDidDispose(e => {
			QueryResultsMappingService.deleteQueryResultsMapping(globalState, uuid);
		});

		//lock the tab group in vscode.ViewColumn.Two
		await vscode.commands.executeCommand('workbench.action.lockEditorGroup');

	}

	const resultsGridRender = new ResultsGridRender(panel.webview);

	const request = {
		jobsPromise: queryResponse,
		startIndex: 0,
		maxResults: 50,
		jobIndex: 0,
		openInTabVisible: true
	} as ResultsGridRenderRequest;

	resultsGridRender.render(request);

	QueryResultsMappingService.udpateQueryResultsMapping(globalState, uuid, request);

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

			resetBigQueryClient();

		});

	reporter?.sendTelemetryEvent('commandUserLogin', {});
};

export const commandUserLoginWithDrive = function (...args: any[]) {

	resetBigQueryClient();

	Authentication.userLoginWithDrive()
		.then(result => {
			if (result.valid) {
				vscode.window.showInformationMessage('Bigquery: User login - successful');
				vscode.commands.executeCommand(COMMAND_AUTHENTICATION_REFRESH);
			} else {
				vscode.window.showErrorMessage('Bigquery: User login - had invalid response');
				reporter?.sendTelemetryErrorEvent('commandUserLoginWithDrive', { error: 'Bigquery: User login - had invalid response' });
			}

			resetBigQueryClient();

		});

	reporter?.sendTelemetryEvent('commandUserLoginWithDrive', {});
};

export const commandServiceAccountLogin = async function (...args: any[]) {

	resetBigQueryClient();

	const showOpenDialogResult = await vscode.window.showOpenDialog({ canSelectFiles: true, canSelectMany: false, canSelectFolders: false });

	if (showOpenDialogResult) {

		let fileUri = showOpenDialogResult[0];
		const serviceAccountLoginResult = await Authentication.serviceAccountLogin(fileUri);

		if (serviceAccountLoginResult.valid) {
			vscode.window.showInformationMessage('Bigquery: Service account login - successful');
			vscode.commands.executeCommand(COMMAND_AUTHENTICATION_REFRESH);
		} else {
			vscode.window.showErrorMessage('Bigquery: Service account login - had invalid response');
			reporter?.sendTelemetryErrorEvent('commandUserLogin', { error: 'Bigquery: Service account login - had invalid response' });
		}

		resetBigQueryClient();
	}


	reporter?.sendTelemetryEvent('commandServiceAccountLogin', {});

};

export const commandGCloudInit = function (...args: any[]) {

	reporter?.sendTelemetryEvent('commandGCloudInit', {});

	resetBigQueryClient();

	const terminal = vscode.window.createTerminal("gcloud");

	terminal.show();

	terminal.sendText('gcloud init');

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

export const commandViewTable = async function (...args: any[]) {

	const t1 = Date.now();

	const item = args[0] as BigqueryTreeItem;

	const title = `${item.projectId}.${item.datasetId}.${item.tableId}`;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	const table = getBigQueryClient().getTable(item.projectId, item.datasetId, item.tableId);
	const metadata = await table.getMetadata();

	const panel = vscode.window.createWebviewPanel("vscode-bigquery-query-results", title, { viewColumn: vscode.ViewColumn.Active }, { enableFindWidget: true, enableScripts: true });
	const newresultsGridRender = new ResultsGridRender(panel.webview);

	if (metadata[0].type === 'EXTERNAL') {

		const queryResponse = getBigQueryClient().runQuery(
			`SELECT * FROM \`${item.projectId}.${item.datasetId}.${item.tableId}\``);

		const request = {
			jobsPromise: queryResponse,
			startIndex: 0,
			maxResults: 50,
			jobIndex: 0,
			openInTabVisible: false
		} as ResultsGridRenderRequest;

		newresultsGridRender.render(request);

	} else {

		const request = {
			table: table,
			startIndex: 0,
			maxResults: 50,
			jobIndex: 0,
			openInTabVisible: false
		} as TableGridRenderRequest;

		newresultsGridRender.renderTable(request);
	}

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

export const commandCreateTableDefaultQuery = async function (...args: any[]) {

	const t1 = Date.now();

	const item = args[0] as BigqueryTreeItem;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	const metadata = await getBigQueryClient().getMetadata(item.projectId, item.datasetId, item.tableId);
	const query = QueryGeneratorService.generateSelectQuery(metadata);

	const doc = await vscode.workspace.openTextDocument({
		language: 'bqsql',
		content: query
	});

	await vscode.commands.executeCommand<vscode.TextDocumentShowOptions>("vscode.open", doc.uri);

	reporter?.sendTelemetryEvent('commandCreateTableDefaultQuery', {}, { elapsedMs: Date.now() - t1 });

};

export const commandSetDefaultProject = function (...args: any[]) {

	resetBigQueryClient();

	const item = args[0] as BigqueryTreeItem;

	Authentication.setDefaultProjectId(item.projectId || 'xxx')
		.then(result => {
			vscode.commands.executeCommand(COMMAND_EXPLORER_REFRESH);

			resetBigQueryClient();
		});

	reporter?.sendTelemetryEvent('setDefaultProjectId', {});
};

export const commandDownloadCsv = async function (this: any, ...args: any[]) {

	const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

	if (activeTab === undefined || activeTab.input === undefined) {
		return;
	}

	if (!((activeTab.input as any).viewType as string).endsWith('-bigquery-query-results')) {
		return;
	}

	const uuid = activeTab.label.substring(activeTab.label.length - 8);

	const globalState: vscode.Memento = this.globalState;
	let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
	if (queryResultsMapping) {

		const item = queryResultsMapping.find(c => c.uuid === uuid);
		if (item && item.jobReferences && item.jobIndex !== undefined) {
			await DownloadCsv.download(getBigQueryClient(), item.jobReferences[item.jobIndex]);
		}

	}

	reporter?.sendTelemetryEvent('downloadCsv', {});
};

export const commandPinOrUnpinProject = function (...args: any[]) {

	const item = args[0] as BigqueryTreeItem;
	const projectId = item.projectId || 'xxx';

	let pinnedProjects = vscode.workspace
		.getConfiguration()
		.get(SETTING_PINNED_PROJECTS) as string[] || [];

	// let split = pinnedProjects.split(';');
	if (pinnedProjects.indexOf(projectId) >= 0) {
		pinnedProjects = pinnedProjects.filter(c => c && c !== projectId);
	} else {
		pinnedProjects.push(projectId);
	}

	vscode.workspace
		.getConfiguration()
		.update(SETTING_PINNED_PROJECTS, pinnedProjects);

	vscode.commands.executeCommand(COMMAND_EXPLORER_REFRESH);

	reporter?.sendTelemetryEvent('commandPinOrUnpinProject', {});
};

let bigQueryClient: BigQueryClient | null;

export const getBigQueryClient = function (): BigQueryClient {
	if (!bigQueryClient) {
		const t1 = Date.now();
		bigQueryClient = new BigQueryClient();
		reporter?.sendTelemetryEvent('CreateBigQueryClient', {}, { elapsedMs: Date.now() - t1 });
	}

	return bigQueryClient;
};

const resetBigQueryClient = function () {
	bigQueryClient = null;
};
