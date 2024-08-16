import * as vscode from 'vscode';
import { BigQueryClient } from './services/bigqueryClient';
import { bigQueryTreeDataProvider, QUERY_RESULTS_VIEW_TYPE, TABLE_RESULTS_VIEW_TYPE, TROUBLESHOOT_VIEW_TYPE, gcpAuthenticationTreeDataProvider } from './extension';
// import { ResultsGridRenderRequest } from './tableResultsPanel/resultsGridRenderRequest';
import { Authentication } from './services/authentication';
import { BigqueryTreeItem, BigqueryTreeItemType } from './activitybar/bigqueryTreeItem';
// import { SchemaRender } from './tableResultsPanel/schemaRender';
import { QueryGeneratorService } from './services/queryGeneratorService';
import { ResultsGridRender } from './tableResultsPanel/resultsGridRender';
import { v4 as uuidv4 } from 'uuid';
import { DownloadCsv } from './tableResultsPanel/downloadCsv';
import { QueryResultsMappingService } from './services/queryResultsMappingService';
import { QueryResultsMapping } from './services/queryResultsMapping';
// import { JobReference } from "./services/queryResultsMapping";
// import { TableReference } from './services/tableMetadata';
// import { ResultsChartRender } from './charts/resultsChartRender';
import { ResultsRender } from './services/resultsRender';
// import { ResultsChartRenderRequest } from './charts/ResultsChartRenderRequest';
import { QueryResultsVisualizationType } from './services/queryResultsVisualizationType';
// import { TelemetryEventProperties } from '@vscode/extension-telemetry';
import { TroubleshootSerializer } from './activitybar/troubleshootSerializer';
import { DownloadJsonl } from './tableResultsPanel/downloadJsonl';
import { SendToPubsub } from './tableResultsPanel/sendToPubsub';
// import { Job } from '@google-cloud/bigquery';
import { ResultsGridRenderRequestV2, ResultsGridRenderRequestV2Type } from './tableResultsPanel/resultsGridRenderRequestV2';
import { AuthenticationTreeItem, AuthenticationTreeItemType } from './activitybar/authenticationTreeItem';
import { Dataset, Table } from '@google-cloud/bigquery';

export const COMMAND_RUN_QUERY = "vscode-bigquery.run-query";
export const COMMAND_RUN_SELECTED_QUERY = "vscode-bigquery.run-selected-query";
export const COMMAND_USER_LOGIN = "vscode-bigquery.user-login";
export const COMMAND_USER_LOGIN_WITH_DRIVE = "vscode-bigquery.user-login-drive";
export const COMMAND_USER_LOGIN_NO_LAUNCH_BROWSER = "vscode-bigquery.user-login-no-launch-browser";
export const COMMAND_USER_ACTIVATE = "vscode-bigquery.gcp-user-activate";
export const COMMAND_USER_REMOVE = "vscode-bigquery.gcp-user-remove";
export const COMMAND_GCLOUD_INIT = "vscode-bigquery.gcloud-init";
export const COMMAND_SERVICE_ACCOUNT_LOGIN = "vscode-bigquery.service-account-login";
export const COMMAND_AUTHENTICATION_REFRESH = "vscode-bigquery.authentication-refresh";
export const COMMAND_EXPLORER_REFRESH = "vscode-bigquery.explorer-refresh";
export const COMMAND_VIEW_TABLE = "vscode-bigquery.view-table";
// export const COMMAND_VIEW_TABLE_SCHEMA = "vscode-bigquery.view-table-schema";
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

	// getTelemetryReporter()?.sendTelemetryEvent((queryType === RunQueryType.query) ? 'commandRunQuery' : 'commandRunSelectedQuery', {}, { numberOfJobs: numberOfJobs, elapsedMs: Date.now() - t1 });

};

const runQuery = async function (globalState: vscode.Memento, queryResultsWebviewMapping: Map<string, ResultsRender>, uuid: string, mainLabel: string, queryText: string): Promise<number> {

	// const queryResponse = getBigQueryClient().runQuery(queryText);

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

		await resultsGridRender.render1();

		QueryResultsMappingService.updateQueryResultsMappingWebviewPanel(queryResultsWebviewMapping, uuid, resultsGridRender);

		//action when panel is closed
		panel.onDidDispose(e => {
			QueryResultsMappingService.deleteQueryResultsMapping(globalState, uuid);
		});
	}

	try {
		let _postMessageResult1 = await resultsGridRender.postMessage({
			requestType: ResultsGridRenderRequestV2Type.clear.toString(),
			projectId: null,
			token: null,
			job: null,
			error: null
		} as ResultsGridRenderRequestV2);

		const bqClient = await getBigQueryClient();
		const projectId = await bqClient.getProjectId();
		// console.log('projectId:', projectId);
		const token = await bqClient.getToken();
		// console.log('token:', token);
		const job = await bqClient.runQuery(queryText);

		// const jobReferences = job.map(c => { return { jobId: c.id, projectId: c.projectId, location: c.location } as JobReference; });


		let _postMessageResult2 = await resultsGridRender.postMessage({
			requestType: ResultsGridRenderRequestV2Type.executeQuery.toString(),
			projectId: projectId,
			token: token,
			job: job.metadata,
			error: null
		} as ResultsGridRenderRequestV2);

	} catch (errorx) {
		// resultsGridRender.renderException(error);
		const error =
		{
			message: (errorx as any).message || 'undefined message',
			reason: ''
		};

		let _postMessageResult3 = await resultsGridRender.postMessage({
			requestType: ResultsGridRenderRequestV2Type.error.toString(),
			projectId: null,
			token: null,
			job: null,
			error: error
		} as ResultsGridRenderRequestV2);
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
				// getTelemetryReporter()?.sendTelemetryErrorEvent('commandUserLogin', { error: 'Bigquery: User login - had invalid response' });
			}

			resetBigQueryClient();

		});

	// getTelemetryReporter()?.sendTelemetryEvent('commandUserLogin', {});
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
				// getTelemetryReporter()?.sendTelemetryErrorEvent('commandUserLoginWithDrive', { error: 'Bigquery: User login - had invalid response' });
			}

			resetBigQueryClient();

		});

	// getTelemetryReporter()?.sendTelemetryEvent('commandUserLoginWithDrive', {});
};

export const commandUserLoginNoLaunchBrowser = function (...args: any[]) {

	// getTelemetryReporter()?.sendTelemetryEvent('commandUserLoginNoLaunchBrowser', {});

	resetBigQueryClient();

	const terminal = vscode.window.createTerminal("gcloud");

	terminal.show();

	terminal.sendText('gcloud auth login --update-adc --add-quota-project-to-adc --quiet --verbosity warning --no-launch-browser');
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
			// getTelemetryReporter()?.sendTelemetryErrorEvent('commandUserLogin', { error: 'Bigquery: Service account login - had invalid response' });
		}

		resetBigQueryClient();
	}

	// getTelemetryReporter()?.sendTelemetryEvent('commandServiceAccountLogin', {});

};

export const commandGcpUserActivate = async function (...args: any[]) {

	resetBigQueryClient();

	const item = args[0] as AuthenticationTreeItem;

	Authentication.activate(item.label)
		.then(result => {
			vscode.commands.executeCommand(COMMAND_AUTHENTICATION_REFRESH);
		});

	// getTelemetryReporter()?.sendTelemetryEvent('commandGcpUserActivate', {});
};

export const commandGcpUserRemove = async function (...args: any[]) {

	resetBigQueryClient();

	const item = args[0] as AuthenticationTreeItem;

	Authentication.revoke(item.label)
		.then(result => {
			vscode.commands.executeCommand(COMMAND_AUTHENTICATION_REFRESH);
		});

	// getTelemetryReporter()?.sendTelemetryEvent('commandGcpUserRemove', {});
};

export const commandGCloudInit = function (...args: any[]) {

	// getTelemetryReporter()?.sendTelemetryEvent('commandGCloudInit', {});

	resetBigQueryClient();

	const terminal = vscode.window.createTerminal("gcloud");

	terminal.show();

	terminal.sendText('gcloud init');

};

export const commandAuthenticationRefresh = function (...args: any[]) {

	const t1 = Date.now();

	resetBigQueryClient();

	gcpAuthenticationTreeDataProvider.refresh();

	// getTelemetryReporter()?.sendTelemetryEvent('commandAuthenticationRefresh', {}, { elapsedMs: Date.now() - t1 });
};

export const commandExplorerRefresh = function (...args: any[]) {

	const t1 = Date.now();

	bigQueryTreeDataProvider.refresh();

	// getTelemetryReporter()?.sendTelemetryEvent('commandExplorerRefresh', {}, { elapsedMs: Date.now() - t1 });
};

export const commandViewTable = async function (...args: any[]) {

	const t1 = Date.now();

	const item = args[0] as BigqueryTreeItem;

	const title = `${item.projectId}.${item.datasetId}.${item.tableId}`;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	if (item.treeItemType === BigqueryTreeItemType.tableView) {

		await openQueryEditor(item);

	} else {

		const bqClient = await getBigQueryClient();

		const table = bqClient.getTable(item.projectId, item.datasetId, item.tableId);
		const metadata = await table.getMetadata();

		if (metadata[0].type === 'EXTERNAL') {
			await openQueryEditor(item);
		} else {

			let panel: vscode.WebviewPanel;
			if (args.length > 1 && args[1] && args[1].viewType === TABLE_RESULTS_VIEW_TYPE) {
				panel = args[1];
			} else {
				panel = vscode.window.createWebviewPanel(TABLE_RESULTS_VIEW_TYPE, title, { viewColumn: vscode.ViewColumn.Active }, { enableFindWidget: true, enableScripts: true });
			}

			const resultsGridRender = new ResultsGridRender(panel);

			await resultsGridRender.render1();

			// 	const request = {
			// 		tableReference: { projectId: item.projectId, datasetId: item.datasetId, tableId: item.tableId } as TableReference,
			// 		startIndex: 0,
			// 		maxResults: 50,
			// 		jobIndex: 0,
			// 		openInTabVisible: false
			// 	} as ResultsGridRenderRequest;

			// 	newresultsGridRender.render(request);

			try {
				let _postMessageResult1 = await resultsGridRender.postMessage({
					requestType: ResultsGridRenderRequestV2Type.clear.toString(),
					projectId: null,
					token: null,
					job: null,
					error: null
				} as ResultsGridRenderRequestV2);

				const bqClient = await getBigQueryClient();
				// const projectId = await bqClient.getProjectId();
				// console.log('projectId:', projectId);
				const token = await bqClient.getToken();
				// console.log('token:', token);
				// const job = await bqClient.runQuery(queryText);
				const projectId = item.projectId;
				const datasetId = item.datasetId;
				const tableId = item.tableId;

				// const jobReferences = job.map(c => { return { jobId: c.id, projectId: c.projectId, location: c.location } as JobReference; });

				let _postMessageResult2 = await resultsGridRender.postMessage({
					requestType: ResultsGridRenderRequestV2Type.previewTable.toString(),
					projectId: projectId,
					datasetId: datasetId,
					tableId: tableId,
					token: token,
					job: null,
					error: null
				} as ResultsGridRenderRequestV2);

			} catch (errorx) {
				// resultsGridRender.renderException(error);
				const error =
				{
					message: (errorx as any).message || 'undefined message',
					reason: ''
				};

				let _postMessageResult3 = await resultsGridRender.postMessage({
					requestType: ResultsGridRenderRequestV2Type.error.toString(),
					projectId: null,
					token: null,
					job: null,
					error: error
				} as ResultsGridRenderRequestV2);
			}
		}
	}

	// getTelemetryReporter()?.sendTelemetryEvent('commandViewTable', {}, { elapsedMs: Date.now() - t1 });
};

async function openQueryEditor(item: BigqueryTreeItem) {
	const query = `SELECT * \nFROM \`${item.projectId}.${item.datasetId}.${item.tableId}\``;

	const doc = await vscode.workspace.openTextDocument({
		language: 'bqsql',
		content: query
	});

	doc.positionAt(7);
}

// export const commandViewTableSchema = async function (...args: any[]) {

// 	const t1 = Date.now();

// 	const item = args[0] as BigqueryTreeItem;

// 	const title = `Schema: ${item.projectId}.${item.datasetId}.${item.tableId}`;

// 	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
// 		return;
// 	}
// 	const bqClient = await getBigQueryClient();

// 	const metadataPromise = bqClient.getMetadata(item.projectId, item.datasetId, item.tableId);
// 	const panel = vscode.window.createWebviewPanel("vscode-bigquery-table-schema", title, { viewColumn: vscode.ViewColumn.Active }, { enableFindWidget: true, enableScripts: true });
// 	const schemaRender = new SchemaRender(panel.webview);

// 	schemaRender.render(metadataPromise);

// 	// getTelemetryReporter()?.sendTelemetryEvent('commandViewTableSchema', {}, { elapsedMs: Date.now() - t1 });

// };

export const commandCreateTableDefaultQuery = async function (...args: any[]) {

	const t1 = Date.now();

	const item = args[0] as BigqueryTreeItem;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	let query = QueryGeneratorService.generateSelectQuerySimple(item.projectId, item.datasetId, item.tableId);
	try {
		const bqClient = await getBigQueryClient();
		const metadata = await bqClient.getMetadata(item.projectId, item.datasetId, item.tableId);
		query = QueryGeneratorService.generateSelectQuery(metadata);
	} catch (error) { }

	const doc = await vscode.workspace.openTextDocument({
		language: 'bqsql',
		content: query
	});

	await vscode.commands.executeCommand<vscode.TextDocumentShowOptions>("vscode.open", doc.uri);

	// getTelemetryReporter()?.sendTelemetryEvent('commandCreateTableDefaultQuery', {}, { elapsedMs: Date.now() - t1 });

};

export const commandOpenDdl = async function (...args: any[]) {

	const t1 = Date.now();

	const item = args[0] as BigqueryTreeItem;

	if (item.projectId === null || item.datasetId === null || item.tableId === null) {
		return;
	}

	try {

		let query = QueryGeneratorService.generateDdlQuery(item);
		const bqClient = await getBigQueryClient();

		const queryRun = await bqClient.runQuery(query);
		const queryResult = await queryRun.getQueryResults();
		const ddl = queryResult[0][0].ddl;

		const doc = await vscode.workspace.openTextDocument({
			language: 'bqsql',
			content: ddl
		});

		await vscode.commands.executeCommand<vscode.TextDocumentShowOptions>("vscode.open", doc.uri);

	} catch (error) {
		vscode.window.showErrorMessage(JSON.stringify(error));
	}

	// getTelemetryReporter()?.sendTelemetryEvent('commandOpenDdl', {}, { elapsedMs: Date.now() - t1 });

};

export const commandSetDefaultProject = function (...args: any[]) {

	resetBigQueryClient();

	const item = args[0] as BigqueryTreeItem;

	Authentication.setDefaultProjectId(item.projectId || 'xxx')
		.then(result => {
			vscode.commands.executeCommand(COMMAND_EXPLORER_REFRESH);

			resetBigQueryClient();
		});

	// getTelemetryReporter()?.sendTelemetryEvent('setDefaultProjectId', {});
};

export const commandDownloadCsv = async function (this: any, ...args: any[]) {

	if (args.length > 0) {

		let data = args[0];
		if (data.command === "download_csv") {

			if (data.jobReference || data.tableReference) {

				const bqClient = await getBigQueryClient();

				if (data.jobReference) {
					let jobReference = data.jobReference;
					await DownloadCsv.download(bqClient, jobReference);
				} else {
					let tableReference = data.tableReference;

					const table = bqClient.getTable(tableReference.projectId, tableReference.datasetId, tableReference.tableId);


					await DownloadCsv.downloadTable(bqClient, table);
				}
			}

		}

	}

	// const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

	// if (activeTab === undefined || activeTab.input === undefined) {
	// 	return;
	// }
	// const bqClient = await getBigQueryClient();

	// const viewType = ((activeTab.input as any).viewType as string);
	// if (viewType?.endsWith('-bigquery-query-results')) {

	// 	const uuid = activeTab.label.substring(activeTab.label.length - 8);

	// 	const globalState: vscode.Memento = this.globalState;
	// 	let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
	// 	if (queryResultsMapping) {

	// 		const item = queryResultsMapping.find(c => c.uuid === uuid);
	// 		if (item && item.jobReferences && item.jobIndex !== undefined) {
	// 			await DownloadCsv.download(bqClient, item.jobReferences[item.jobIndex]);
	// 		}
	// 	}
	// } else {
	// 	if (viewType?.endsWith('-bigquery-table-results')) {

	// 		const tableId = activeTab.label.split('.');
	// 		const table = bqClient.getTable(tableId[0], tableId[1], tableId[2]);

	// 		await DownloadCsv.downloadTable(bqClient, table);

	// 	}
	// }

	// const telemetryProperties: TelemetryEventProperties = {
	// 	"button": (args.length > 0 && typeof (args[0]) === "string" ? args[0] : 'webViewPanel')
	// };

	// getTelemetryReporter()?.sendTelemetryEvent('commandDownloadCsv', telemetryProperties);
};

export const commandDownloadJsonl = async function (this: any, ...args: any[]) {

	if (args.length > 0) {

		let data = args[0];
		if (data.command === "download_jsonl") {

			if (data.jobReference || data.tableReference) {

				const bqClient = await getBigQueryClient();

				if (data.jobReference) {
					let jobReference = data.jobReference;
					await DownloadJsonl.download(bqClient, jobReference);
				} else {
					let tableReference = data.tableReference;

					const table = bqClient.getTable(tableReference.projectId, tableReference.datasetId, tableReference.tableId);


					await DownloadJsonl.downloadTable(bqClient, table);
				}
			}


			// const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

			// if (activeTab === undefined || activeTab.input === undefined) {
			// 	return;
			// }

			// const viewType = ((activeTab.input as any).viewType as string);
			// const bqClient = await getBigQueryClient();

			// if (viewType?.endsWith('-bigquery-query-results')) {

			// 	const uuid = activeTab.label.substring(activeTab.label.length - 8);

			// 	const globalState: vscode.Memento = this.globalState;
			// 	let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
			// 	if (queryResultsMapping) {

			// 		const item = queryResultsMapping.find(c => c.uuid === uuid);
			// 		if (item && item.jobReferences && item.jobIndex !== undefined) {
			// 			await DownloadJsonl.download(bqClient, item.jobReferences[item.jobIndex]);
			// 		}
			// 	}
			// } else {
			// 	if (viewType?.endsWith('-bigquery-table-results')) {

			// 		const tableId = activeTab.label.split('.');
			// 		const table = bqClient.getTable(tableId[0], tableId[1], tableId[2]);

			// 		await DownloadJsonl.downloadTable(bqClient, table);

			// 	}
			// }

			// const telemetryProperties: TelemetryEventProperties = {
			// 	"button": (args.length > 0 && typeof (args[0]) === "string" ? args[0] : 'webViewPanel')
			// };
			// getTelemetryReporter()?.sendTelemetryEvent('commandDownloadJsonl', telemetryProperties);
		}
	}
};

export const commandSendPubsub = async function (this: any, ...args: any[]) {

	if (args.length > 0) {

		let data = args[0];
		if (data.command === "send_pubsub") {
			if (data.jobReference) {
				const bqClient = await getBigQueryClient();

				let jobReference = data.jobReference;
				await SendToPubsub.sendJobResult(bqClient, jobReference);
			}

			// const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

			// if (activeTab === undefined || activeTab.input === undefined) {
			// 	return;
			// }

			// const viewType = ((activeTab.input as any).viewType as string);
			// if (viewType?.endsWith('-bigquery-query-results')) {

			// 	const uuid = activeTab.label.substring(activeTab.label.length - 8);

			// 	const globalState: vscode.Memento = this.globalState;
			// 	let queryResultsMapping: QueryResultsMapping[] | undefined = globalState.get('queryResultsMapping');
			// 	if (queryResultsMapping) {

			// 		const item = queryResultsMapping.find(c => c.uuid === uuid);
			// 		if (item && item.jobReferences && item.jobIndex !== undefined) {
			// 			const bqClient = await getBigQueryClient();
			// 			await SendToPubsub.sendJobResult(bqClient, item.jobReferences[item.jobIndex]);
			// 		}
			// 	}
			// }
			// //  else {
			// // 	if (viewType?.endsWith('-bigquery-table-results')) {

			// // 		const tableId = activeTab.label.split('.');
			// // 		const table = getBigQueryClient().getTable(tableId[0], tableId[1], tableId[2]);

			// // 		await DownloadJsonl.downloadTable(getBigQueryClient(), table);

			// // 	}
			// // }

			// const telemetryProperties: TelemetryEventProperties = {
			// 	"button": (args.length > 0 && typeof (args[0]) === "string" ? args[0] : 'webViewPanel')
			// };

			// getTelemetryReporter()?.sendTelemetryEvent('commandSendPubsub', telemetryProperties);
		}
	}
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

	// getTelemetryReporter()?.sendTelemetryEvent('commandPinOrUnpinProject', {});
};

// export const commandPlotChart = async function (this: any, ...args: any[]) {

// 	const t1 = Date.now();

// 	const activeTab = vscode.window.tabGroups.activeTabGroup.activeTab;

// 	if (activeTab === undefined) {
// 		return;
// 	}

// 	const textEditor = vscode.window.activeTextEditor;
// 	if (textEditor === undefined) {
// 		return;
// 	}

// 	const queryText: string = textEditor.document.getText() ?? '';

// 	const globalState: vscode.Memento = this.globalState;
// 	const queryResultsWebviewMapping: Map<string, ResultsRender> = this.queryResultsWebviewMapping;

// 	let uuid = QueryResultsMappingService.getQueryResultsMappingUuid(globalState, textEditor, QueryResultsVisualizationType.chart);
// 	if (!uuid) {
// 		uuid = uuidv4().substring(0, 8);
// 	}

// 	QueryResultsMappingService.upsertQueryResultsMapping(globalState, uuid, textEditor, QueryResultsVisualizationType.chart);

// 	const numberOfJobs = await runQueryToChart(globalState, queryResultsWebviewMapping, uuid, activeTab.label, queryText);

// 	getTelemetryReporter()?.sendTelemetryEvent('commandPlotChart', {}, { numberOfJobs: numberOfJobs, elapsedMs: Date.now() - t1 });

// };

export const commandAuthenticationTroubleshoot = async function (this: any, ...args: any[]) {

	const t1 = Date.now();

	const panel = vscode.window.createWebviewPanel(
		TROUBLESHOOT_VIEW_TYPE,
		'Troubleshoot',
		vscode.ViewColumn.One,
		{ retainContextWhenHidden: true }
	);

	panel.webview.html = TroubleshootSerializer.getTroubleshootHtml(panel);

	// getTelemetryReporter()?.sendTelemetryEvent('commandAuthenticationTroubleshoot', {}, { elapsedMs: Date.now() - t1 });

};

export const commandOpenSettingProjects = async function (this: any, ...args: any[]) {

	const t1 = Date.now();

	vscode.commands.executeCommand('workbench.action.openWorkspaceSettings', 'vscode-bigquery.projects');

	// getTelemetryReporter()?.sendTelemetryEvent('commandOpenSettingProjects', {}, { elapsedMs: Date.now() - t1 });

};

export const commandOpenSettingTables = async function (this: any, ...args: any[]) {

	const t1 = Date.now();

	vscode.commands.executeCommand('workbench.action.openWorkspaceSettings', 'vscode-bigquery.tables');

	// getTelemetryReporter()?.sendTelemetryEvent('commandOpenSettingTables', {}, { elapsedMs: Date.now() - t1 });

};

// const runQueryToChart = async function (globalState: vscode.Memento, queryResultsWebviewMapping: Map<string, ResultsRender>, uuid: string, mainLabel: string, queryText: string): Promise<number> {

// 	const label = `Visualization: ${mainLabel} | ${uuid}`;

// 	let resultsChartRender = QueryResultsMappingService.getQueryResultsMappingResultsChartRender(queryResultsWebviewMapping, uuid);

// 	if (resultsChartRender) {

// 		resultsChartRender.reveal(undefined, true);

// 	} else {

// 		const panel = vscode.window.createWebviewPanel(CHART_VIEW_TYPE, label, { viewColumn: vscode.ViewColumn.Two, preserveFocus: true }, { enableFindWidget: true, enableScripts: true });
// 		resultsChartRender = new ResultsChartRender(panel);

// 		QueryResultsMappingService.updateQueryResultsChartMappingWebviewPanel(queryResultsWebviewMapping, uuid, resultsChartRender);

// 		//action when panel is closed
// 		panel.onDidDispose(e => {
// 			QueryResultsMappingService.deleteQueryResultsMapping(globalState, uuid);
// 		});

// 	}

// 	try {

// 		resultsChartRender.renderLoadingIcon();

// 		const queryResponse = getBigQueryClient().runQuery(queryText);
// 		const jobReferences = (await queryResponse).map(c => { return { jobId: c.id, projectId: c.projectId, location: c.location } as JobReference; });

// 		const request = {
// 			jobReferences: jobReferences,
// 			jobIndex: 0,
// 		} as ResultsChartRenderRequest;

// 		resultsChartRender.render(request);

// 		QueryResultsMappingService.updateQueryResultsMapping(globalState, uuid, request);

// 		return (await queryResponse).length;
// 	} catch (error) {
// 		resultsChartRender.renderException(error);
// 	}

// 	return 0;
// };

let bigQueryClient: BigQueryClient | null;

export const getBigQueryClient = async function (): Promise<BigQueryClient> {
	if (!bigQueryClient) {
		const t1 = Date.now();
		const projectId = await Authentication.getDefaultProjectId();
		bigQueryClient = new BigQueryClient(projectId);
		// getTelemetryReporter()?.sendTelemetryEvent('CreateBigQueryClient', {}, { elapsedMs: Date.now() - t1 });
	}

	return bigQueryClient;
};

const resetBigQueryClient = function () {
	bigQueryClient = null;
};
