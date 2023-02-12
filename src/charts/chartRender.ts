import internal = require('stream');
import * as vscode from 'vscode';
import { extensionUri } from '../extension';
import { BigQueryClient } from '../services/bigqueryClient';
import { JobReference } from '../services/queryResultsMapping';
import { SimpleQueryRowsResponseError } from '../services/simpleQueryRowsResponseError';

export class ChartRender {
    private webViewPanel: vscode.WebviewPanel;

    constructor(webViewPanel: vscode.WebviewPanel) {
        this.webViewPanel = webViewPanel;
        const listener = this.webViewPanel.webview.onDidReceiveMessage(this.listenerResultsOnDidReceiveMessage, this);
        webViewPanel.onDidDispose(c => { listener.dispose(); });
    }

    public async render(bigqueryClient: BigQueryClient, jobReference: JobReference) {

        try {

            const job = bigqueryClient.getJob(jobReference);
            let queryResults = await job.getQueryResults({ autoPaginate: true, maxResults: 1000 });


            const uri: vscode.Uri | undefined = await vscode.window.showSaveDialog(
                {
                    title: 'Save export',
                    filters: {
                        'csv': ['csv']
                    },
                }
            );
            if (uri !== undefined) {

                var enc = new TextEncoder();
                await vscode.workspace.fs.writeFile(
                    uri,
                    enc.encode(JSON.stringify(queryResults[2]?.schema))
                );

            }

            const ids: number[] = [];//queryResults[0].map(c => this.hashCode(c.combi_number));
            for (let index = 0; index < queryResults[0].length; index++) {
                ids.push(index);
            }

            const binIdsX: any[] = queryResults[0].map(c => c.binIdsX);
            const binIdsZ: any[] = queryResults[0].map(c => c.binIdsZ);
            const values: any[] = queryResults[0].map(c => c.value);

            let binsZ: number = 0;
            new Set(binIdsZ).forEach(() => { binsZ++; });
            let labelsZ: any[] = [];
            new Set(binIdsZ).forEach((v) => labelsZ.push(v));


            //pass the file path to html page builder getChartHtml and set html
            const html = await this.getChartHtml(ids, binIdsX, binIdsZ, values, binsZ, labelsZ);
            this.webViewPanel.webview.html = html;

        } catch (error: any) {
            this.webViewPanel.webview.html = this.getExceptionHtml(error.message);
        }
    }

    getChartHtml(ids: any[], binIdsX: any[], binIdsZ: any[], values: any[]
        , binsZ: number
        , labelsZ: any[]
    ) {
        return `<!DOCTYPE html>
        <html lang="en-us">
        
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Stack Chart (Count)</title>
            <style type="text/css">
                html,
                body {
                    height: 100%;
                    margin: 0;
                }
            </style>
            <script type="module">
                import * as MorphCharts from "https://cdn.skypack.dev/morphcharts";
                window.onload = () => {
                    // Core
                    const core = new MorphCharts.Core();
        
                    // Renderer
                    core.renderer = new MorphCharts.Renderers.Basic.Main();
        
                    // Data
                    const ids = ${JSON.stringify(ids)};
                    const binIdsX = ${JSON.stringify(binIdsX)};
                    const binIdsZ = ${JSON.stringify(binIdsZ)};
                    const values = ${JSON.stringify(values)};

                    const binsX = binIdsX.reduce((a,b)=> (a > b)? a : b) + 1;
                    const binsZ = binIdsZ.reduce((a,b)=> (a > b)? a : b) + 1;
                    const sizeX = 5;
                    const sizeZ = 5;

                    // Palette
                    const palette = MorphCharts.Helpers.PaletteHelper.resample(core.paletteResources.palettes[MorphCharts.PaletteName.blues].colors, 32, false);
        
                    // Transition buffer
                    const transitionBuffer = core.renderer.createTransitionBuffer(ids);
                    core.renderer.transitionBuffers = [transitionBuffer];
                    transitionBuffer.currentBuffer.unitType = MorphCharts.UnitType.block;
                    transitionBuffer.currentPalette.colors = palette;
        
                    // Order by value
                    // const orderedIds = new Uint32Array(ids);
                    // orderedIds.sort(function (a, b) { return values[a] - values[b]; });
        
                    // Layout
                    const stack = new MorphCharts.Layouts.Stack(core);
                    stack.layout(transitionBuffer.currentBuffer, ids, {
                        binsX: binsX,
                        binsZ: binsZ,
                        binIdsX: binIdsX,
                        binIdsZ: binIdsZ,
                        sizeX: sizeX,
                        sizeZ: sizeZ,
                        spacingX: 1,
                        spacingZ: 1,
                    });
                    stack.update(transitionBuffer.currentBuffer, ids, {
                        colors: values,
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
                        maxValueX: binsX - 1,
                        minValueY: 0,
                        // maxValueY: stack.maxLevel * sizeX * sizeZ,
                        maxValueY: stack.maxTotal,
                        minValueZ: 0,
                        maxValueZ: binsZ - 1,
                        titleX: "x",
                        titleY: "y",
                        titleZ: "z",
                        isDiscreteX: true,
                        isDiscreteZ: true,
                        labelsX: (value) => { return value.toString(); },
                        labelsY: (value) => { return Math.round(value).toString(); },
                        labelsZ: (value) => { return value.toString(); }
                    });
                    // Hide zero lines
                    axes.zero[0] = -1;
                    axes.zero[2] = -1;
                    core.renderer.currentAxes = [core.renderer.createCartesian3dAxesVisual(axes)];
        
                    // Alt-azimuth camera
                    const camera = core.camera;
                    camera.setPosition([0, 0, -0.05], false);
                    camera.setAltAzimuth(MorphCharts.Helpers.AngleHelper.degreesToRadians(30), 0, false);
                };
            </script>
        </head>
        
        <body></body>
        
        </html>`;
    }

    /* This function will run as an event triggered when the JS on the webview triggers
     * the `postMessage` method. For query results
    */
    private async listenerResultsOnDidReceiveMessage(message: any): Promise<void> {
    }

    private getExceptionHtml(exception: any): string {

        const toolkitUri = this.getUri(this.webViewPanel.webview, extensionUri, [
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

}