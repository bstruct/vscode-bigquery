import * as vscode from 'vscode';
import { getExtensionUri } from '../extension';
import { SimpleQueryRowsResponseError } from '../services/simpleQueryRowsResponseError';
import { TableMetadata } from '../services/tableMetadata';
// import { SchemaGrid } from './schemaGrid.ts';

//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class SchemaRender {

    private webView: vscode.Webview;

    constructor(webView: vscode.Webview) {
        this.webView = webView;
    }

    public render(metadataPromise: Promise<any>) {

        try {

            //set waiting gif
            this.webView.html = this.getWaitingHtml();

            metadataPromise
                .then(async (metadata: TableMetadata) => {

                    const html = await this.getResultsHtml(metadata);
                    this.webView.html = html;

                })
                .catch(exception => {
                    this.webView.html = this.getExceptionHtml(exception);
                });

        } catch (error: any) {
            this.webView.html = this.getExceptionHtml(error.message);
            // vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
        }
    }

    private getWaitingHtml(): string {

        const toolkitUri = this.getUri(this.webView, getExtensionUri(), [
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

        const extensionUri = getExtensionUri();

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

    private async getResultsHtml(tableMetadata: TableMetadata): Promise<string> {

        // const schema = JSON.stringify(tableMetadata.schema.fields);
        const extensionUri = getExtensionUri();

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
                <script type="module" src="https://cdn.jsdelivr.net/npm/@finos/perspective/dist/cdn/perspective.js"></script>
                <script type="module" src="https://cdn.jsdelivr.net/npm/@finos/perspective-viewer/dist/cdn/perspective-viewer.js"></script>
                <script type="module" src="https://cdn.jsdelivr.net/npm/@finos/perspective-viewer-datagrid/dist/cdn/perspective-viewer-datagrid.js"></script>
                <script type="module" src="https://cdn.jsdelivr.net/npm/@finos/perspective-viewer-d3fc/dist/cdn/perspective-viewer-d3fc.js"></script>

                <link rel="stylesheet" crossorigin="anonymous" href="https://cdn.jsdelivr.net/npm/@finos/perspective-viewer/dist/css/pro.css"/>

                <script type="module" src="${toolkitUri}"></script>
                <link href="${codiconsUri}" rel="stylesheet" />
                <link href="${gridCss}" rel="stylesheet" />
        	</head>
        	<body>

                <perspective-viewer></perspective-viewer>
                <script type="module">
                    import perspective from "https://cdn.jsdelivr.net/npm/@finos/perspective/dist/cdn/perspective.js";
                
                    const worker = perspective.worker();
                    const table = agent.table({ x: [1, 2, 3, 4, 5] });
                    document.querySelector("perspective-viewer").load(table);
                </script>

                <div class="labelValue"><span class="label">Project Id</span><span class="value">${tableMetadata.tableReference.projectId}</span></div>
                <div class="labelValue"><span class="label">Dataset Id</span><span class="value">${tableMetadata.tableReference.datasetId}</span></div>
                <div class="labelValue"><span class="label">Table Id</span><span class="value">${tableMetadata.tableReference.tableId}</span></div>
                
                <div class="labelValue"><span class="label">Location</span><span class="value">${tableMetadata.location}</span></div>
                <div class="labelValue"><span class="label">Number of rows</span><span class="value">${tableMetadata.numRows}</span></div>

                <div class="labelValue"><span class="label">Creation time</span><span class="value">${new Date(Number(tableMetadata.creationTime))}</span></div>
                <div class="labelValue"><span class="label">Last modified time</span><span class="value">${new Date(Number(tableMetadata.lastModifiedTime))}</span></div>

                <div class="spacer"></div> 
        	</body>
        </html>`;

    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

}