import * as assert from 'assert';

// You can import and use all API from the 'vscode' module
// as well as import your extension to test it
import * as vscode from 'vscode';
import { COMMAND_SERVICE_ACCOUNT_LOGIN } from '../../extensionCommands';

suite('Extension Test Suite', async () => {
	vscode.window.showInformationMessage('Start all tests.');

	test('COMMAND_SERVICE_ACCOUNT_LOGIN: No file selected', async () => {

		const extension = vscode.extensions.getExtension('bstruct.vscode-bigquery');
		if (!extension) { assert.fail('extension not found'); }
		await extension.activate();
	
		let showOpenDialogCount = 0;
		vscode.window.showOpenDialog = function (options?: vscode.OpenDialogOptions): Thenable<vscode.Uri[] | undefined> {

			showOpenDialogCount++;

			assert.equal(true, options?.canSelectFiles);
			assert.equal(false, options?.canSelectFolders);
			assert.equal(false, options?.canSelectMany);

			return new Promise((resolve, reject) => { resolve(undefined); });
		};

		const commandResult = await vscode.commands.executeCommand(COMMAND_SERVICE_ACCOUNT_LOGIN);

		assert.equal(1, showOpenDialogCount);

	});

	// test('Sample test', async () => {

	// 	// const extension = vscode.extensions.getExtension('bstruct.vscode-bigquery');
	// 	// const panel = extension?.exports?.bigqueryWebviewViewProvider?.webviewView;
	// 	// debugger;

	// 	// const doc = await vscode.workspace.openTextDocument({
	// 	// 	language: 'bqsql',
	// 	// 	content: 'SELECT 1,2,3'
	// 	// });

	// 	// await vscode.commands.executeCommand<vscode.TextDocumentShowOptions>("vscode.open", doc.uri);

	// 	// await vscode.commands.executeCommand("vscode-bigquery.run-query", doc.uri);

	// });

});
