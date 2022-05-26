import * as vscode from 'vscode';
import bigquery from '@google-cloud/bigquery/build/src/types';
import { extensionUri } from '../extension';
import { BigQueryDate, SimpleQueryRowsResponse } from '@google-cloud/bigquery';
import { SimpleQueryRowsResponseError } from '../bigquery/simple_query_rows_response_error';
import { Grid } from './grid';

//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class ResultsGridRender {

    public render(webView: vscode.Webview, queryResponse: Promise<SimpleQueryRowsResponse>) {

        const toolkitUri = this.getUri(webView, extensionUri, [
            "node_modules",
            "@vscode",
            "webview-ui-toolkit",
            "dist",
            "toolkit.min.js",
        ]);

        webView.html = this.getWaitingHtml(toolkitUri);

        queryResponse
            .then(result => {

                webView.html = this.getResultsHtml(toolkitUri, result);

            })
            .catch(exception => {

                webView.html = this.getExceptionHtml(toolkitUri, exception);

            });

    }

    private getWaitingHtml(toolkitUri: vscode.Uri): string {

        return `<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="UTF-8">
				<meta name="viewport" content="width=device-width, initial-scale=1.0">
				<script type="module" src="${toolkitUri}"></script>
			</head>
			<body>
                <vscode-progress-ring></vscode-progress-ring>
			</body>
		</html>`;

    }

    private getExceptionHtml(toolkitUri: vscode.Uri, exception: any): string {

        if (exception.errors) {

            const errors = (exception as SimpleQueryRowsResponseError).errors;

            const rows = JSON.stringify(errors.map(c => (
                {
                    "message": c.message,
                    "reason": c.reason,
                    "locationType": c.locationType
                }
            )));

            return `<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <script type="module" src="${toolkitUri}"></script>
                </head>
                <body>
                <vscode-data-grid id="basic-grid" generate-header="sticky" aria-label="Default"></vscode-data-grid>
    
                <script>
                    document.getElementById('basic-grid').rowsData = ${rows};
                </script>
                </body>
            </html>`;

        } else {

            const rows = JSON.stringify([{ message: exception.message, stack: exception.stack }]);

            return `<!DOCTYPE html>
            <html lang="en">
                <head>
                    <meta charset="UTF-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1.0">
                    <script type="module" src="${toolkitUri}"></script>
                </head>
                <body>
                <vscode-data-grid id="basic-grid" generate-header="sticky" aria-label="Default"></vscode-data-grid>
    
                <script>
                    document.getElementById('basic-grid').rowsData = ${rows};
                </script>
                </body>
            </html>`;

        }

    }

    private getResultsHtml(toolkitUri: vscode.Uri, results: SimpleQueryRowsResponse) {

        // const rows = JSON.stringify(this.bqResultResolver(results));

        // const g = (new Grid()).render();

        // return `<!DOCTYPE html>
		// <html lang="en">
		// 	<head>
		// 		<meta charset="UTF-8">
		// 		<meta name="viewport" content="width=device-width, initial-scale=1.0">
		// 		<script type="module" src="${toolkitUri}"></script>
		// 	</head>
		// 	<body>
        //         <vscode-data-grid id="basic-grid" generate-header="sticky" aria-label="Default"></vscode-data-grid>

	 	// 		<script>
	 	// 			document.getElementById('basic-grid').rowsData = ${rows};
	 	// 		</script>
		// 	</body>
		// </html>`;

        return `<!DOCTYPE html>
        <html lang="en">
        	<head>
        		<meta charset="UTF-8">
        		<meta name="viewport" content="width=device-width, initial-scale=1.0">
        		<script type="module" src="${toolkitUri}"></script>
        	</head>
        	<body>
                ${(new Grid(results))}
        	</body>
        </html>`;

    }

    /**
     * Modify the complex types into strings
     * 
     * @queryRowsResponse made into type `any` because of wrong implementation in the SimpleQueryRowsResponse. There, the values do not match the expected types
     */
    bqResultResolver(queryRowsResponse: any): any[] {

        const queryResults: bigquery.IGetQueryResultsResponse = queryRowsResponse[2];

        const schema: bigquery.ITableSchema | null = queryResults.schema || null;
        if (!schema) { return []; }

        const fields: bigquery.ITableFieldSchema[] = schema.fields || [];

        if (queryRowsResponse[0].length == 0) {

            const columns: any = {};
            for (let fieldIndex = 0; fieldIndex < fields.length; fieldIndex++) {
                const field: bigquery.ITableFieldSchema = fields[fieldIndex];
                const fieldName = field.name || '';
                columns[fieldName] = null;
            }

            return [columns];
        } else {

            const results: any[] = queryRowsResponse[0];

            for (let fieldIndex = 0; fieldIndex < fields.length; fieldIndex++) {
                const field: bigquery.ITableFieldSchema = fields[fieldIndex];

                const fieldName = field.name || '';

                switch (field.type || 'STRING') {
                    case 'DATE':
                        for (let rowIndex = 0; rowIndex < results.length; rowIndex++) {
                            const bigQueryDate = results[rowIndex][fieldName] as BigQueryDate;
                            results[rowIndex][fieldName] = bigQueryDate.value;
                        }
                        break;
                }
            }

            return results;
        }
    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

}