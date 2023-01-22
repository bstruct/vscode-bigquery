import * as vscode from 'vscode';

export class ChartRender {
    private webViewPanel: vscode.WebviewPanel;

    constructor(webViewPanel: vscode.WebviewPanel) {
        this.webViewPanel = webViewPanel;
        const listener = this.webViewPanel.webview.onDidReceiveMessage(this.listenerResultsOnDidReceiveMessage, this);
        webViewPanel.onDidDispose(c => { listener.dispose(); });
    }

    public async render() {

        try {

            const html = await this.getChartHtml();
            this.webViewPanel.webview.html = html;

        } catch (error: any) {
            // this.webViewPanel.webview.html = this.getExceptionHtml(error.message);
            // vscode.window.showErrorMessage(`Unexpected error!\n${error.message}`);
        }
    }

    getChartHtml() {
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
                    const count = 1000;
                    const ids = new Uint32Array(count);
                    const binIdsX = new Float64Array(count);
                    const binIdsZ = new Float64Array(count);
                    const values = new Float64Array(count);
                    const binsX = 5;
                    const binsZ = 1;
                    const sizeX = 5;
                    const sizeZ = 5;
                    for (let i = 0; i < count; i++) {
                        ids[i] = i;
                        binIdsX[i] = Math.floor(binsX * Math.pow(Math.random(), 2));
                        binIdsZ[i] = Math.floor(binsZ * Math.pow(Math.random(), 2));
                        values[i] = Math.pow(Math.random(), 2);
                    }
        
                    // Palette
                    const palette = MorphCharts.Helpers.PaletteHelper.resample(core.paletteResources.palettes[MorphCharts.PaletteName.blues].colors, 32, false);
        
                    // Transition buffer
                    const transitionBuffer = core.renderer.createTransitionBuffer(ids);
                    core.renderer.transitionBuffers = [transitionBuffer];
                    transitionBuffer.currentBuffer.unitType = MorphCharts.UnitType.block;
                    transitionBuffer.currentPalette.colors = palette;
        
                    // Order by value
                    const orderedIds = new Uint32Array(ids);
                    // orderedIds.sort(function (a, b) { return values[a] - values[b]; });
        
                    // Layout
                    const stack = new MorphCharts.Layouts.Stack(core);
                    stack.layout(transitionBuffer.currentBuffer, orderedIds, {
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
                        maxValueY: stack.maxLevel * sizeX * sizeZ,
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

}