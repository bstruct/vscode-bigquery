import { QueryResultsOptions } from '@google-cloud/bigquery/build/src/job';
import * as vscode from 'vscode';
import { getExtensionUri } from '../extension';
import { getBigQueryClient } from '../extensionCommands';
import { SimpleQueryRowsResponseError } from '../services/simpleQueryRowsResponseError';
import { ResultsChartRenderRequest } from './ResultsChartRenderRequest';

export class ResultsChartRender {
    private webViewPanel: vscode.WebviewPanel;

    constructor(webViewPanel: vscode.WebviewPanel) {
        this.webViewPanel = webViewPanel;
        const listener = this.webViewPanel.webview.onDidReceiveMessage(this.listenerResultsOnDidReceiveMessage, this);
        webViewPanel.onDidDispose(c => { listener.dispose(); });
    }

    public async render(request: ResultsChartRenderRequest) {

        try {

            //set waiting gif
            this.webViewPanel.webview.html = this.getWaitingHtml(request.jobIndex);

            const [html, totalRows] = await this.getChartHtml(request);
            this.webViewPanel.webview.html = html;

        } catch (error: any) {
            this.webViewPanel.webview.html = this.getExceptionHtml(error.message);
        }
    }

    public renderException(error: any) {
        this.webViewPanel.webview.html = this.getExceptionHtml(error);
    }

    public renderLoadingIcon() {
        this.webViewPanel.webview.html = this.getWaitingHtml(undefined);
    }

    private getWaitingHtml(jobIndex: number | undefined): string {

        const extensionUri = getExtensionUri();

        const toolkitUri = this.getUri(this.webViewPanel.webview, extensionUri, [
            "resources",
            "toolkit.min.js",
        ]);

        return `<!DOCTYPE html>
		<html lang="en">
			<head>
				<meta charset="UTF-8">
				<meta name="viewport" content="width=device-width, initial-scale=1.0">
				<script type="module" src="${toolkitUri}"></script>
                <script>
                    const qElement = document.querySelectorAll('div.editor-actions ul.actions-container > li.action-item a[aria-label="\${x1}"]');
                    if(qElement.length >0){
                        const element = qElement[0];
                        element.innerText = 'trying';
                    }
                
                    const vscode = acquireVsCodeApi();
                    vscode.setState({ jobIndex: ${jobIndex} });
                </script>
			</head>
			<body>
                <vscode-progress-ring></vscode-progress-ring>
			</body>
		</html>`;

    }

    async getChartHtml(request: ResultsChartRenderRequest) :  Promise<[string, number]> {

        // const job = bigqueryClient.getJob(jobReference);
        // let queryResults = await job.getQueryResults({ autoPaginate: true, maxResults: 10000 });

        // const schema = JSON.stringify(queryResults[2]?.schema);
        // const data = JSON.stringify(queryResults[0]);
        let schema = '';
        let data = '';
        let totalRows = 0;

        // let totalRows: number = 0;
        // let rows: any[] = [];
        // let schema: bigquery.ITableSchema = {};

        if (request.jobReferences && request.jobReferences.length > 0) {
            const jobsReference = request.jobReferences[request.jobIndex];
            const bqClient = await getBigQueryClient();
            const job = bqClient.getJob(jobsReference);
            const queryResultOptions: QueryResultsOptions = {startIndex: '0', maxResults: 1_000_000};
            const queryRowsResponse = (await job.getQueryResults(queryResultOptions));
            data = JSON.stringify(queryRowsResponse[0]);
            schema = JSON.stringify(queryRowsResponse[2]?.schema || {});
            totalRows = Number(queryRowsResponse[2]?.totalRows || 0);

            // } else {
            //     if (request.tableReference) {

            //         const tableReference = request.tableReference;
            //         const table = getBigQueryClient().getTable(tableReference.projectId, tableReference.datasetId, tableReference.tableId);
            //         const metadata = await table.getMetadata();
            //         schema = metadata[0].schema;
            //         totalRows = Number(metadata[0].numRows || 0);
            //         rows = (await table.getRows({ startIndex: request.startIndex.toString(), maxResults: request.maxResults }))[0];

        } else {
            throw new Error('Unexpected error: "No job results nor table was found"');
        }
        // }

        const toolkitUri = this.getUri(this.webViewPanel.webview, getExtensionUri(), [
            "resources",
            "toolkit.min.js",
        ]);

        return [`<!DOCTYPE html>
        <html lang="en-us">
        
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Transition</title>
        	<script type="module" src="${toolkitUri}"></script>
            <style type="text/css">
                html,
                body {
                    height: 100%;
                    margin: 0;
                }
        
                .range {
                    margin: 8px;
                }
            </style>
            <script>
                const vscode = acquireVsCodeApi();
                vscode.setState({ jobIndex: ${request.jobIndex} });
            </script>
            <script id="chart-data-schema" type="application/json">${schema}</script>
            <script id="chart-data" type="application/json">${data}</script>
        
            <script type="module">
                import * as MorphCharts from "https://cdn.skypack.dev/morphcharts";
                window.onload = () => {
        
                    //fields with a known type
                    const schema = JSON.parse(document.getElementById('chart-data-schema').textContent);
                    const fields = schema.fields
                        .filter(c => c.type === 'STRING');
        
                    //populate
                    const selectors = ['selectXAxis', 'selectZAxis', 'selectColor'];//, 'selectFacet'
        
                    selectors.forEach(elementName => {
                        const selectElement = document.getElementById(elementName);
        
                        fields.forEach(i => {
                            var option = document.createElement('option');
                            option.text = i.name;
                            selectElement.add(option);
                        });
        
                        selectElement.onchange = function (event) {
        
                            const columnNames = [];
        
                            selectors.forEach(elementName => {
                                const selectElement = document.getElementById(elementName);
                                if (selectElement.value === '-') {
                                    columnNames.push(null);
                                } else {
                                    columnNames.push(selectElement.value);
                                }
                            });
        
                            const columnX = columnNames[0] ? getColumn(columnNames[0]) : null;
                            const columnZ = columnNames[1] ? getColumn(columnNames[1]) : null;
                            const columnColor = columnNames[2] ? getColumn(columnNames[2]) : getFakeColumn();
                            // const columnFacet = columnNames[3] ? getColumn(columnNames[3]) : null;
        
                            plotChart(columnX, columnZ, columnColor, null);
        
                        };
        
                    });
        
                    //data
                    const data = JSON.parse(document.getElementById('chart-data').textContent);
                    const ids = [...Array(data.length).keys()];
                    const count = data.length;
        
                    //categories - as string in this example
                    const getColumn = (categoryName) => {
                        const categories_vector = data.map(i => i[categoryName]);
                        const categories_distinct =
                            categories_vector
                                .filter((value, currentIndex, array) => {
                                    return array.indexOf(value) == currentIndex;
                                })
                                .sort((a, b) => (a || '').localeCompare(b));
        
                        const categories_count = categories_distinct.length;
                        const categories_vector_index = data.map(i => categories_distinct.indexOf(i[categoryName]));
        
                        return {
                            categories_distinct,
                            categories_count,
                            categories_vector_index,
                        };
                    };
        
                    const getFakeColumn = () => {
                        return {
                            categories_distinct: [''],
                            categories_count: 1,
                            categories_vector_index: data.map(i => 0),
                        };
                    };
        
                    // Container
                    const container = document.getElementById("container");
        
                    // Core
                    const core = new MorphCharts.Core({
                        container: container,
                        fontRasterizerOptions: {
                            fontAtlas: new MorphCharts.FontAtlas(2048, 2048),
                            fontSize: 192,
                            border: 24,
                            fontFamily: "'segoe ui semibold', sans-serif",
                            fontWeight: "normal",
                            fontStyle: "normal",
                            baseline: "alphabetic",
                            maxDistance: 64,
                            edgeValue: 0xc0,
                        },
                        useInputManager: true
                    });
        
                    const itemSelected = document.getElementById('itemSelected');
                    core.inputManager.pickItemCallback = function (item) {
                        document.getElementById('itemSelectedParent').style.display = '';
                        const dataItem = data[item.id];
                        itemSelected.innerText = JSON.stringify(dataItem, undefined, 2);
                    };
        
        
                    // Renderer
                    // core.renderer = new MorphCharts.Renderers.Advanced.Main();
                    // core.renderer.config.isFxaaEnabled = true;
                    // core.renderer.config.shadowWidth = core.renderer.config.shadowHeight = 4096;
                    // core.renderer.labelSets[0].isVisible = core.renderer.labelSets[1].isVisible = false;
                    core.renderer = new MorphCharts.Renderers.Basic.Main();
                    // core.renderer.transitionTime = 0;
                    // core.renderer.axesVisibility = MorphCharts.AxesVisibility.previous;
        
                    // Alt-azimuth camera
                    const camera = core.camera;
                    camera.setPosition([0, 0, 0.15], false);
                    camera.setAltAzimuth(MorphCharts.Helpers.AngleHelper.degreesToRadians(0), 0, false);
                    let id3dAngleCamera = false;
        
                    // Transition buffer
                    const transitionBuffer = core.renderer.createTransitionBuffer(ids);
                    core.renderer.transitionBuffers = [transitionBuffer];
                    transitionBuffer.currentBuffer.unitType = MorphCharts.UnitType.block;
        
                    const plotChart = (columnX, columnZ, columnColor, columnFacet) => {
        
                        if (columnColor === null) {
                            columnColor = getFakeColumn();
                        }
        
                        // Palette
                        const palette = MorphCharts.Helpers.PaletteHelper.resample(core.paletteResources.palettes[MorphCharts.PaletteName.blues].colors, columnColor.categories_count, true);
                        transitionBuffer.currentPalette.colors = palette;
        
                        //1D vs 2D vs 3D
                        if (columnX === null) {
        
                            // Layout
                            const sheet = new MorphCharts.Layouts.Sheet(core);
                            sheet.layout(transitionBuffer.currentBuffer, ids, { side: Math.ceil(Math.sqrt(ids.length)) });
                            sheet.update(transitionBuffer.currentBuffer, ids, { thickness: 0.01, padding: 0.1 });
                            core.renderer.currentAxes = [];
        
                        } else {
        
                            //2D vs 3D
                            if (columnZ) {
        
                                const sizeX = 5;
                                const sizeZ = 5;
        
                                // Layout
                                const stack = new MorphCharts.Layouts.Stack(core);
                                stack.layout(transitionBuffer.currentBuffer, ids, {
                                    binsX: columnX.categories_count,
                                    binsZ: columnZ.categories_count,
                                    binIdsX: columnX.categories_vector_index,
                                    binIdsZ: columnZ.categories_vector_index,
                                    sizeX: sizeX,
                                    sizeZ: sizeZ,
                                    spacingX: 1,
                                    spacingZ: 1,
                                });
                                stack.update(transitionBuffer.currentBuffer, ids, {
                                    maxColor: columnColor.categories_count - 1,
                                    colors: columnColor.categories_vector_index,
                                    padding: 0.025,
                                });
        
                                // Axes
                                const axes = MorphCharts.Axes.Cartesian3dAxesHelper.create(core, {
                                    minBoundsX: stack.minModelBoundsX,
                                    minBoundsY: stack.minModelBoundsY,
                                    minBoundsZ: stack.minModelBoundsZ,
                                    maxBoundsX: stack.maxModelBoundsX,
                                    maxBoundsY: stack.maxModelBoundsY,
                                    maxBoundsZ: stack.maxModelBoundsZ,
                                    minValueX: 0,
                                    maxValueX: columnX.categories_count - 1,
                                    minValueY: 0,
                                    maxValueY: stack.maxLevel * sizeX * sizeZ,
                                    minValueZ: 0,
                                    maxValueZ: columnZ.categories_count - 1,
                                    // titleX: "x",
                                    // titleY: "y",
                                    // titleZ: "z",
                                    isDiscreteX: true,
                                    isDiscreteZ: true,
                                    labelsX: (value) => { return columnX.categories_distinct[value] || '-'; },
                                    labelsY: (value) => { return Math.round(value).toString(); },
                                    labelsZ: (value) => { return columnZ.categories_distinct[value] || '-'; }
                                });
        
                                // Hide zero lines
                                axes.zero[0] = -1;
                                axes.zero[2] = -1;
                                core.renderer.currentAxes = [core.renderer.createCartesian3dAxesVisual(axes)];
        
                                if (!id3dAngleCamera) {
                                    camera.setAltAzimuth(MorphCharts.Helpers.AngleHelper.degreesToRadians(15), MorphCharts.Helpers.AngleHelper.degreesToRadians(-45), false);
                                    id3dAngleCamera = true;
                                }
        
        
                            } else {
        
                                // Layout
                                const sizeX = Math.ceil(Math.sqrt(count) / columnX.categories_count);
                                const stack = new MorphCharts.Layouts.Stack(core);
                                stack.layout(transitionBuffer.currentBuffer, ids, {
                                    binsX: columnX.categories_count,
                                    binIdsX: columnX.categories_vector_index, // #bins = #categories
                                    spacingX: 1,
                                    sizeX: sizeX,
                                });
                                stack.update(transitionBuffer.currentBuffer, ids, {
                                    maxColor: columnColor.categories_count - 1,
                                    colors: columnColor.categories_vector_index,
                                    padding: 0.1,
                                    thickness: 0.01,
                                });
        
                                // Axes
                                const axes = MorphCharts.Axes.Cartesian2dAxesHelper.create(core, {
                                    minBoundsX: stack.minModelBoundsX,
                                    minBoundsY: stack.minModelBoundsY,
                                    maxBoundsX: stack.maxModelBoundsX,
                                    maxBoundsY: stack.maxModelBoundsY,
                                    arePickDivisionsVisibleX: false,
                                    arePickDivisionsVisibleY: false,
                                    minValueX: 0,
                                    maxValueX: columnX.categories_count - 1,
                                    minValueY: 0,
                                    maxValueY: stack.maxLevel * sizeX,
                                    isDiscreteX: true,
                                    labelsX: (value) => { return columnX.categories_distinct[value] || '-'; },
                                    labelsY: (value) => { return Math.round(value).toString(); },
                                });
                                axes.zero[0] = axes.zero[2] = -1; // Hide zero lines
                                axes.minorGridlines[1] = 1;
                                axes.isEdgeVisible[MorphCharts.Edge2D.right] = false;
                                axes.isEdgeVisible[MorphCharts.Edge2D.top] = false;
                                core.renderer.currentAxes = [core.renderer.createCartesian2dAxesVisual(axes)];
        
                                // camera.setAltAzimuth(MorphCharts.Helpers.AngleHelper.degreesToRadians(0), 0, false);
        
                            }
                        }
                    };
        
                    plotChart(null, null, null, null);
        
                };
            </script>
        </head>
        
        <body>
            <div style="margin:8px;position:absolute;left:0;top:0;">
                <div style="padding-bottom: 5px;">
                    <label style="width: 40px;display: inline-block;" for="selectXAxis">X</label>
                    <select id="selectXAxis">
                        <option value="-">-</option>
                    </select>
                </div>
                <div style="padding-bottom: 5px;">
                    <label style="width: 40px;display: inline-block;" for="selectZAxis">Z</label>
                    <select id="selectZAxis">
                        <option value="-">-</option>
                    </select>
                </div>
                <div style="padding-bottom: 5px;">
                    <label style="width: 40px;display: inline-block;" for="selectColor">Color</label>
                    <select id="selectColor">
                        <option value="-">-</option>
                    </select>
                </div>
                <!-- <div style="padding-bottom: 5px;">
                    <label style="width: 40px;display: inline-block;" for="selectFacet">Facet</label>
                    <select id="selectFacet">
                        <option value="-">-</option>
                    </select>
                </div> -->
                <div id="itemSelectedParent" style="padding-bottom: 5px;display: none;">
                    <button onclick="document.getElementById('itemSelectedParent').style.display = 'none';">Close</button>
                    <pre id="itemSelected"></pre>
                </div>
            </div>
        
            <div id="container" class="container" style="width:100%;height:100%;"></div>
        </body>
        
        </html>`,
            totalRows
        ];

    }

    /* This function will run as an event triggered when the JS on the webview triggers
     * the `postMessage` method. For query results
    */
    private async listenerResultsOnDidReceiveMessage(message: any): Promise<void> {
    }

    private getExceptionHtml(exception: any): string {

        const toolkitUri = this.getUri(this.webViewPanel.webview, getExtensionUri(), [
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

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

    reveal(viewColumn?: vscode.ViewColumn, preserveFocus?: boolean): void {
        this.webViewPanel.reveal(viewColumn, preserveFocus);
    }

}