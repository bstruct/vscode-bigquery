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
import { BqsqlInlineCompletionItemProvider } from './language/bqsqlInlineCompletionItemProvider';

export const bigqueryWebviewViewProvider = new WebviewViewProvider();
export const authenticationWebviewProvider = new BigqueryAuthenticationWebviewViewProvider();
export const bigQueryTreeDataProvider = new BigQueryTreeDataProvider();
export let extensionUri: vscode.Uri;
export let bigqueryIcons: BigqueryIcons;
export let reporter: TelemetryReporter | null;

export function activate(context: vscode.ExtensionContext) {

	extensionUri = context.extensionUri;

	bigqueryIcons = new BigqueryIcons();

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

	//language
	context.subscriptions.push(
		vscode.languages.registerCompletionItemProvider(
			{ language: 'bqsql' },
			new BqsqlCompletionItemProvider()
		)
	);

	context.subscriptions.push(
		vscode.languages.registerInlineCompletionItemProvider(
			{ language: 'bqsql' },
			new BqsqlInlineCompletionItemProvider()
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

	// vscode.env.onDidChangeTelemetryEnabled

	// vscode.env.isTelemetryEnabled

}

// this method is called when your extension is deactivated
export function deactivate() { }
