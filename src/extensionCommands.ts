import * as vscode from 'vscode';
import { BigQueryClient } from './services/bigqueryClient';
import { authenticationWebviewProvider, bigQueryTreeDataProvider, CHART_VIEW_TYPE, QUERY_RESULTS_VIEW_TYPE, reporter, TABLE_RESULTS_VIEW_TYPE, TROUBLESHOOT_VIEW_TYPE } from './extension';
import { ResultsGridRenderRequest } from './tableResultsPanel/resultsGridRenderRequest';
import { Authentication } from './services/authentication';
import { BigqueryTreeItem } from './activitybar/treeItem';
import { SchemaRender } from './tableResultsPanel/schemaRender';
import { QueryGeneratorService } from './services/queryGeneratorService';
import { ResultsGridRender } from './tableResultsPanel/resultsGridRender';
import { v4 as uuidv4 } from 'uuid';
import { DownloadCsv } from './tableResultsPanel/downloadCsv';
import { QueryResultsMappingService } from './services/queryResultsMappingService';
import { QueryResultsMapping } from './services/queryResultsMapping';
import { JobReference } from "./services/queryResultsMapping";
import { TableReference } from './services/tableMetadata';
import { ResultsChartRender } from './charts/resultsChartRender';
import { ResultsRender } from './services/resultsRender';
import { ResultsChartRenderRequest } from './charts/ResultsChartRenderRequest';
import { QueryResultsVisualizationType } from './services/queryResultsVisualizationType';
import { TelemetryEventProperties } from '@vscode/extension-telemetry';
import { TroubleshootSerializer } from './activitybar/troubleshootSerializer';
import { DownloadJsonl } from './tableResultsPanel/downloadJsonl';
import { BigQuery } from '@google-cloud/bigquery';
import { SendToPubsub } from './tableResultsPanel/sendToPubsub';

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
export const COMMAND_OPEN_DDL = "vscode-bigquery.open-ddl";
export const COMMAND_SET_DEFAULT_PROJECT = "vscode-bigquery.set-default-project";
export const COMMAND_PROJECT_PIN = "vscode-bigquery.project-pin";
export const COMMAND_DOWNLOAD_CSV = "vscode-bigquery.download-csv";
export const COMMAND_DOWNLOAD_JSONL = "vscode-bigquery.download-jsonl";
export const COMMAND_SEND_PUBSUB = "vscode-bigquery.send-pubsub";
export const COMMAND_PLOT_CHART = "vscode-bigquery.plot-chart";
export const SETTING_PINNED_PROJECTS = "vscode-bigquery.pinned-projects";
export const SETTING_PROJECTS = "vscode-bigquery.projects";
export const SETTING_TABLES = "vscode-bigquery.tables";
export const AUTHENTICATION_TROUBLESHOOT = "vscode-bigquery.troubleshoot";
export const OPEN_SETTING_PROJECTS = "vscode-bigquery.open-settings-projects";
export const OPEN_SETTING_TABLES = "vscode-bigquery.open-settings-tables";

export const commandRunQuery = async function (this: any, ...args: any[]) {

	return commandQuery(this, RunQueryType.query);

};

export const commandRunSelectedQuery = async function (this: any, ...args: any[]) {

	return commandQuery(this, RunQueryType.selectedQuery);

};

enum RunQueryType {
	query = 1,
	selectedQuery = 2
}

const commandQuery = async function (local: any, queryType: RunQueryType) {

	const t1 = Date.now();

	const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

	if (activeTab === undefined) {
		return;
	}

	const textEditor = vscode.window.activeTextEditor;
	if (textEditor === undefined) {
		return;
	}

	const queryText: string = (queryType === RunQueryType.query) ? textEditor.document.getText() ?? '' : textEditor.document.getText(textEditor.selection) ?? '';

	const globalState: vscode.Memento = local.globalState;
	const queryResultsWebviewMapping: Map<string, ResultsRender> = local.queryResultsWebviewMapping;

	let uuid = QueryResultsMappingService.getQueryResultsMappingUuid(globalState, textEditor, QueryResultsVisualizationType.table);
	if (!uuid) {
		uuid = uuidv4().substring(0, 8);
	}

	QueryResultsMappingService.upsertQueryResultsMapping(globalState, uuid, textEditor, QueryResultsVisualizationType.table);

	const numberOfJobs = await runQuery(globalState, queryResultsWebviewMapping, uuid, activeTab.label, queryText);

	reporter?.sendTelemetryEvent((queryType === RunQueryType.query) ? 'commandRunQuery' : 'commandRunSelectedQuery', {}, { numberOfJobs: numberOfJobs, elapsedMs: Date.now() - t1 });

};

const runQuery = async function (globalState: vscode.Memento, queryResultsWebviewMapping: Map<string, ResultsRender>, uuid: string, mainLabel: string, queryText: string): Promise<number> {

	const queryResponse = getBigQueryClient().runQuery(queryText);

	let performLock = false;
	if (vscode.window.tabGroups.all.filter(c => c.viewColumn === vscode.ViewColumn.Two).length === 0) {
		await vscode.commands.executeCommand('workbench.action.editorLayoutTwoRows');
		performLock = true;
	}

	const label = `Visualization: ${mainLabel} | ${uuid}`;

	let resultsGridRender = QueryResultsMappingService.getQueryResultsMappingResultsGridRender(queryResultsWebviewMapping, uuid);

	if (resultsGridRender) {

		resultsGridRender.reveal(undefined, true);

	} else {

		const panel = vscode.window.createWebviewPanel(QUERY_RESULTS_VIEW_TYPE, label, { viewColumn: vscode.ViewColumn.Two, preserveFocus: true }, { enableFindWidget: true, enableScripts: true });
		resultsGridRender = new ResultsGridRender(panel);

		//lock the tab group in vscode.ViewColumn.Two
		if (performLock) {
			panel.reveal(undefined, false);
			await vscode.commands.executeCommand('workbench.action.lockEditorGroup');
			await vscode.commands.executeCommand("workbench.action.focusPreviousGroup");
		}

		QueryResultsMappingService.updateQueryResultsMappingWebviewPanel(queryResultsWebviewMapping, uuid, resultsGridRender);

		//action when panel is closed
		panel.onDidDispose(e => {
			QueryResultsMappingService.deleteQueryResultsMapping(globalState, uuid);
		});

	}

	try {
		resultsGridRender.renderLoadingIcon();

		const jobReferences = (await queryResponse).map(c => { return { jobId: c.id, projectId: c.projectId, location: c.location } as JobReference; });

		const request = {
			jobReferences: jobReferences,
			startIndex: 0,
			maxResults: 50,
			jobIndex: 0,
			openInTabVisible: true
		} as ResultsGridRenderRequest;

		resultsGridRender.render(request);

		QueryResultsMappingService.updateQueryResultsMapping(globalState, uuid, request);

		return (await queryResponse).length;
	} catch (error) {
		resultsGridRender.renderException(error);
	}

	return 0;
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

	let panel: vscode.WebviewPanel;
	if (args.length > 1 && args[1] && args[1].viewType === TABLE_RESULTS_VIEW_TYPE) {
		panel = args[1];
	} else {
		panel = vscode.window.createWebviewPanel(TABLE_RESULTS_VIEW_TYPE, title, { viewColumn: vscode.ViewColumn.Active }, { enableFindWidget: true, enableScripts: true });
	}

	const newresultsGridRender = new ResultsGridRender(panel);

	if (metadata[0].type === 'EXTERNAL' || metadata[0].type === 'VIEW') {
		try {
			const queryResponse = await getBigQueryClient().runQuery(
				`SELECT * FROM \`${item.projectId}.${item.datasetId}.${item.tableId}\``);

			const jobReferences = [
				{
					jobId: queryResponse[0].id,
					location: queryResponse[0].location,
					projectId: queryResponse[0].projectId
				} as JobReference];

			const request = {
				jobReferences: jobReferences,
				startIndex: 0,
				maxResults: 50,
				jobIndex: 0,
				openInTabVisible: false
			} as ResultsGridRenderRequest;

			newresultsGridRender.render(request);
		} catch (error) {
			newresultsGridRender.renderException(error);
		}

	} else {

		const request = {
			tableReference: { projectId: item.projectId, datasetId: item.datasetId, tableId: item.tableId } as TableReference,
			startIndex: 0,
			maxResults: 50,
			jobIndex: 0,
			openInTabVisible: false
		} as ResultsGridRenderRequest;

		newresultsGridRender.render(request);
	}

	reporter?.sendTelemetryEvent('commandViewTable', {}, { elapsedMs: Date.now() - t1 });
}; 0

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

	let query = QueryGeneratorService.generateSelectQuerySimple(item.projectId, item.datasetId, item.tableId);
	try {

		const metadata = await getBigQueryClient().getMetadata(item.projectId, item.datasetId, item.tableId);
		query = QueryGeneratorService.generateSelectQuery(metadata);
	} catch (error) { }

	const doc = await vscode.workspace.openTextDocument({
		language: 'bqsql',
		content: query
	});

	await vscode.commands.executeCommand<vscode.TextDocumentShowOptions>("vscode.open", doc.uri);

	reporter?.sendTelemetryEvent('commandCreateTableDefaultQuery', {}, { elapsedMs: Date.now() - t1 });

};

export const commandOpenDdl = async function (...args: any[]) {

	const t1 = Date.now();

	const item = args[0] as BigqueryTreeItem;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	try {
		let query = QueryGeneratorService.generateDdlQuery(item);

		const queryRun = await getBigQueryClient().runQuery(query);
		const queryResult = await queryRun[0].getQueryResults();
		const ddl = queryResult[0][0].ddl;

		const doc = await vscode.workspace.openTextDocument({
			language: 'bqsql',
			content: ddl
		});

		await vscode.commands.executeCommand<vscode.TextDocumentShowOptions>("vscode.open", doc.uri);

	} catch (error) {
		vscode.window.showErrorMessage(JSON.stringify(error));
	}

	reporter?.sendTelemetryEvent('commandOpenDdl', {}, { elapsedMs: Date.now() - t1 });

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

	const viewType = ((activeTab.input as any).viewType as string);
	if (viewType?.endsWith('-bigquery-query-results')) {

		const uuid = activeTab.label.substring(activeTab.label.length - 8);

		const globalState: vscode.Memento = this.globalState;
		let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
		if (queryResultsMapping) {

			const item = queryResultsMapping.find(c => c.uuid === uuid);
			if (item && item.jobReferences && item.jobIndex !== undefined) {
				await DownloadCsv.download(getBigQueryClient(), item.jobReferences[item.jobIndex]);
			}
		}
	} else {
		if (viewType?.endsWith('-bigquery-table-results')) {

			const tableId = activeTab.label.split('.');
			const table = getBigQueryClient().getTable(tableId[0], tableId[1], tableId[2]);

			await DownloadCsv.downloadTable(getBigQueryClient(), table);

		}
	}

	const telemetryProperties: TelemetryEventProperties = {
		"button": (args.length > 0 && typeof (args[0]) === "string" ? args[0] : 'webViewPanel')
	};

	reporter?.sendTelemetryEvent('commandDownloadCsv', telemetryProperties);
};

export const commandDownloadJsonl = async function (this: any, ...args: any[]) {

	const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

	if (activeTab === undefined || activeTab.input === undefined) {
		return;
	}

	const viewType = ((activeTab.input as any).viewType as string);
	if (viewType?.endsWith('-bigquery-query-results')) {

		const uuid = activeTab.label.substring(activeTab.label.length - 8);

		const globalState: vscode.Memento = this.globalState;
		let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
		if (queryResultsMapping) {

			const item = queryResultsMapping.find(c => c.uuid === uuid);
			if (item && item.jobReferences && item.jobIndex !== undefined) {
				await DownloadJsonl.download(getBigQueryClient(), item.jobReferences[item.jobIndex]);
			}
		}
	} else {
		if (viewType?.endsWith('-bigquery-table-results')) {

			const tableId = activeTab.label.split('.');
			const table = getBigQueryClient().getTable(tableId[0], tableId[1], tableId[2]);

			await DownloadJsonl.downloadTable(getBigQueryClient(), table);

		}
	}

	const telemetryProperties: TelemetryEventProperties = {
		"button": (args.length > 0 && typeof (args[0]) === "string" ? args[0] : 'webViewPanel')
	};

	reporter?.sendTelemetryEvent('commandDownloadJsonl', telemetryProperties);
};

export const commandSendPubsub = async function (this: any, ...args: any[]) {

	const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

	if (activeTab === undefined || activeTab.input === undefined) {
		return;
	}

	const viewType = ((activeTab.input as any).viewType as string);
	if (viewType?.endsWith('-bigquery-query-results')) {

		const uuid = activeTab.label.substring(activeTab.label.length - 8);

		const globalState: vscode.Memento = this.globalState;
		let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
		if (queryResultsMapping) {

			const item = queryResultsMapping.find(c => c.uuid === uuid);
			if (item && item.jobReferences && item.jobIndex !== undefined) {
				await SendToPubsub.sendJobResult(getBigQueryClient(), item.jobReferences[item.jobIndex]);
			}
		}
	}
	//  else {
	// 	if (viewType?.endsWith('-bigquery-table-results')) {

	// 		const tableId = activeTab.label.split('.');
	// 		const table = getBigQueryClient().getTable(tableId[0], tableId[1], tableId[2]);

	// 		await DownloadJsonl.downloadTable(getBigQueryClient(), table);

	// 	}
	// }

	const telemetryProperties: TelemetryEventProperties = {
		"button": (args.length > 0 && typeof (args[0]) === "string" ? args[0] : 'webViewPanel')
	};

	reporter?.sendTelemetryEvent('commandDownloadJsonl', telemetryProperties);
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

export const commandPlotChart = async function (this: any, ...args: any[]) {

	const t1 = Date.now();

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
	const queryResultsWebviewMapping: Map<string, ResultsRender> = this.queryResultsWebviewMapping;

	let uuid = QueryResultsMappingService.getQueryResultsMappingUuid(globalState, textEditor, QueryResultsVisualizationType.chart);
	if (!uuid) {
		uuid = uuidv4().substring(0, 8);
	}

	QueryResultsMappingService.upsertQueryResultsMapping(globalState, uuid, textEditor, QueryResultsVisualizationType.chart);

	const numberOfJobs = await runQueryToChart(globalState, queryResultsWebviewMapping, uuid, activeTab.label, queryText);

	reporter?.sendTelemetryEvent('commandPlotChart', {}, { numberOfJobs: numberOfJobs, elapsedMs: Date.now() - t1 });

};

export const commandAuthenticationTroubleshoot = async function (this: any, ...args: any[]) {

	const t1 = Date.now();

	const panel = vscode.window.createWebviewPanel(
		TROUBLESHOOT_VIEW_TYPE,
		'Troubleshoot',
		vscode.ViewColumn.One,
		{ retainContextWhenHidden: true }
	);

	panel.webview.html = TroubleshootSerializer.getTroubleshootHtml(panel);

	reporter?.sendTelemetryEvent('commandAuthenticationTroubleshoot', {}, { elapsedMs: Date.now() - t1 });

};

export const commandOpenSettingProjects = async function (this: any, ...args: any[]) {

	const t1 = Date.now();

	vscode.commands.executeCommand('workbench.action.openWorkspaceSettings', 'vscode-bigquery.projects');

	reporter?.sendTelemetryEvent('commandOpenSettingProjects', {}, { elapsedMs: Date.now() - t1 });

};

export const commandOpenSettingTables = async function (this: any, ...args: any[]) {

	const t1 = Date.now();

	vscode.commands.executeCommand('workbench.action.openWorkspaceSettings', 'vscode-bigquery.tables');

	reporter?.sendTelemetryEvent('commandOpenSettingTables', {}, { elapsedMs: Date.now() - t1 });

};

const runQueryToChart = async function (globalState: vscode.Memento, queryResultsWebviewMapping: Map<string, ResultsRender>, uuid: string, mainLabel: string, queryText: string): Promise<number> {

	const label = `Visualization: ${mainLabel} | ${uuid}`;

	let resultsChartRender = QueryResultsMappingService.getQueryResultsMappingResultsChartRender(queryResultsWebviewMapping, uuid);

	if (resultsChartRender) {

		resultsChartRender.reveal(undefined, true);

	} else {

		const panel = vscode.window.createWebviewPanel(CHART_VIEW_TYPE, label, { viewColumn: vscode.ViewColumn.Two, preserveFocus: true }, { enableFindWidget: true, enableScripts: true });
		resultsChartRender = new ResultsChartRender(panel);

		QueryResultsMappingService.updateQueryResultsChartMappingWebviewPanel(queryResultsWebviewMapping, uuid, resultsChartRender);

		//action when panel is closed
		panel.onDidDispose(e => {
			QueryResultsMappingService.deleteQueryResultsMapping(globalState, uuid);
		});

	}

	try {

		resultsChartRender.renderLoadingIcon();

		const queryResponse = getBigQueryClient().runQuery(queryText);
		const jobReferences = (await queryResponse).map(c => { return { jobId: c.id, projectId: c.projectId, location: c.location } as JobReference; });

		const request = {
			jobReferences: jobReferences,
			jobIndex: 0,
		} as ResultsChartRenderRequest;

		resultsChartRender.render(request);

		QueryResultsMappingService.updateQueryResultsMapping(globalState, uuid, request);

		return (await queryResponse).length;
	} catch (error) {
		resultsChartRender.renderException(error);
	}

	return 0;
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
