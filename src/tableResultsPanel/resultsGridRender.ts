import * as vscode from 'vscode';
import { getExtensionUri } from '../extension';
import { COMMAND_DOWNLOAD_CSV, COMMAND_DOWNLOAD_JSONL, COMMAND_SEND_PUBSUB } from '../extensionCommands';
import { ResultsGridRenderRequestV2 } from './resultsGridRenderRequestV2';

//https://github.com/microsoft/vscode-webview-ui-toolkit/blob/main/docs/getting-started.md

export class ResultsGridRender {

    private webViewPanel: vscode.WebviewPanel;
    private diagnosticsAttached: boolean = false;

    constructor(webViewPanel: vscode.WebviewPanel) {
        this.webViewPanel = webViewPanel;
        this.attachDiagnostics();
    }

    private attachDiagnostics(): void {
        if (this.diagnosticsAttached) {
            return;
        }
        this.diagnosticsAttached = true;

        this.webViewPanel.onDidChangeViewState(e => {
            console.log(`[vscode-bigquery] results panel viewState title="${e.webviewPanel.title}" visible=${e.webviewPanel.visible} active=${e.webviewPanel.active}`);
        });

        this.webViewPanel.onDidDispose(() => {
            console.log(`[vscode-bigquery] results panel disposed title="${this.webViewPanel.title}"`);
        });
    }

    public static executeCommand(c: any) {
        if ((c as any).command) {
            const command = (c as any).command;
            const data = {
                tableReference: (c as any).table_reference,
                jobReference: (c as any).job_reference,
                command: command,
            };

            switch (command) {
                case "download_csv": { vscode.commands.executeCommand(COMMAND_DOWNLOAD_CSV, data); }
                case "download_jsonl": { vscode.commands.executeCommand(COMMAND_DOWNLOAD_JSONL, data); }
                case "send_pubsub": { vscode.commands.executeCommand(COMMAND_SEND_PUBSUB, data); }
            }
        }
    }

    private buildHtml(gridJs: vscode.Uri, gridCss: vscode.Uri, chartGlobalJs: vscode.Uri): string {
        return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <link rel="stylesheet" href="${gridCss}">
    <script>const vscode = acquireVsCodeApi();</script>
    <style>
        * { box-sizing: border-box; }
        html, body { height: 100%; margin: 0; padding: 0; overflow: hidden; }
        body { display: flex; flex-direction: column; }
        .tab-bar {
            display: flex;
            background: var(--vscode-editorGroupHeader-tabsBackground, #1e1e1e);
            border-bottom: 1px solid var(--vscode-panel-border, #444);
            flex-shrink: 0;
        }
        .tab-btn {
            padding: 6px 16px;
            background: none;
            border: none;
            border-bottom: 2px solid transparent;
            color: var(--vscode-foreground, #ccc);
            cursor: pointer;
            font-size: 13px;
            font-family: var(--vscode-font-family, sans-serif);
        }
        .tab-btn:hover { background: var(--vscode-toolbar-hoverBackground, #2a2d2e); }
        .tab-btn.active {
            color: var(--vscode-tab-activeForeground, #fff);
            border-bottom-color: var(--vscode-focusBorder, #007acc);
        }
        .tab-content { display: none; flex: 1; min-height: 0; overflow-y: auto; }
        .tab-content.active { display: block; }
        #tab-viz { padding: 12px; }
        #tab-jobinfo { padding: 12px; }

        .viz-controls { display: flex; flex-wrap: wrap; gap: 8px; align-items: center; flex-shrink: 0; }
        .viz-controls label { font-size: 12px; color: var(--vscode-foreground, #ccc); }
        .viz-controls select {
            background: var(--vscode-dropdown-background, #3c3c3c);
            color: var(--vscode-dropdown-foreground, #ccc);
            border: 1px solid var(--vscode-dropdown-border, #555);
            padding: 3px 6px;
            font-size: 12px;
            border-radius: 2px;
        }
        .viz-controls button {
            background: var(--vscode-button-background, #0e639c);
            color: var(--vscode-button-foreground, #fff);
            border: none;
            padding: 4px 12px;
            font-size: 12px;
            border-radius: 2px;
            cursor: pointer;
        }
        .viz-controls button:hover { background: var(--vscode-button-hoverBackground, #1177bb); }
        .chart-wrap { position: relative; height: 400px; flex-shrink: 0; }
        #viz-msg { font-size: 13px; color: var(--vscode-descriptionForeground, #888); padding: 8px 0; flex-shrink: 0; }

        .jobinfo-table { width: 100%; border-collapse: collapse; font-size: 12px; }
        .jobinfo-table th, .jobinfo-table td {
            text-align: left;
            padding: 4px 8px;
            border-bottom: 1px solid var(--vscode-panel-border, #333);
            vertical-align: top;
            word-break: break-all;
        }
        .jobinfo-table th {
            color: var(--vscode-descriptionForeground, #888);
            width: 200px;
            white-space: nowrap;
        }
        .jobinfo-table td { color: var(--vscode-foreground, #ccc); }
        #jobinfo-placeholder { font-size: 13px; color: var(--vscode-descriptionForeground, #888); }
    </style>
</head>
<body>
    <div class="tab-bar">
        <button class="tab-btn active" data-tab="results">Results</button>
        <button class="tab-btn" data-tab="viz">Visualization</button>
        <button class="tab-btn" data-tab="jobinfo">Job Information</button>
    </div>
    <div id="tab-results" class="tab-content active">
        <div id="q1"></div>
    </div>
    <div id="tab-viz" class="tab-content">
        <div class="viz-controls">
            <label>Chart type:
                <select id="viz-type">
                    <option value="bar">Bar</option>
                    <option value="line">Line</option>
                    <option value="scatter">Scatter</option>
                    <option value="pie">Pie</option>
                    <option value="doughnut">Doughnut</option>
                </select>
            </label>
            <label>X axis:
                <select id="viz-x"><option value="">— select —</option></select>
            </label>
            <label>Y axis:
                <select id="viz-y"><option value="">— select —</option></select>
            </label>
            <button id="viz-render-btn">Render</button>
        </div>
        <div id="viz-msg">Run a query first, then open this tab to visualize results.</div>
        <div class="chart-wrap" style="display:none;">
            <canvas id="viz-chart"></canvas>
        </div>
    </div>
    <div id="tab-jobinfo" class="tab-content">
        <div id="jobinfo-placeholder">Run a query first to see job information.</div>
        <table class="jobinfo-table" id="jobinfo-table" style="display:none;"><tbody id="jobinfo-body"></tbody></table>
    </div>

    <script src="${chartGlobalJs}"></script>
    <script type="module" src="${gridJs}"></script>
    <script>
        // Tab switching
        document.querySelectorAll('.tab-btn').forEach(btn => {
            btn.addEventListener('click', () => {
                const tab = btn.dataset.tab;
                document.querySelectorAll('.tab-btn').forEach(b => b.classList.remove('active'));
                document.querySelectorAll('.tab-content').forEach(c => c.classList.remove('active'));
                btn.classList.add('active');
                document.getElementById('tab-' + tab).classList.add('active');
                if (tab === 'viz' && _jobState && !_cachedRows) { loadVizData(); }
            });
        });

        // State
        let _jobState = null;
        let _chart = null;
        let _cachedRows = null;
        let _loadingViz = false;

        // Handle messages from extension — grid.js re-dispatches as MessageEvent with .data
        window.addEventListener('external_message', e => {
            const msg = e.data;
            if (msg && msg.requestType === 'execute_query' && msg.job) {
                _jobState = { job: msg.job, projectId: msg.projectId, token: msg.token };
                _cachedRows = null;
                populateJobInfo(msg.job);
                document.getElementById('viz-msg').textContent = 'Switch to this tab or click Render to visualize.';
            }
        });

        function flattenObj(obj, prefix) {
            const rows = [];
            for (const key of Object.keys(obj || {})) {
                const val = obj[key];
                const label = prefix ? prefix + '.' + key : key;
                if (val !== null && typeof val === 'object' && !Array.isArray(val)) {
                    rows.push(...flattenObj(val, label));
                } else {
                    rows.push([label, Array.isArray(val) ? JSON.stringify(val) : String(val ?? '')]);
                }
            }
            return rows;
        }

        function populateJobInfo(job) {
            const tbody = document.getElementById('jobinfo-body');
            const placeholder = document.getElementById('jobinfo-placeholder');
            const table = document.getElementById('jobinfo-table');
            tbody.innerHTML = '';
            const rows = flattenObj(job, '');
            rows.forEach(([k, v]) => {
                const tr = document.createElement('tr');
                tr.innerHTML = '<th>' + escHtml(k) + '</th><td>' + escHtml(v) + '</td>';
                tbody.appendChild(tr);
            });
            placeholder.style.display = 'none';
            table.style.display = '';
        }

        function populateColumnSelectors(fields) {
            const xSel = document.getElementById('viz-x');
            const ySel = document.getElementById('viz-y');
            xSel.innerHTML = '<option value="">— select —</option>';
            ySel.innerHTML = '<option value="">— select —</option>';
            fields.forEach(name => {
                xSel.innerHTML += '<option value="' + escHtml(name) + '">' + escHtml(name) + '</option>';
                ySel.innerHTML += '<option value="' + escHtml(name) + '">' + escHtml(name) + '</option>';
            });
        }

        async function loadVizData() {
            if (_loadingViz || !_jobState) { return; }
            _loadingViz = true;
            const vmsg = document.getElementById('viz-msg');
            vmsg.textContent = 'Loading data…';
            try {
                const { rows, fields } = await fetchBqData(_jobState.job, _jobState.projectId, _jobState.token);
                _cachedRows = rows;
                populateColumnSelectors(fields);
                vmsg.textContent = fields.length > 0 ? 'Select columns and click Render.' : 'No columns available.';
            } catch (err) {
                vmsg.textContent = 'Error loading data: ' + err.message;
            } finally {
                _loadingViz = false;
            }
        }

        document.getElementById('viz-render-btn').addEventListener('click', async () => {
            if (!_jobState) {
                document.getElementById('viz-msg').textContent = 'No query results available yet.';
                return;
            }
            const vmsg = document.getElementById('viz-msg');
            if (!_cachedRows) {
                await loadVizData();
                if (!_cachedRows) { return; }
            }
            const xCol = document.getElementById('viz-x').value;
            const yCol = document.getElementById('viz-y').value;
            if (!xCol || !yCol) {
                vmsg.textContent = 'Please select X and Y columns.';
                return;
            }
            renderChart(_cachedRows, xCol, yCol);
            vmsg.textContent = '';
        });

        async function fetchBqData(job, projectId, token) {
            const jobId = job?.jobReference?.jobId || job?.id;
            const location = job?.jobReference?.location || job?.location;
            if (!jobId || !projectId || !token) { throw new Error('Missing job/project/token'); }
            let url = 'https://bigquery.googleapis.com/bigquery/v2/projects/' + encodeURIComponent(projectId) + '/queries/' + encodeURIComponent(jobId) + '?maxResults=10000';
            if (location) { url += '&location=' + encodeURIComponent(location); }
            const resp = await fetch(url, { headers: { 'Authorization': 'Bearer ' + token } });
            if (!resp.ok) { throw new Error('HTTP ' + resp.status); }
            const body = await resp.json();
            const fields = ((body.schema && body.schema.fields) || []).map(f => f.name);
            const rawRows = body.rows || [];
            const rows = rawRows.map(row => {
                const obj = {};
                (row.f || []).forEach((cell, i) => { obj[fields[i] || i] = cell.v; });
                return obj;
            });
            return { rows, fields };
        }

        function renderChart(rows, xCol, yCol) {
            const chartType = document.getElementById('viz-type').value;
            const labels = rows.map(r => r[xCol]);
            const values = rows.map(r => parseFloat(r[yCol]));
            const wrap = document.querySelector('.chart-wrap');
            wrap.style.display = '';

            if (_chart) { _chart.destroy(); _chart = null; }
            const canvas = document.getElementById('viz-chart');
            _chart = new window.Chart(canvas, {
                type: chartType,
                data: {
                    labels: labels,
                    datasets: [{
                        label: yCol,
                        data: chartType === 'scatter' ? rows.map(r => ({ x: parseFloat(r[xCol]), y: parseFloat(r[yCol]) })) : values,
                        backgroundColor: 'rgba(0, 122, 204, 0.6)',
                        borderColor: 'rgba(0, 122, 204, 1)',
                        borderWidth: 1
                    }]
                },
                options: {
                    responsive: true,
                    maintainAspectRatio: false,
                    plugins: { legend: { display: true } }
                }
            });
        }

        function escHtml(s) {
            return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;').replace(/"/g,'&quot;');
        }

        vscode.postMessage({command:'load_complete'});
    </script>
</body>
</html>`;
    }

    public render1(): Promise<boolean> {
        const extensionUri = getExtensionUri();
        const gridJs = this.getUri(this.webViewPanel.webview, extensionUri, ['resources', 'grid.js']);
        const gridCss = this.getUri(this.webViewPanel.webview, extensionUri, ['resources', 'grid.css']);
        const chartGlobalJs = this.getUri(this.webViewPanel.webview, extensionUri, ['dist', 'chartGlobal.js']);

        return new Promise((resolve, reject) => {
            const timer = setTimeout(() => { reject(null); }, 10 * 1000);

            this.webViewPanel.webview.onDidReceiveMessage(c => {
                if ((c as any).command === 'load_complete') {
                    console.log(`[vscode-bigquery] webview load_complete received title="${this.webViewPanel.title}"`);
                    clearTimeout(timer);
                    resolve(true);
                } else {
                    ResultsGridRender.executeCommand(c);
                }
            });

            this.webViewPanel.webview.html = this.buildHtml(gridJs, gridCss, chartGlobalJs);
        });
    }

    public render2() {
        const extensionUri = getExtensionUri();
        const gridJs = this.getUri(this.webViewPanel.webview, extensionUri, ['resources', 'grid.js']);
        const gridCss = this.getUri(this.webViewPanel.webview, extensionUri, ['resources', 'grid.css']);
        const chartGlobalJs = this.getUri(this.webViewPanel.webview, extensionUri, ['dist', 'chartGlobal.js']);

        this.webViewPanel.webview.onDidReceiveMessage(c => {
            if ((c as any).command !== 'load_complete') {
                ResultsGridRender.executeCommand(c);
            } else {
                console.log(`[vscode-bigquery] webview load_complete received (render2) title="${this.webViewPanel.title}"`);
            }
        });

        this.webViewPanel.webview.html = this.buildHtml(gridJs, gridCss, chartGlobalJs);
    }

    public postMessage(message: ResultsGridRenderRequestV2): Thenable<boolean> {
        return this.webViewPanel.webview.postMessage(message);
    }

    private getUri(webview: vscode.Webview, extensionUri: vscode.Uri, pathList: string[]) {
        return webview.asWebviewUri(vscode.Uri.joinPath(extensionUri, ...pathList));
    }

    reveal(viewColumn?: vscode.ViewColumn, preserveFocus?: boolean): void {
        this.webViewPanel.reveal(viewColumn, preserveFocus);
    }

}
