import * as vscode from 'vscode';
import { BigqueryAuthenticationWebviewViewProvider } from './activitybar/authentication-webview-view-provider';
import { BigQueryTreeDataProvider } from './activitybar/tree-data-provider';
import * as commands from './extension-commands';
import { WebviewViewProvider } from './table_results_panel/webview-view-provider';

export const bigqueryWebviewViewProvider = new WebviewViewProvider();
export let extensionUri: vscode.Uri;

export function activate(context: vscode.ExtensionContext) {

	extensionUri = context.extensionUri;

	//vscode-bigquery.run-query
	context.subscriptions.push(
		vscode.commands.registerCommand(
			'vscode-bigquery.run-query',
			commands.command_runQuery
		)
	);

	// bigquery-authentication
	context.subscriptions.push(
		vscode.window.registerWebviewViewProvider(
			"bigquery-authentication",
			new BigqueryAuthenticationWebviewViewProvider(),
			{ webviewOptions: { retainContextWhenHidden: true } }
		)
	);

	//bigquery-tree-data-provider
	context.subscriptions.push(
		vscode.window.registerTreeDataProvider(
			'bigquery-tree-data-provider',
			new BigQueryTreeDataProvider()
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
