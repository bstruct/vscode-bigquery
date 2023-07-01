import * as assert from 'assert';
import * as vscode from 'vscode';
import { COMMAND_DOWNLOAD_CSV, COMMAND_DOWNLOAD_JSONL, COMMAND_RUN_QUERY, COMMAND_SERVICE_ACCOUNT_LOGIN } from '../../extensionCommands';
import { LocalMemento } from './localMemento';

suite('Extension Test Suite', async () => {
	vscode.window.showInformationMessage('Start all tests.');

	let globalState: LocalMemento = new LocalMemento();
	let queryResultsWebviewMapping: Map<string, vscode.WebviewPanel> = new Map<string, vscode.WebviewPanel>();

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
		// const fileContent = await vscode.workspace.fs.readFile(fileUri);
		// const fileContentString = new TextDecoder().decode(fileContent);
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

		let commandInput = {
			"globalState": globalState,
			queryResultsWebviewMapping: queryResultsWebviewMapping
		};

		await vscode.commands.executeCommand(COMMAND_RUN_QUERY, commandInput);

		//there is a second group tab
		const secondGroupTab = vscode.window.tabGroups.all.find(c => c.viewColumn === vscode.ViewColumn.Two);
		assert.ok(secondGroupTab !== null && secondGroupTab !== undefined);

		//
		if (secondGroupTab !== null && secondGroupTab !== undefined) {
			assert.ok(secondGroupTab.tabs.length > 0);
			assert.equal(secondGroupTab.tabs.length,
				secondGroupTab
					.tabs
					.filter(c => ((c.input as any).viewType as string)?.endsWith("-bigquery-query-results")).length);
		}

	});

	test('COMMAND_RUN_QUERY: SELECT 1,2,3 - DOWNLOAD CSV', async (...args: any[]) => {

		//there is a second group tab
		const secondGroupTab = vscode.window.tabGroups.all.find(c => c.viewColumn === vscode.ViewColumn.Two);
		assert.ok(secondGroupTab !== null && secondGroupTab !== undefined);

		//
		if (secondGroupTab !== null && secondGroupTab !== undefined) {
			assert.ok(secondGroupTab.tabs.length > 0);

			const path = process.env.GITHUB_WORKSPACE || __dirname;
			const downloadFileUri = vscode.Uri.joinPath(vscode.Uri.file(path), 'download1.csv');

			let showOpenDialogCount = 0;
			vscode.window.showSaveDialog = function (options?: vscode.SaveDialogOptions): Thenable<vscode.Uri | undefined> {

				if (options !== undefined) {
					showOpenDialogCount++;

					assert.equal('Save export', options?.title);
					assert.ok(options?.filters?.csv);
					assert.ok(options?.filters?.csv.length);
					assert.equal('csv', options?.filters?.csv[0]);

					return new Promise((resolve, reject) => { resolve(downloadFileUri); });
				} else {
					return new Promise((resolve, reject) => { reject('options not defined'); });
				}
			};

			let showInformationMessageCount = 0;
			vscode.window.showInformationMessage = function (message: string, ...items: any[]): Thenable<any | undefined> {

				showInformationMessageCount++;

				return new Promise((resolve, reject) => { resolve(undefined); });
			};

			await vscode.commands.executeCommand('workbench.action.focusNextGroup');
			await vscode.commands.executeCommand(COMMAND_DOWNLOAD_CSV);

			assert.equal(1, showOpenDialogCount);
			if (downloadFileUri !== undefined) {
				const fileContent = await vscode.workspace.fs.readFile(downloadFileUri);
				const fileContentString = new TextDecoder().decode(fileContent);
				assert('f0_,f1_,f2_\n1,2,3', fileContentString);

			} else {
				assert.fail('unexpected');
			}

			assert.equal(2, showInformationMessageCount);

		} else {
			assert.fail('mapping not found');
		}

	});

	test('COMMAND_RUN_QUERY: INSERT', async () => {

		// await vscode.commands.executeCommand('workbench.action.closeAllEditors');
		// await vscode.commands.executeCommand('workbench.action.closeAllGroups');

		const doc = await vscode.workspace.openTextDocument({
			language: 'bqsql',
			content: 'INSERT INTO Business.dataflow_test SELECT CURRENT_TIMESTAMP(), "NAME" as NAME, "body"'
		});

		await vscode.commands.executeCommand<vscode.TextDocumentShowOptions>("vscode.open", doc.uri);

		let commandInput = {
			"globalState": globalState,
			queryResultsWebviewMapping: queryResultsWebviewMapping
		};

		await vscode.commands.executeCommand(COMMAND_RUN_QUERY, commandInput);

		//there is a second group tab
		const secondGroupTab = vscode.window.tabGroups.all.find(c => c.viewColumn === vscode.ViewColumn.Two);
		assert.ok(secondGroupTab !== null && secondGroupTab !== undefined);

		//
		if (secondGroupTab !== null && secondGroupTab !== undefined) {
			assert.ok(secondGroupTab.tabs.length > 0);
			assert.equal(secondGroupTab.tabs.length,
				secondGroupTab
					.tabs
					.filter(c => ((c.input as any).viewType as string)?.endsWith("-bigquery-query-results")).length);
		}

	});

	test('COMMAND_RUN_QUERY: INSERT - DOWNLOAD CSV', async (...args: any[]) => {

		//there is a second group tab
		const secondGroupTab = vscode.window.tabGroups.all.find(c => c.viewColumn === vscode.ViewColumn.Two);
		assert.ok(secondGroupTab !== null && secondGroupTab !== undefined);

		//
		if (secondGroupTab !== null && secondGroupTab !== undefined) {
			assert.ok(secondGroupTab.tabs.length > 0);

			const path = process.env.GITHUB_WORKSPACE || __dirname;
			const downloadFileUri = vscode.Uri.joinPath(vscode.Uri.file(path), 'download2.csv');

			let showOpenDialogCount = 0;
			vscode.window.showSaveDialog = function (options?: vscode.SaveDialogOptions): Thenable<vscode.Uri | undefined> {

				if (options !== undefined) {
					showOpenDialogCount++;

					assert.equal('Save export', options?.title);
					assert.ok(options?.filters?.csv);
					assert.ok(options?.filters?.csv.length);
					assert.equal('csv', options?.filters?.csv[0]);

					return new Promise((resolve, reject) => { resolve(downloadFileUri); });
				} else {
					return new Promise((resolve, reject) => { reject('options not defined'); });
				}
			};

			let showInformationMessageCount = 0;
			vscode.window.showInformationMessage = function (message: string, ...items: any[]): Thenable<any | undefined> {

				showInformationMessageCount++;

				return new Promise((resolve, reject) => { resolve(undefined); });
			};

			await vscode.commands.executeCommand('workbench.action.focusNextGroup');
			assert.ok(
				secondGroupTab.tabs
					.find(t => t.label.startsWith('Visualization: INSERT'))
					?.isActive
			);
			await vscode.commands.executeCommand(COMMAND_DOWNLOAD_CSV);

			assert.equal(1, showOpenDialogCount);
			if (downloadFileUri !== undefined) {
				const fileContent = await vscode.workspace.fs.readFile(downloadFileUri);
				const fileContentString = new TextDecoder().decode(fileContent);
				assert('insertedRowCount,updatedRowCount,deletedRowCount\n1,,', fileContentString);

			} else {
				assert.fail('unexpected');
			}

			assert.equal(2, showInformationMessageCount);

		} else {
			assert.fail('mapping not found');
		}

	});

	test('COMMAND_RUN_QUERY: DELETE', async () => {

		// await vscode.commands.executeCommand('workbench.action.closeAllEditors');
		// await vscode.commands.executeCommand('workbench.action.closeAllGroups');

		const doc = await vscode.workspace.openTextDocument({
			language: 'bqsql',
			content: 'DELETE Business.dataflow_test WHERE timestamp <= CURRENT_TIMESTAMP()'
		});

		await vscode.commands.executeCommand<vscode.TextDocumentShowOptions>("vscode.open", doc.uri);

		let commandInput = {
			"globalState": globalState,
			queryResultsWebviewMapping: queryResultsWebviewMapping
		};

		await vscode.commands.executeCommand(COMMAND_RUN_QUERY, commandInput);

		//there is a second group tab
		const secondGroupTab = vscode.window.tabGroups.all.find(c => c.viewColumn === vscode.ViewColumn.Two);
		assert.ok(secondGroupTab !== null && secondGroupTab !== undefined);

		//
		if (secondGroupTab !== null && secondGroupTab !== undefined) {
			assert.ok(secondGroupTab.tabs.length > 0);
			assert.equal(secondGroupTab.tabs.length,
				secondGroupTab
					.tabs
					.filter(c => ((c.input as any).viewType as string)?.endsWith("-bigquery-query-results")).length);
		}

	});

	test('COMMAND_RUN_QUERY: DELETE - DOWNLOAD JSONL', async (...args: any[]) => {

		//there is a second group tab
		const secondGroupTab = vscode.window.tabGroups.all.find(c => c.viewColumn === vscode.ViewColumn.Two);
		assert.ok(secondGroupTab !== null && secondGroupTab !== undefined);

		//
		if (secondGroupTab !== null && secondGroupTab !== undefined) {
			assert.ok(secondGroupTab.tabs.length > 0);

			const path = process.env.GITHUB_WORKSPACE || __dirname;
			const downloadFileUri = vscode.Uri.joinPath(vscode.Uri.file(path), 'download1.jsonl');

			let showOpenDialogCount = 0;
			vscode.window.showSaveDialog = function (options?: vscode.SaveDialogOptions): Thenable<vscode.Uri | undefined> {

				if (options !== undefined) {
					showOpenDialogCount++;

					assert.equal('Save export', options?.title);
					assert.ok(options?.filters?.jsonl);
					assert.ok(options?.filters?.jsonl.length);
					assert.equal('jsonl', options?.filters?.jsonl[0]);

					return new Promise((resolve, reject) => { resolve(downloadFileUri); });
				} else {
					return new Promise((resolve, reject) => { reject('options not defined'); });
				}
			};

			// let showInformationMessageCount = 0;
			// vscode.window.showInformationMessage = function (message: string, ...items: any[]): Thenable<any | undefined> {

			// 	showInformationMessageCount++;

			// 	return new Promise((resolve, reject) => { resolve(undefined); });
			// };

			await vscode.commands.executeCommand('workbench.action.focusNextGroup');
			assert.ok(
				secondGroupTab.tabs
					.find(t => t.label.startsWith('Visualization: DELETE'))
					?.isActive
			);
			await vscode.commands.executeCommand(COMMAND_DOWNLOAD_JSONL);

			// assert.equal(1, showOpenDialogCount);
			if (downloadFileUri !== undefined) {
				const fileContent = await vscode.workspace.fs.readFile(downloadFileUri);
				const fileContentString = new TextDecoder().decode(fileContent);
				assert('{"insertedRowCount":null,"updatedRowCount":null,"deletedRowCount":1}', fileContentString);

			} else {
				assert.fail('unexpected');
			}

			// assert.equal(2, showInformationMessageCount);

		} else {
			assert.fail('mapping not found');
		}

	});

});
