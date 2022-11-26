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

	test('COMMAND_SERVICE_ACCOUNT_LOGIN: Invalid file selected', async () => {

		const extension = vscode.extensions.getExtension('bstruct.vscode-bigquery');
		if (!extension) { assert.fail('extension not found'); }
		await extension.activate();

		//create dummy file to 
		const fileUri = vscode.Uri.joinPath(vscode.Uri.parse(__dirname), 'dummy.json');
		await vscode.workspace.fs.writeFile(fileUri, (new TextEncoder()).encode('dummy'));

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
		vscode.window.showErrorMessage = function<T extends string> (message: string, ...items: T[]): Thenable<any | undefined> {

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
		const fileUri = vscode.Uri.joinPath(vscode.Uri.parse(__dirname), 'credentials.json');

		//valid service account with no permissions
		const sa = {
			"type": "service_account",
			// eslint-disable-next-line @typescript-eslint/naming-convention
			"project_id": "damiao-project-1",
			// eslint-disable-next-line @typescript-eslint/naming-convention
			"private_key_id": "ced103010fae8ef1da93b1c8bd4cda55333a83a6",
			// eslint-disable-next-line @typescript-eslint/naming-convention
			"private_key": "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQC4V8Lg0tBQhulQ\ngM6bRfZ4hpZxs5G+WulMmbcNjfBAL1NGZMYGPlCJOrKsmOV34A53Y/zWtMfmNlEu\n6T/SMuIQ4cP2rZ522eXkOxHmTXpuqGyYb6xQ3T9TZqVdt4i7u70o/UqzKF2nYY5z\nVniKIsxLuPkPntAUqLmnbWLCAWdj37GExKuMIGf5kGhle1iepT0ECCGbmWJexj15\nIiPEXVe2DJhUaEkOMROo7sUpUSbjFpc/nQFG1Shy9aYq6Mjer2jiVbpjsYCEqQil\nosgjrr+eTD7lBxpIlB02lSRf//fuUtLh0jgC13DXr3v54B/rBnMzqgO/lZgNOToG\n0sQlDJ/NAgMBAAECggEAIB+nBZ+nDolcChvQJS3NdBZcDCdTK/1Sr3cP96w25DB7\nDgbMnVTpmuhgL4SbZEbmrnZ9nnq1ZRAtGIQsC3izfWaiTA/YT/TLw3hpt5zjy3nN\nJsk7GYJcoS9/fLPZf4GJRqXRCSRtIZh2BvJYIhZLTJzIxiiSMS9v3tXIgm9VMYmD\nc9MHJQqIaTxgDMPe3+AjqtRerfOlIYrUHaWbH/JfI7GRmwn/QrGilMS32Z3ewYYn\nraz9JnWj+sxEWZQuND0ZNCoZc3CNnksHDtA5dMFsWOTgrpyGvO5ItYuI3GvvWtHC\nT+Fq3ORbS8G2P0GrsQJZgPaHPXABlA2OC2+op6oULwKBgQDh+X2WbJhStSAVzzKk\npWl/wIo8ujyKCy3X52YTMbybejR4fHlZy3gwRBkW8v6oM2orveWF/ld6RUrrga4V\nkYLKpcD41CQQkF+rfV4WBtSW8aEJtLn1nsONAcGIUPw9nxNM0zDec2sNj9RMS+G4\ndlD7SjFEL6h3cYyIHYUn2N0RBwKBgQDQ1itdtD7URx9LT1nTWaedUmADeIFeqRFZ\nWmso8AYE8CQsslE4qDCYBddE9l1s6bAL+Wb9f3WRNRHfTZ14K5mMlAp1g6COhYyR\n5OQn62fTGyx9RzpicTyVAdYZkNzi2qZO+nNs8WjGT7C7B0RDpH+dNHaO5fEHAgW0\nv0TYCi1XiwKBgB3bS/ebA4kx+zpGdQeB/21ssBcT+Dm4/mafYUI5+RSF0fb1Y0c0\n9f1SkgoRMwpgOK/s2C2bDE3QZ/Sz3p9k6WYC/nsh9F3n5WwQFWVNo7sJ0+Ana8aD\nIo035S0wnhM6OF+XK8bIcyWIkmE+SWWI5Gw/QkEjFtwpOYsmc1hvc83zAoGAJGCq\nPTBgoWmiMeQoqYA8ilMHFpOvNWYN95qggAkPg1yxcHe/Xjct/81Eqmaf75DlcbCI\nGDBTsm+kJVnHzF8L6EkBaWb8WNc6nU9Zzvpao5NgNJJrwSooe2xOdzWcxIeB4NAP\nuzJyJdlL18h0Q7Qr5p506H91dNsNU2bd/yQGsfECgYEAramGk5GTOsBlgNhhV6/4\nYsb7JIAz3QgKQWtqMErNNOLRoa9TLk2DpsIsRdgVIrkfKQH8YkxbooWzFvJdxALj\nXY/k41QJfDygpUFm6wptvzMHgc2/T39Z3lZmaWUpfiHgFfnCzkvt2CEY1kjdQSSs\nGvOUBLrbgT1i3ChWgz/O7VE=\n-----END PRIVATE KEY-----\n",
			// eslint-disable-next-line @typescript-eslint/naming-convention
			"client_email": "dummy-sa@damiao-project-1.iam.gserviceaccount.com",
			// eslint-disable-next-line @typescript-eslint/naming-convention
			"client_id": "113597072127791794001",
			// eslint-disable-next-line @typescript-eslint/naming-convention
			"auth_uri": "https://accounts.google.com/o/oauth2/auth",
			// eslint-disable-next-line @typescript-eslint/naming-convention
			"token_uri": "https://oauth2.googleapis.com/token",
			// eslint-disable-next-line @typescript-eslint/naming-convention
			"auth_provider_x509_cert_url": "https://www.googleapis.com/oauth2/v1/certs",
			// eslint-disable-next-line @typescript-eslint/naming-convention
			"client_x509_cert_url": "https://www.googleapis.com/robot/v1/metadata/x509/dummy-sa%40damiao-project-1.iam.gserviceaccount.com"
		};

		await vscode.workspace.fs.writeFile(fileUri, (new TextEncoder()).encode(JSON.stringify(sa)));

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

});
