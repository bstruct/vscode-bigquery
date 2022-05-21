import * as vscode from 'vscode';
import { BigQueryTreeDataProvider } from './bigquery-tree-data-provider';
import { BigQueryWebviewPanelSerializer } from './bigquery-webview-panel-serializer';
import { BigQueryWebviewViewProvider } from './bigquery-webview-view-provider';
import * as commands from './extension-commands';

export const bigQueryWebviewViewProvider = new BigQueryWebviewViewProvider();

export function activate(context: vscode.ExtensionContext) {

	context.subscriptions.push(
		vscode.commands.registerCommand(
			'vscode-bigquery.run-query',
			commands.command_runQuery
		)
	);

	context.subscriptions.push(
		vscode.window.registerTreeDataProvider(
			'bigquery-tree-data-provider',
			new BigQueryTreeDataProvider()
		)
	);

	context.subscriptions.push(
		vscode.window.registerWebviewViewProvider(
			"vscode-bigquery-query-results-1",
			bigQueryWebviewViewProvider,
			{ webviewOptions: { retainContextWhenHidden: true } }
		)
	);

	// console.info(context.extensionUri);

	context.subscriptions.push(
		vscode.window.registerWebviewPanelSerializer(
			"xxx",
			new BigQueryWebviewPanelSerializer()
		)
	);

}

// this method is called when your extension is deactivated
export function deactivate() { }
