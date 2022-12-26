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

export const bigqueryWebviewViewProvider = new WebviewViewProvider();
export const authenticationWebviewProvider = new BigqueryAuthenticationWebviewViewProvider();
export const bigQueryTreeDataProvider = new BigQueryTreeDataProvider();
export const bigqueryTableSchemaService = new BigqueryTableSchemaService();
export let extensionUri: vscode.Uri;
export let bigqueryIcons: BigqueryIcons;
export let reporter: TelemetryReporter | null;
export let statusBarInfo: vscode.StatusBarItem | null;
export let queryResultsWebviewMapping: Map<string, vscode.WebviewPanel> = new Map<string, vscode.WebviewPanel>();

export function activate(context: vscode.ExtensionContext) {

	extensionUri = context.extensionUri;

	bigqueryIcons = new BigqueryIcons();

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
			{ "globalState": context.globalState }
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_RUN_SELECTED_QUERY,
			commands.commandRunSelectedQuery,
			{ "globalState": context.globalState }
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

	//bigquery-query-results
	context.subscriptions.push(
		vscode.window.registerWebviewPanelSerializer(
			"bigquery-query-results",
			new QueryResultsSerializer(context.globalState)
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

			const uuid = QueryResultsMappingService.getQueryResultsMappingUuid(context.globalState, e);
			if (uuid) {
				const panel = QueryResultsMappingService.getQueryResultsMappingWebviewPanel(uuid);
				if (panel) {
					panel.reveal(undefined, true);
				}
			}
		}

	});

	// vscode.env.onDidChangeTelemetryEnabled

	// vscode.env.isTelemetryEnabled

}

// this method is called when your extension is deactivated
export function deactivate() { }
