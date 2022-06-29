import * as vscode from 'vscode';
import { extensionUri } from '../extension';
import { Table } from '@google-cloud/bigquery';
import { SimpleQueryRowsResponseError } from '../services/simple_query_rows_response_error';
import { MetadataResponse } from '@google-cloud/common';
import { BigQueryClient } from '../services/bigquery-client';

//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class SchemaRender {

    private webView: vscode.Webview;

    constructor(webView: vscode.Webview) {
        this.webView = webView;
    }

    public render(table: Table) {

        try {

            //set waiting gif
            this.webView.html = this.getWaitingHtml();

            //put this logic in BigQueryClient


            // 	SELECT
            //     field_path, collation_name, description
            //   FROM
            //     `damiao-project-1.PvhTest`.INFORMATION_SCHEMA.COLUMN_FIELD_PATHS
            //   WHERE
            //     table_name = 'PimExportLatest';

            const querySchemaPlus = BigQueryClient.runQuery('');


            // table.metadata({});

            table.getMetadata()
                .then(async (tableMetadata) => {

                    const html = await this.getResultsHtml(tableMetadata);
                    this.webView.html = html;

                })
                .catch(exception => {
                    this.webView.html = this.getExceptionHtml(exception);
                });

        } catch (error: any) {
            vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
        }
    }

    private getWaitingHtml(): string {

        const toolkitUri = this.getUri(this.webView, extensionUri, [
            "resources",
            "toolkit.min.js",
        ]);

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

    private getExceptionHtml(exception: any): string {

        const toolkitUri = this.getUri(this.webView, extensionUri, [
            "resources",
            "toolkit.min.js",
        ]);

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

    /* 
    * weird response because the total rows are only known in the `getQueryResults` response
    */
    private async getResultsHtml(tableMetadata: MetadataResponse): Promise<string> {

        const schema = JSON.stringify(tableMetadata[0].schema.fields);

        const toolkitUri = this.getUri(this.webView, extensionUri, [
            "resources",
            "toolkit.min.js",
        ]);

        const codiconsUri = this.getUri(this.webView, extensionUri, [
            'resources',
            'codicon.css']
        );

        const gridCss = this.getUri(this.webView, extensionUri, [
            'resources',
            'grid.css']
        );

        return `<!DOCTYPE html>
        <html lang="en" style="display:flex;">
        	<head>
        		<meta charset="UTF-8">
        		<script type="module" src="${toolkitUri}"></script>
                <link href="${codiconsUri}" rel="stylesheet" />
                <link href="${gridCss}" rel="stylesheet" />
        	</head>
        	<body>
                <vscode-data-grid id="basic-grid" generate-header="sticky" aria-label="Default"></vscode-data-grid>
                        
                <script>
                    document.getElementById('basic-grid').rowsData = ${schema};                           
                    const vscode = acquireVsCodeApi();
                </script>
        	</body>
        </html>`;

    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

}