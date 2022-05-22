import bigquery from '@google-cloud/bigquery/build/src/types';
import * as vscode from 'vscode';
import { BigQueryQueryRunner } from './bigquery-query-runner';
import { ResultsGridRender } from './table_results_panel/results_grid_render';
// import { BigQueryQueryRunner } from './bigquery-query-runner';
// import { BigQueryWebviewViewProvider } from './bigquery-webview-view-provider';
// import { bigQueryWebviewViewProvider } from './extension';

export const command_runQuery = async function (...args: any[]) {

	if (vscode.window.activeTextEditor === undefined) {
		return;
	}

	const textEditor = vscode.window.activeTextEditor;

	const bqRunner = new BigQueryQueryRunner();

	const queryText: string = textEditor.document.getText() ?? '';

	const queryResponse = bqRunner.runQuery(queryText);

	//display result in panel to show the query result rows
	const panel = vscode.window.createWebviewPanel("xxx", "xxx", vscode.ViewColumn.Beside,
		{
			enableScripts: true,
			enableFindWidget: true,
		});

	const _ = new ResultsGridRender().render(panel.webview, queryResponse);

	// bigQueryWebviewViewProvider.setNewJob(textEditor, job);


	// //https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md


	// // const job: Promise<bigquery.IGetQueryResultsResponse> = bqRunner.runQuery(queryText);
	// const extensionUri = vscode.extensions.getExtension('bstruct.vscode-bigquery')?.extensionUri;
	// if (extensionUri === undefined) { return; }

	// // console.info(extensionUri);

	// function getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
	// 	return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
	// }


	// const toolkitUri = getUri(panel.webview, extensionUri, [
	// 	"node_modules",
	// 	"@vscode",
	// 	"webview-ui-toolkit",
	// 	"dist",
	// 	"toolkit.js", // A toolkit.min.js file is also available
	// ]);

	// panel.webview.html = `<!DOCTYPE html>
	// 	<html lang="en">
	// 		<head>
	// 			<meta charset="UTF-8">
	// 			<meta name="viewport" content="width=device-width, initial-scale=1.0">
	// 			<script type="module" src="${toolkitUri}"></script>
	// 			<title>test</title>
	// 		</head>
	// 		<body>
	// 			<h1>xxxx</h1>
	// 			<vscode-badge id="badge">3</vscode-badge>
	// 			<br />
	// 			<br />
	// 			<br />
	// 			<vscode-data-grid id="basic-grid" aria-label="Default"></vscode-data-grid>

	// 			<script>
	// 				document.getElementById('basic-grid').rowsData = [
	// 					{Header1: 'Cell Data', Header2: 'Cell Data', Header3: 'Cell Data', Header4: 'Cell Data'},
	// 					{Header1: 'Cell Data', Header2: 'Cell Data', Header3: 'Cell Data', Header4: 'Cell Data'},
	// 					{Header1: 'Cell Data', Header2: 'Cell Data', Header3: 'Cell Data', Header4: 'Cell Data'},
	// 				];

	// 			</script>

	// 		</body>
	// 	</html>`;



	// bigQueryWebviewViewProvider.setNewJob(textEditor, job);

	// vscode.window.showInformationMessage(`totalBytesProcessed ${job.statistics?.totalBytesProcessed}`);

}
