import * as vscode from 'vscode';
import { BigqueryAuthenticationWebviewViewProvider } from './activitybar/authenticationWebviewViewProvider';
import { BigQueryTreeDataProvider } from './activitybar/treeDataProvider';
import { BigqueryIcons } from './bigqueryIcons';
import * as commands from './extensionCommands';
import { WebviewViewProvider } from './tableResultsPanel/webviewViewProvider';
import TelemetryReporter from '@vscode/extension-telemetry';
import { BqsqlCompletionItemProvider } from './language/bqsqlCompletionItemProvider';
import { BqsqlDocumentSemanticTokensProvider } from './language/bqsqlDocumentSemanticTokensProvider';
import { BqsqlInlayHintsProvider } from './language/bqsqlInlayHintsProvider';
import { BigqueryTableSchemaService } from './services/bigqueryTableSchemaService';
import { BqsqlDiagnostics } from './language/bqsqlDiagnostics';
import { QueryResultsSerializer } from './tableResultsPanel/queryResultsSerializer';
import { QueryResultsMappingService } from './services/queryResultsMappingService';
import { TableResultsSerializer } from './tableResultsPanel/tableResultsSerializer';
import { ResultsRender } from './services/resultsRender';
import { ChartResultsSerializer } from './charts/chartResultsSerializer';
import { QueryResultsVisualizationType } from './services/queryResultsVisualizationType';
import { TroubleshootSerializer } from './activitybar/troubleshootSerializer';

export const bigqueryWebviewViewProvider = new WebviewViewProvider();
export const authenticationWebviewProvider = new BigqueryAuthenticationWebviewViewProvider();
export const bigQueryTreeDataProvider = new BigQueryTreeDataProvider();
export const bigqueryTableSchemaService = new BigqueryTableSchemaService();
export let extensionUri: vscode.Uri;
export let bigqueryIcons: BigqueryIcons;
export let reporter: TelemetryReporter | null;
export let statusBarInfo: vscode.StatusBarItem | null;

export const CHART_VIEW_TYPE = "bigquery-query-chart";
export const QUERY_RESULTS_VIEW_TYPE = "bigquery-query-results";
export const TABLE_RESULTS_VIEW_TYPE = "bigquery-table-results";
export const TROUBLESHOOT_VIEW_TYPE = "authentication-troubleshoot";

export function activate(context: vscode.ExtensionContext) {

	extensionUri = context.extensionUri;

	bigqueryIcons = new BigqueryIcons();

	let queryResultsWebviewMapping: Map<string, ResultsRender> = new Map<string, ResultsRender>();

	try {

		reporter = new TelemetryReporter(context.extension.id, context.extension.packageJSON.version, '10f4da7d-e729-4526-8d9b-92529b10cb32');
		context.subscriptions.push(reporter);

	} catch (e) { console.error(e); }

	//statusBarInfo
	statusBarInfo = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 0);
	context.subscriptions.push(statusBarInfo);


	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_RUN_QUERY,
			commands.commandRunQuery,
			{
				"globalState": context.globalState,
				queryResultsWebviewMapping: queryResultsWebviewMapping
			}
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_RUN_SELECTED_QUERY,
			commands.commandRunSelectedQuery,
			{
				"globalState": context.globalState,
				queryResultsWebviewMapping: queryResultsWebviewMapping
			}
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_USER_LOGIN,
			commands.commandUserLogin
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_USER_LOGIN_WITH_DRIVE,
			commands.commandUserLoginWithDrive
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_SERVICE_ACCOUNT_LOGIN,
			commands.commandServiceAccountLogin
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_GCLOUD_INIT,
			commands.commandGCloudInit
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_VIEW_TABLE,
			commands.commandViewTable
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_VIEW_TABLE_SCHEMA,
			commands.commandViewTableSchema
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_CREATE_TABLE_DEFAULT_QUERY,
			commands.commandCreateTableDefaultQuery
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_OPEN_DDL,
			commands.commandOpenDdl
		)
	);

	//https://code.visualstudio.com/api/references/when-clause-contexts
	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_AUTHENTICATION_REFRESH,
			commands.commandAuthenticationRefresh
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_EXPLORER_REFRESH,
			commands.commandExplorerRefresh
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_SET_DEFAULT_PROJECT,
			commands.commandSetDefaultProject
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_DOWNLOAD_CSV,
			commands.commandDownloadCsv,
			{ "globalState": context.globalState }
		),
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_PROJECT_PIN,
			commands.commandPinOrUnpinProject
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_PLOT_CHART,
			commands.commandPlotChart,
			{
				"globalState": context.globalState,
				queryResultsWebviewMapping: queryResultsWebviewMapping
			}
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.AUTHENTICATION_TROUBLESHOOT,
			commands.commandAuthenticationTroubleshoot
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.OPEN_SETTING_PROJECTS,
			commands.commandOpenSettingProjects
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.OPEN_SETTING_TABLES,
			commands.commandOpenSettingTables
		)
	);

	// bigquery-authentication
	context.subscriptions.push(
		vscode.window.registerWebviewViewProvider(
			"bigquery-authentication",
			authenticationWebviewProvider,
			{ webviewOptions: { retainContextWhenHidden: true } }
		)
	);

	//bigquery-tree-data-provider
	context.subscriptions.push(
		vscode.window.registerTreeDataProvider(
			'bigquery-tree-data-provider',
			bigQueryTreeDataProvider
		)
	);

	//bigquery-query-chart
	context.subscriptions.push(
		vscode.window.registerWebviewPanelSerializer(
			CHART_VIEW_TYPE,
			new ChartResultsSerializer(context.globalState, queryResultsWebviewMapping)
		)
	);

	//bigquery-query-results
	context.subscriptions.push(
		vscode.window.registerWebviewPanelSerializer(
			QUERY_RESULTS_VIEW_TYPE,
			new QueryResultsSerializer(context.globalState, queryResultsWebviewMapping)
		)
	);

	//bigquery-table-results
	context.subscriptions.push(
		vscode.window.registerWebviewPanelSerializer(
			TABLE_RESULTS_VIEW_TYPE,
			new TableResultsSerializer()
		)
	);

	//troubleshoot
	context.subscriptions.push(
		vscode.window.registerWebviewPanelSerializer(
			TROUBLESHOOT_VIEW_TYPE,
			new TroubleshootSerializer()
		)
	);

	//language
	const baseDiagnostics = vscode.languages.createDiagnosticCollection('base_diagnostics');
	context.subscriptions.push(baseDiagnostics);
	BqsqlDiagnostics.subscribeToDocumentChanges(context, baseDiagnostics);

	context.subscriptions.push(
		vscode.languages.registerCompletionItemProvider(
			{ language: 'bqsql' },
			new BqsqlCompletionItemProvider()
		)
	);

	context.subscriptions.push(
		vscode.languages.registerDocumentSemanticTokensProvider(
			{ language: 'bqsql' },
			new BqsqlDocumentSemanticTokensProvider(),
			BqsqlDocumentSemanticTokensProvider.getSemanticTokensLegend()
		)
	);

	context.subscriptions.push(
		vscode.languages.registerInlayHintsProvider(
			{ language: 'bqsql' },
			new BqsqlInlayHintsProvider()
		)
	);

	//later
	// context.subscriptions.push(
	// 	vscode.languages.registerHoverProvider(
	// 		{ language: 'bqsql' },
	// 		new BqsqlHoverProvider()
	// 	)
	// );

	//check if the theme has changed and the tree icons need to change colour
	vscode.workspace.onDidChangeConfiguration(event => {
		if (event.affectsConfiguration('workbench.colorTheme')) {
			vscode.commands.executeCommand(commands.COMMAND_EXPLORER_REFRESH);
			reporter?.sendTelemetryEvent('onDidChangeActiveColorTheme', { activeColorThemeKind: vscode.ColorThemeKind[vscode.window.activeColorTheme.kind] });
		}
	});

	vscode.window.onDidChangeActiveTextEditor(e => {

		if (e?.document.languageId === 'bqsql') {

			//check if results tab exist and it's known
			//  is possible that is not know in case that vscode was restarted and that window was not opened
			//  in this scenario, the tab exists but is not possible to determine the correspondent panel
			//  panels are lazy loaded

			[QueryResultsVisualizationType.chart, QueryResultsVisualizationType.table].forEach(t => {
				const uuid = QueryResultsMappingService.getQueryResultsMappingUuid(context.globalState, e, t);
				if (uuid) {
					const resultsGridRender = QueryResultsMappingService.getQueryResultsMappingResultsGridRender(queryResultsWebviewMapping, uuid);
					if (resultsGridRender) {
						resultsGridRender.reveal(undefined, true);
					}
				}
			});
		}

	});

	// vscode.env.onDidChangeTelemetryEnabled

	// vscode.env.isTelemetryEnabled

}

// this method is called when your extension is deactivated
export function deactivate() { }
