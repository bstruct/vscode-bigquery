import * as vscode from 'vscode';
import { BigqueryAuthenticationWebviewViewProvider } from './activitybar/authentication-webview-view-provider';
import { BigQueryTreeDataProvider } from './activitybar/tree-data-provider';
import { BigqueryIcons } from './bigquery-icons';
import * as commands from './extension-commands';
import { WebviewViewProvider } from './table_results_panel/webview-view-provider';
import TelemetryReporter from '@vscode/extension-telemetry';

export const bigqueryWebviewViewProvider = new WebviewViewProvider();
export const authenticationWebviewProvider = new BigqueryAuthenticationWebviewViewProvider();
export const bigQueryTreeDataProvider = new BigQueryTreeDataProvider();
export let extensionUri: vscode.Uri;
export let bigqueryIcons: BigqueryIcons;
export let reporter: TelemetryReporter | null;

export function activate(context: vscode.ExtensionContext) {

	extensionUri = context.extensionUri;

	bigqueryIcons = new BigqueryIcons();

	//vscode.env.appHost - desktop
	//vscode.env.appName - 'Visual Studio Code'
	//vscode.env.isNewAppInstall - False
	//vscode.env.isTelemetryEnabled - True
	//vscode.env.language - en
	//vscode.env.machineId - d2f43aff5df41dbeb13da3933848166e6baeae1de1a555c46ed5f044a23f53d5
	//vscode.env.sessionId - a3bdc4eb-a72f-492e-9255-994b55530cf91656861468776
	//vscode.env.uiKind - 1
	// const c = vscode.UIKind[vscode.env.uiKind];


	try {

		reporter = new TelemetryReporter(context.extension.id, context.extension.packageJSON.version, '10f4da7d-e729-4526-8d9b-92529b10cb32');
		context.subscriptions.push(reporter);

	} catch (e) { console.error(e); }

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_RUN_QUERY,
			commands.commandRunQuery
		)
	);

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_RUN_SELECTED_QUERY,
			commands.commandRunSelectedQuery
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
			commands.COMMAND_SERVICE_ACCOUNT_LOGIN,
			commands.commandServiceAccountLogin
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

	//vscode-bigquery-query-results-main
	context.subscriptions.push(
		vscode.window.registerWebviewViewProvider(
			"vscode-bigquery-query-results-main",
			bigqueryWebviewViewProvider
		)
	);

	vscode.workspace.onDidChangeConfiguration(event => {
		if (event.affectsConfiguration('workbench.colorTheme')) {
			vscode.commands.executeCommand(commands.COMMAND_EXPLORER_REFRESH);
			reporter?.sendTelemetryEvent('onDidChangeActiveColorTheme', { activeColorThemeKind: vscode.ColorThemeKind[vscode.window.activeColorTheme.kind] });
		}
	});

	// vscode.env.onDidChangeTelemetryEnabled

	// vscode.env.isTelemetryEnabled

}

// this method is called when your extension is deactivated
export function deactivate() { }
