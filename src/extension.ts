import * as vscode from 'vscode';
import { BigqueryAuthenticationWebviewViewProvider } from './activitybar/authentication-webview-view-provider';
import { BigQueryTreeDataProvider } from './activitybar/tree-data-provider';
import { BigqueryIcons } from './bigquery-icons';
import * as commands from './extension-commands';
import { WebviewViewProvider } from './table_results_panel/webview-view-provider';

export const bigqueryWebviewViewProvider = new WebviewViewProvider();
export const authenticationWebviewProvider = new BigqueryAuthenticationWebviewViewProvider();
export const bigQueryTreeDataProvider = new BigQueryTreeDataProvider();
export let extensionUri: vscode.Uri;
export let bigqueryIcons : BigqueryIcons;

export function activate(context: vscode.ExtensionContext) {

	extensionUri = context.extensionUri;

	bigqueryIcons = new BigqueryIcons();

	vscode.workspace.onDidChangeConfiguration(event => {
		if (event.affectsConfiguration('workbench.colorTheme')) {
			vscode.commands.executeCommand(commands.COMMAND_EXPLORER_REFRESH);
		}
	});

	context.subscriptions.push(
		vscode.commands.registerCommand(
			commands.COMMAND_RUN_QUERY,
			commands.commandRunQuery
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

}

// this method is called when your extension is deactivated
export function deactivate() { }
