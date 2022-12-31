import * as assert from 'assert';
import * as vscode from 'vscode';
import { COMMAND_RUN_QUERY, COMMAND_SERVICE_ACCOUNT_LOGIN } from '../../extensionCommands';

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

	test('COMMAND_SERVICE_ACCOUNT_LOGIN: Invalid file selected', async () => {

		const extension = vscode.extensions.getExtension('bstruct.vscode-bigquery');
		if (!extension) { assert.fail('extension not found'); }
		await extension.activate();

		//create dummy file to 
		const path = process.env.GITHUB_WORKSPACE || __dirname;
		const fileUri = vscode.Uri.joinPath(vscode.Uri.file(path), 'dummy.json');

		let showOpenDialogCount = 0;
		vscode.window.showOpenDialog = function (options?: vscode.OpenDialogOptions): Thenable<vscode.Uri[] | undefined> {

			showOpenDialogCount++;

			assert.equal(true, options?.canSelectFiles);
			assert.equal(false, options?.canSelectFolders);
			assert.equal(false, options?.canSelectMany);

			return new Promise((resolve, reject) => { resolve([fileUri]); });
		};

		let showInformationMessageCount = 0;
		vscode.window.showInformationMessage = function (message: string, ...items: any[]): Thenable<any | undefined> {

			showInformationMessageCount++;

			return new Promise((resolve, reject) => { resolve(undefined); });
		};

		let showErrorMessageCount = 0;
		vscode.window.showErrorMessage = function <T extends string>(message: string, ...items: T[]): Thenable<any | undefined> {

			showErrorMessageCount++;

			assert.equal('Bigquery: Service account login - had invalid response', message);

			return new Promise((resolve, reject) => { resolve(undefined); });
		};
		try {
			const commandResult = await vscode.commands.executeCommand(COMMAND_SERVICE_ACCOUNT_LOGIN);

		} catch (error) { }

		assert.equal(1, showOpenDialogCount);
		assert.equal(1, showErrorMessageCount);
		assert.equal(0, showInformationMessageCount);

	});

	test('COMMAND_SERVICE_ACCOUNT_LOGIN: Valid file selected', async () => {

		const extension = vscode.extensions.getExtension('bstruct.vscode-bigquery');
		if (!extension) { assert.fail('extension not found'); }
		await extension.activate();

		//create dummy file to 
		const path = process.env.GITHUB_WORKSPACE || __dirname;
		const fileUri = vscode.Uri.joinPath(vscode.Uri.file(path), 'credentials.json');
		// if(!process.env.GITHUB_WORKSPACE){
		// 	await vscode.workspace.fs.writeFile(fileUri, (new TextEncoder()).encode('{test:1}'));
		// }

		let showOpenDialogCount = 0;
		vscode.window.showOpenDialog = function (options?: vscode.OpenDialogOptions): Thenable<vscode.Uri[] | undefined> {

			showOpenDialogCount++;

			assert.equal(true, options?.canSelectFiles);
			assert.equal(false, options?.canSelectFolders);
			assert.equal(false, options?.canSelectMany);

			return new Promise((resolve, reject) => { resolve([fileUri]); });
		};

		let showInformationMessageCount = 0;
		vscode.window.showInformationMessage = function (message: string, ...items: any[]): Thenable<any | undefined> {

			showInformationMessageCount++;

			assert.equal('Bigquery: Service account login - successful', message);

			return new Promise((resolve, reject) => { resolve(undefined); });
		};

		let showErrorMessageCount = 0;
		vscode.window.showErrorMessage = function (message: string, ...items: any[]): Thenable<any | undefined> {

			showErrorMessageCount++;

			return new Promise((resolve, reject) => { resolve(undefined); });
		};

		const commandResult = await vscode.commands.executeCommand(COMMAND_SERVICE_ACCOUNT_LOGIN);

		assert.equal(1, showOpenDialogCount);
		assert.equal(0, showErrorMessageCount);
		assert.equal(1, showInformationMessageCount);

	});

	test('COMMAND_RUN_QUERY: SELECT 1,2,3', async () => {

		const doc = await vscode.workspace.openTextDocument({
			language: 'bqsql',
			content: 'SELECT 1,2,3'
		});

		await vscode.commands.executeCommand<vscode.TextDocumentShowOptions>("vscode.open", doc.uri);

		await vscode.commands.executeCommand(COMMAND_RUN_QUERY);

		//there is a second group tab
		const secondGroupTab = vscode.window.tabGroups.all.find(c => c.viewColumn === vscode.ViewColumn.Two);
		assert.equal(true, secondGroupTab !== null && secondGroupTab !== undefined);

		//
		if (secondGroupTab !== null && secondGroupTab !== undefined) {
			assert.equal(true, secondGroupTab.tabs.length > 0);
			assert.equal(secondGroupTab.tabs.length, 
				secondGroupTab
				.tabs
				.filter(c => ((c.input as any).viewType as string).endsWith("-bigquery-query-results")).length);
		}

	});

});
